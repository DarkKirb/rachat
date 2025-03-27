//! The root crate for rachat
use std::sync::Arc;

use clap::Parser;
use eyre::Result;
use rachat_config::{
    Config, ConfigSource, ConfigSourceExt, ConfigurationOverlay, FileConfig, global_config,
};
use rachat_i18n::{Localizer, info};
use rachat_misc::paths::Directories;

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
    configuration: Config,
    /// Localizer handle
    _localizer: Arc<Localizer>,
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

        let directories = Directories::new()?;
        let config_path = directories.config().await?.join("config.toml");

        let global_config = global_config(config_path).await?;

        let profile = match args.profile {
            Some(profile) => profile,
            _ => global_config
                .get::<_, String>("profile.default")?
                .unwrap_or_else(|| "default".to_string()),
        };

        let profile_config: Arc<FileConfig> =
            FileConfig::new(directories.config().await?.join(format!("{profile}.toml"))).await?;

        let configuration: Config = ConfigurationOverlay::new(global_config, profile_config);

        let localizer = Localizer::new(&configuration)?;

        info!(starting_rachat);
        info!(rachat_copyright);
        info!(using_profile, profile = profile);

        Ok(Arc::new(Self {
            configuration,
            _localizer: localizer,
        }))
    }

    /// Access the configuration store
    #[must_use]
    pub fn config(&self) -> Arc<dyn ConfigSource + Send + Sync> {
        Arc::clone(&self.configuration)
    }
}
