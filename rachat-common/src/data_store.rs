//! Backing datastore for the client
//!
//! Frontend code renders values from this module
use anyhow::Result;
use directories_next::ProjectDirs;
use matrix_sdk::{Client, OwnedServerName};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use tracing::instrument;

use crate::crypto::KDFSecretKey;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Configuration for a single profile
pub struct ProfileConfig {
    /// The server name to connect to
    pub server_name: OwnedServerName,
}

#[derive(Clone, Debug)]
/// Backing datastore for the client
pub struct DataStore {
    /// The root key for the key hierarchy.
    root_key: KDFSecretKey,
    /// Path to the configuration directory
    config_dir: PathBuf,
    /// Configuration file
    config: Option<ProfileConfig>,
    /// Path to the data directory
    data_dir: PathBuf,
    /// Path to the cache directory
    cache_dir: PathBuf,
    /// Matrix client, may not exist at startup
    client: Option<Client>,
}

impl DataStore {
    /// Creates a new data store
    #[instrument]
    pub async fn new(project_dirs: &ProjectDirs, profile: &str) -> Result<Arc<Self>> {
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
            match tokio::fs::read_to_string(&config_dir.join("config.json")).await {
                Ok(v) => serde_json::from_str(&v).ok(),
                Err(_) => None,
            };

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
            config,
            data_dir,
            cache_dir,
            client,
        }))
    }
}
