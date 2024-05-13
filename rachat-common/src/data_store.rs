use anyhow::Result;
use directories_next::ProjectDirs;
use keyring::Entry;
use matrix_sdk::config::StoreConfig;
use rand::{distributions::Alphanumeric, Rng};
use std::{path::PathBuf, sync::Arc};
use tracing::instrument;

use crate::crypto::RootKey;

#[derive(Clone, Debug)]
pub struct DataStore {
    root_key: RootKey,
    data_dir: PathBuf,
    cache_dir: PathBuf,
    store_config: StoreConfig,
}

impl DataStore {
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

        let root_key = RootKey::load_from_keyring(profile).await?;

        let secret = root_key.subkey_passphrase("matrix-rust-sdk");

        let store_config =
            matrix_sdk_sqlite::make_store_config(&data_dir.join("sdk.db"), Some(&secret)).await?;

        Ok(Arc::new(Self {
            root_key,
            data_dir,
            cache_dir,
            store_config,
        }))
    }
}
