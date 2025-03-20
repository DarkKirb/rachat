//! The root crate for rachat
use std::sync::Arc;

use eyre::Result;
use rachat_config::{ConfigSource, global_config};
use rachat_misc::paths::Directories;
use tracing::info;

/// Main Rachat application
#[derive(Clone, Debug)]
pub struct Rachat {
    /// Configuration store for rachat
    configuration: Arc<dyn ConfigSource + Send + Sync>,
}

impl Rachat {
    /// Initializes the main rachat application
    ///
    /// # Errors
    ///
    /// This function returns an error if a fatal error occurs during startup.
    pub async fn new() -> Result<Arc<Self>> {
        rachat_misc::logging::init()?;

        info!("Starting rachat…");
        info!(
            "Rachat is Free Software, released under the {} license. You can find the source code at {}.",
            env!("CARGO_PKG_LICENSE"),
            env!("CARGO_PKG_REPOSITORY")
        );

        let directories = Directories::new()?;
        let config_path = directories.config().await?.join("config.toml");

        Ok(Arc::new(Self {
            configuration: global_config(config_path).await?,
        }))
    }

    /// Access the configuration store
    #[must_use]
    pub fn config(&self) -> Arc<dyn ConfigSource + Send + Sync> {
        Arc::clone(&self.configuration)
    }
}
