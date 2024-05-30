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
/// Errors related to the global configuration
pub enum ConfigError {
    #[error("IO Error")]
    #[diagnostic(code(rachat_common::config::io))]
    /// There has been an IO error trying to access the configuration
    IOError(#[from] std::io::Error),
    #[error("Dhall Error")]
    #[diagnostic(code(rachat_common::config::dhall))]
    /// The configuration file could not be serialized/deserialized
    DhallError(#[from] Box<serde_dhall::Error>),
}

impl From<serde_dhall::Error> for ConfigError {
    fn from(value: serde_dhall::Error) -> Self {
        Self::from(Box::new(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, StaticType)]
#[serde(default)]
/// Global configuration
pub struct Config {
    /// Default profile name
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
    /// Attempts to read the global configuration, and update it if necessary
    ///
    /// # Errors
    /// This function returns an error if the configuration file exists but can’t be accessed, deserialized, or updated
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
/// Errors that can occur in rachat
pub enum RachatError {
    #[error("Couldn’t find the project directories")]
    #[diagnostic(code(rachat_common::no_project_dirs))]
    /// This error is returned if the project directories can’t be found.
    NoProjectDirectories,
    #[error("IO Error")]
    #[diagnostic(code(rachat_common::io))]
    /// Generic IO error when preparing the project directories
    IOError(#[from] std::io::Error),
    #[error("Config Error")]
    #[diagnostic(code(rachat_common::config))]
    /// Configuration file is invalid or can’t be read
    ConfigError(#[from] ConfigError),
    #[error("Data Store Error")]
    #[diagnostic(code(rachat_common::data_store))]
    /// Data store can’t be opened
    DataStoreError(#[from] data_store::DataStoreError),
}

/// Root application state
#[derive(Debug)]
pub struct Rachat {
    /// Data store
    data_store: Arc<data_store::DataStore>,
    /// Global configuration
    config: Config,
}

impl Rachat {
    /// Attempts to create a new Rachat instance
    ///
    /// # Errors
    /// This function returns an error if the project directories can’t be found, the configuration file can’t be read, or the data store fails to open.
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
