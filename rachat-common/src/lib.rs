//! Rachat data repository
//!
//! Performs all of the behind the scenes work for Rachat.

use config::Config;
use directories_next::ProjectDirs;
use eyre::{Context, OptionExt, Result};
use std::sync::Arc;
use tokio::fs;

pub mod config;
pub mod crypto;
pub mod data_store;
pub(crate) mod utils;

/// Root application state
#[derive(Debug)]
pub struct Rachat {
    /// Data store
    data_store: Arc<data_store::DataStore>,
    /// Global configuration
    config: Arc<Config>,
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
        let config = Config::new(&project_dirs);
        let profile = config.chosen_profile().await?;
        let data_store = data_store::DataStore::new(&project_dirs, &profile)
            .await
            .with_context(|| format!("Creating data store for profile {profile}",))?;
        Ok(Arc::new(Self { data_store, config }))
    }

    /// Returns a handle to the data store
    #[must_use]
    pub fn data_store(&self) -> Arc<data_store::DataStore> {
        Arc::clone(&self.data_store)
    }
}
