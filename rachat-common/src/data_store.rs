use anyhow::Result;
use directories_next::ProjectDirs;
use keyring::Entry;
use matrix_sdk::config::StoreConfig;
use rand::{distributions::Alphanumeric, Rng};
use std::{path::PathBuf, sync::Arc};
use tracing::instrument;

#[derive(Clone, Debug)]
pub struct DataStore {
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

        let profile_owned = profile.to_owned();
        let secret = tokio::task::spawn_blocking(move || -> Result<String> {
            let entry = Entry::new("rs.chir.rachat", &format!("{profile_owned}-key"))?;
            match entry.get_password() {
                Ok(entry) => Ok(entry),
                Err(keyring::Error::NoEntry) => {
                    let secret: String = rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(32)
                        .map(char::from)
                        .collect();
                    entry.set_password(&secret)?;
                    Ok(secret)
                }
                Err(e) => Err(anyhow::anyhow!(e)),
            }
        })
        .await??;

        let store_config =
            matrix_sdk_sqlite::make_store_config(&data_dir.join("sdk.db"), Some(&secret)).await?;

        Ok(Arc::new(Self {
            data_dir,
            cache_dir,
            store_config,
        }))
    }
}
