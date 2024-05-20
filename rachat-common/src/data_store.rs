//! Backing datastore for the client
//!
//! Frontend code renders values from this module
use anyhow::Result;
use directories_next::ProjectDirs;
use matrix_sdk::config::StoreConfig;
use secrecy::ExposeSecret;
use std::{path::PathBuf, sync::Arc};
use tracing::instrument;

use crate::crypto::KDFSecretKey;

#[derive(Clone, Debug)]
/// Backing datastore for the client
pub struct DataStore {
    /// The root key for the key hierarchy.
    root_key: KDFSecretKey,
    /// Path to the data directory
    data_dir: PathBuf,
    /// Path to the cache directory
    cache_dir: PathBuf,
    /// matrix-rust-sdk store config
    store_config: StoreConfig,
}

impl DataStore {
    /// Creates a new data store
    #[instrument]
    pub async fn new(project_dirs: &ProjectDirs, profile: &str) -> Result<Arc<Self>> {
        let mut data_dir = project_dirs.data_dir().join(profile);
        let mut cache_dir = project_dirs.cache_dir().join(profile);

        if data_dir == cache_dir {
            data_dir = project_dirs.data_dir().join("data").join(profile);
            cache_dir = project_dirs.data_dir().join("cache").join(profile);
        }

        tokio::fs::create_dir_all(&data_dir).await?;
        tokio::fs::create_dir_all(&cache_dir).await?;

        let root_key = KDFSecretKey::load_from_keyring(profile).await?;

        let secret = root_key.subkey_passphrase("matrix-rust-sdk");

        let store_config = matrix_sdk_sqlite::make_store_config(
            &data_dir.join("sdk.db"),
            Some(secret.expose_secret().as_str()),
        )
        .await?;

        Ok(Arc::new(Self {
            root_key,
            data_dir,
            cache_dir,
            store_config,
        }))
    }
}
