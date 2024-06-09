//! Configuration storage

pub mod config_file;

use std::{borrow::Cow, sync::Arc};

use config_file::ConfigFile;
use directories_next::ProjectDirs;
use eyre::Result;

/// Configuration storage
#[derive(Debug)]
pub struct Config {
    global_config: ConfigFile<'static>,
}

impl Config {
    /// Creates a new configuration storage
    pub fn new(dirs: &ProjectDirs) -> Arc<Self> {
        Arc::new(Self {
            global_config: ConfigFile::const_new(dirs.config_dir().join("config.json").into()),
        })
    }

    /// Returns the default profile name
    ///
    /// This setting can only be changed globally
    pub async fn default_profile(&self) -> Result<Cow<'_, str>> {
        self.global_config
            .default_profile()
            .await
            .map(|o| o.unwrap_or_else(|| "default".into()))
    }

    /// Returns the chosen profile name
    ///
    /// This setting can be changed globally, or through an environment variable
    pub async fn chosen_profile(&self) -> Result<Cow<'_, str>> {
        if let Ok(profile) = std::env::var("RACHAT_PROFILE") {
            Ok(profile.into())
        } else {
            self.default_profile().await
        }
    }
}
