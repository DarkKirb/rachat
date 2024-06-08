//! Rachat data repository
//!
//! Performs all of the behind the scenes work for Rachat.

use directories_next::ProjectDirs;
use eyre::{Context, OptionExt, Result};
use serde::{Deserialize, Serialize};
use serde_dhall::StaticType;
use std::sync::Arc;
use tokio::fs;

pub mod crypto;
pub mod data_store;
pub(crate) mod utils;

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
    pub async fn read(project_dirs: &ProjectDirs) -> Result<Self> {
        let config_path = project_dirs.config_dir().join("config.dhall");
        let config: Self = if config_path.exists() {
            let config_str = fs::read_to_string(&config_path)
                .await
                .context("Reading global config file")?;
            serde_dhall::from_str(&config_str)
                .parse()
                .context("Parsing global config file")?
        } else {
            Self::default()
        };
        // Rewrite the configuration file
        fs::write(
            config_path,
            serde_dhall::serialize(&config)
                .static_type_annotation()
                .to_string()
                .context("Serializing global config")?,
        )
        .await
        .context("Writing global config file")?;
        Ok(config)
    }
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
    pub async fn new() -> Result<Arc<Self>> {
        let project_dirs = ProjectDirs::from("rs", "Raccoon Productions", "rachat")
            .ok_or_eyre("Missing project directories")?;
        fs::create_dir_all(project_dirs.config_dir())
            .await
            .context("Creating project directories")?;
        let config = Config::read(&project_dirs)
            .await
            .context("Reading global configuration")?;
        let data_store = data_store::DataStore::new(&project_dirs, &config.default_profile)
            .await
            .with_context(|| {
                format!("Creating data store for profile {}", config.default_profile)
            })?;
        Ok(Arc::new(Self { data_store, config }))
    }

    /// Returns a handle to the data store
    #[must_use]
    pub fn data_store(&self) -> Arc<data_store::DataStore> {
        Arc::clone(&self.data_store)
    }
}
