//! Configuration file

use std::{borrow::Cow, path::Path};

use eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use tokio::sync::{OnceCell, RwLock};
use tracing::error;

/// Data stored in the configuration file
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct ConfigFileData<'cfg> {
    /// Profile to use
    #[serde(skip_serializing_if = "Option::is_none")]
    default_profile: Option<Cow<'cfg, str>>,
}

impl<'cfg> ConfigFileData<'cfg> {
    async fn load(file_name: impl AsRef<Path>) -> Result<ConfigFileData<'cfg>> {
        match std::fs::read_to_string(file_name) {
            Ok(s) => Ok(serde_json::from_str(&s).context("Parsing configuration file")?),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    return Ok(Self::default());
                }
                error!("Failed to read configuration file: {e:#?}");
                Ok(Self::default())
            }
        }
    }
}

/// Config file structure
#[derive(Debug)]
pub struct ConfigFile<'cfg> {
    /// Lazily loaded configuration data
    data: OnceCell<RwLock<ConfigFileData<'cfg>>>,
    /// Path to the configuration file
    file_name: Cow<'cfg, Path>,
}

impl<'cfg> ConfigFile<'cfg> {
    /// Creates a new configuration file
    pub fn new(file_name: impl Into<Cow<'cfg, Path>>) -> Self {
        Self::const_new(file_name.into())
    }

    /// Creates a new configuration file, in a const context
    pub const fn const_new(file_name: Cow<'cfg, Path>) -> Self {
        Self {
            data: OnceCell::const_new(),
            file_name,
        }
    }

    async fn data(&self) -> Result<&RwLock<ConfigFileData<'cfg>>> {
        self.data
            .get_or_try_init(|| async move {
                let res = RwLock::new(ConfigFileData::load(&self.file_name).await?);
                Ok(res)
            })
            .await
    }

    /// Returns the default profile name
    pub async fn default_profile(&self) -> Result<Option<Cow<'_, str>>> {
        Ok(self.data().await?.read().await.default_profile.clone())
    }
}
