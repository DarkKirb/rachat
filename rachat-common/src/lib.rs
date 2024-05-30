//! Rachat data repository
//!
//! Performs all of the behind the scenes work for Rachat.

use directories_next::ProjectDirs;
use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use serde_dhall::StaticType;
use std::sync::Arc;
use thiserror::Error;
use tokio::fs;

pub mod crypto;
pub mod data_store;
pub(crate) mod utils;

#[derive(Error, Diagnostic, Debug)]
pub enum ConfigError {
    #[error("IO Error")]
    #[diagnostic(code(rachat_common::config::io))]
    IOError(#[from] std::io::Error),
    #[error("Dhall Error")]
    #[diagnostic(code(rachat_common::config::dhall))]
    DhallError(#[from] serde_dhall::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, StaticType)]
#[serde(default)]
pub struct Config {
    pub default_profile: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_profile: "default".to_owned(),
        }
    }
}

impl Config {
    pub async fn read(project_dirs: &ProjectDirs) -> Result<Self, ConfigError> {
        let config_path = project_dirs.config_dir().join("config.dhall");
        let config: Self = if config_path.exists() {
            let config_str = fs::read_to_string(&config_path).await?;
            serde_dhall::from_str(&config_str).parse()?
        } else {
            Self::default()
        };
        // Rewrite the configuration file
        fs::write(
            config_path,
            serde_dhall::serialize(&config)
                .static_type_annotation()
                .to_string()?,
        )
        .await?;
        Ok(config)
    }
}

#[derive(Error, Diagnostic, Debug)]
pub enum RachatError {
    #[error("Couldn’t find the project directories")]
    #[diagnostic(code(rachat_common::no_project_dirs))]
    NoProjectDirectories,
    #[error("IO Error")]
    #[diagnostic(code(rachat_common::io))]
    IOError(#[from] std::io::Error),
    #[error("Config Error")]
    #[diagnostic(code(rachat_common::config))]
    ConfigError(#[from] ConfigError),
    #[error("Data Store Error")]
    #[diagnostic(code(rachat_common::data_store))]
    DataStoreError(#[from] data_store::DataStoreError),
}

#[derive(Debug)]
pub struct Rachat {
    data_store: Arc<data_store::DataStore>,
    config: Config,
}

impl Rachat {
    pub async fn new() -> Result<Arc<Self>, RachatError> {
        let project_dirs = ProjectDirs::from("rs", "Raccoon Productions", "rachat")
            .ok_or(RachatError::NoProjectDirectories)?;
        fs::create_dir_all(project_dirs.config_dir()).await?;
        let config = Config::read(&project_dirs).await?;
        let data_store = data_store::DataStore::new(&project_dirs, &config.default_profile).await?;
        Ok(Arc::new(Self { data_store, config }))
    }

    /// Returns a handle to the data store
    #[must_use]
    pub fn data_store(&self) -> Arc<data_store::DataStore> {
        Arc::clone(&self.data_store)
    }
}
