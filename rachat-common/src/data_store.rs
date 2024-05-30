//! Backing datastore for the client
//!
//! Frontend code renders values from this module
use directories_next::ProjectDirs;
use educe::Educe;
use matrix_sdk::{matrix_auth::MatrixSession, Client, OwnedServerName, ServerName};
use miette::Diagnostic;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use std::{
    future::Future,
    path::{Path, PathBuf},
    sync::Arc,
};
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::instrument;

use crate::crypto::{
    mutable_file::{MutableFile, MutableFileError},
    KDFSecretKey, KDFSecretKeyError,
};

#[derive(Error, Diagnostic, Debug)]
pub enum DataStoreError {
    #[error("KDF Secret Key Error")]
    #[diagnostic(code(rachat_common::crypto::data_store::kdf_secret_key))]
    KDFSecretKeyError(#[from] KDFSecretKeyError),
    #[error("IO Error")]
    #[diagnostic(code(rachat_common::crypto::mutable_file::io_error))]
    IoError(#[from] tokio::io::Error),
    #[error("Client Build Error")]
    #[diagnostic(code(rachat_common::crypto::data_store::client_build_error))]
    MatrixSdk(#[from] matrix_sdk::ClientBuildError),
    #[error("ID parse error")]
    #[diagnostic(code(rachat_common::crypto::data_store::id_parse))]
    IdParse(#[from] matrix_sdk::IdParseError),
    #[error("MutableFileError")]
    #[diagnostic(code(rachat_common::crypto::data_store::mutable_file_error))]
    MutableFileError(#[from] MutableFileError),
    #[error("CBOR Seriallization error")]
    #[diagnostic(code(rachat_common::crypto::data_store::cbor_serialization))]
    CBORSerializationError(#[from] ciborium::de::Error<std::io::Error>),
    #[error("Matrix SDK Error")]
    #[diagnostic(code(rachat_common::crypto::data_store::matrix_sdk))]
    MatrixSdkError(#[from] matrix_sdk::Error),
    #[error("JSON Serialization error")]
    #[diagnostic(code(rachat_common::crypto::data_store::json_serialization))]
    JSONSerializationError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Configuration for a single profile
pub struct ProfileConfig {
    /// The server name to connect to
    pub server_name: OwnedServerName,
}

/// Backing datastore for the client
#[derive(Educe)]
#[educe(Debug)]
pub struct DataStore {
    /// The root key for the key hierarchy.
    root_key: KDFSecretKey,
    /// Path to the configuration directory
    config_dir: PathBuf,
    /// Configuration file
    config: RwLock<Option<ProfileConfig>>,
    /// Path to the data directory
    data_dir: PathBuf,
    /// Path to the cache directory
    cache_dir: PathBuf,
    /// Matrix client, may not exist at startup
    client: RwLock<Option<Arc<Client>>>,
}

impl DataStore {
    /// Creates a new data store
    #[instrument]
    pub async fn new(
        project_dirs: &ProjectDirs,
        profile: &str,
    ) -> Result<Arc<Self>, DataStoreError> {
        let config_dir = project_dirs.config_dir().join(profile);
        let mut data_dir = project_dirs.data_dir().join(profile);
        let mut cache_dir = project_dirs.cache_dir().join(profile);

        if data_dir == cache_dir {
            data_dir = project_dirs.data_dir().join("data").join(profile);
            cache_dir = project_dirs.cache_dir().join("cache").join(profile);
        }

        tokio::fs::create_dir_all(&data_dir).await?;
        tokio::fs::create_dir_all(&cache_dir).await?;
        tokio::fs::create_dir_all(&config_dir).await?;

        let config: Option<ProfileConfig> =
            (tokio::fs::read_to_string(&config_dir.join("config.json")).await)
                .map_or_else(|_| None, |v| serde_json::from_str(&v).ok());

        let root_key = KDFSecretKey::load_from_keyring(profile).await?;

        let secret = root_key.subkey_passphrase("matrix-rust-sdk");

        let client = if let Some(config) = &config {
            Some(
                Client::builder()
                    .server_name(config.server_name.as_ref())
                    .sqlite_store(
                        &data_dir.join("matrix.db"),
                        Some(secret.expose_secret().as_str()),
                    )
                    .user_agent("rachat")
                    .handle_refresh_tokens()
                    .build()
                    .await?,
            )
        } else {
            None
        };

        Ok(Arc::new(Self {
            root_key,
            config_dir,
            config: RwLock::new(config),
            data_dir,
            cache_dir,
            client: RwLock::new(client.map(Arc::new)),
        }))
    }

    /// Returns true if a client has been initialized for this profile
    pub async fn has_client(&self) -> bool {
        self.client.read().await.is_some()
    }

    /// Runs an async closure with the client
    ///
    /// # Errors
    /// This function will only return errors if the passed closure does.
    pub async fn with_client<F, Fut, Ret, E>(&self, fun: F) -> Result<Option<Ret>, E>
    where
        F: FnOnce(Arc<Client>) -> Fut + Send,
        Fut: Future<Output = Result<Ret, E>> + Send,
        E: Send,
    {
        if let Some(ref client) = *self.client.read().await {
            return Ok(Some(fun(Arc::clone(client)).await?));
        }
        Ok(None)
    }

    pub fn is_valid_homeserver_name(&self, server_name: &str) -> bool {
        ServerName::parse(server_name).is_ok()
    }

    /// Sets the homeserver for this profile
    pub async fn set_homeserver(&self, server_name: &str) -> Result<(), DataStoreError> {
        let server_name = ServerName::parse(server_name)?;
        let mut config = self.config.write().await;
        if let Some(config) = config.as_mut() {
            config.server_name = server_name.clone();
        } else {
            *config = Some(ProfileConfig {
                server_name: server_name.clone(),
            });
        }

        let secret = self.root_key.subkey_passphrase("matrix-rust-sdk");

        let client = Client::builder()
            .server_name(server_name.as_ref())
            .sqlite_store(
                &self.data_dir.join("matrix.db"),
                Some(secret.expose_secret().as_str()),
            )
            .user_agent("rachat")
            .handle_refresh_tokens()
            .build()
            .await?;

        // Restore the login session if it exists
        if let Some(session_data) = self
            .root_key
            .open_mutable_file(&self.data_dir, "auth/login")
            .read()
            .await?
        {
            let client_session: MatrixSession = ciborium::de::from_reader(session_data.as_slice())?;
            client.restore_session(client_session).await?;
        }

        *self.client.write().await = Some(Arc::new(client));

        tokio::fs::write(
            self.config_dir.join("config.json"),
            serde_json::to_string(&*config)?,
        )
        .await?;

        drop(config);

        Ok(())
    }

    /// Returns a handle to a mutable data file
    ///
    /// This data will be encrypted on disk
    pub fn open_mutable_file(&self, path: impl AsRef<Path>) -> MutableFile {
        self.root_key.open_mutable_file(&self.data_dir, path)
    }
}
