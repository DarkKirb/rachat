//! The root crate for rachat
use std::sync::Arc;

use clap::Parser;
use eyre::Result;
use rachat_config::{
    ConfigSource, ConfigSourceExt, ConfigurationOverlay, FileConfig, global_config,
};
use rachat_misc::paths::Directories;
use tracing::info;

/// Command line arguments for rachat
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, ignore_errors = true)]
struct Args {
    /// Profile to use. Each profile is isolated from each other, except for certain global settings.
    #[arg(short, long)]
    profile: Option<String>,
}

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

        let args = Args::parse();

        info!("Starting rachat…");
        info!(
            "Rachat is Free Software, released under the {} license. You can find the source code at {}.",
            env!("CARGO_PKG_LICENSE"),
            env!("CARGO_PKG_REPOSITORY")
        );

        let directories = Directories::new()?;
        let config_path = directories.config().await?.join("config.toml");

        let global_config = global_config(config_path).await?;

        let profile = match args.profile {
            Some(profile) => profile,
            _ => global_config
                .get::<_, String>("profile.default")?
                .unwrap_or_else(|| "default".to_string()),
        };

        info!("Using profile {profile}");

        let profile_config: Arc<FileConfig> =
            FileConfig::new(directories.config().await?.join(format!("{profile}.toml"))).await?;

        Ok(Arc::new(Self {
            configuration: ConfigurationOverlay::new(global_config, profile_config),
        }))
    }

    /// Access the configuration store
    #[must_use]
    pub fn config(&self) -> Arc<dyn ConfigSource + Send + Sync> {
        Arc::clone(&self.configuration)
    }
}
