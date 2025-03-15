//! Logging module
//!
//! Contains global logger initialization code

use eyre::{Context, Result};
use tracing_subscriber::{
    EnvFilter, Layer, Registry, fmt, layer::SubscriberExt, util::SubscriberInitExt,
};

/// Initializes the logger
pub fn init() -> Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .context("Initializing the log filter")?;

    #[cfg(windows)]
    {
        Registry::default()
            .with(fmt::layer().pretty().with_filter(filter))
            .with(tracing_etw::LayerBuilder::new(crate::APPLICATION_NAME).build()?)
            .init();
    }
    #[cfg(target_os = "macos")]
    {
        let subsystem_name = format!(
            "{}.{}.{}",
            crate::QUALIFIER,
            crate::ORGANIZATION.replace(' ', "-"),
            crate::APPLICATION_NAME.replace(' ', "-")
        );
        Registry::default()
            .with(fmt::layer().pretty().with_filter(filter))
            .with(tracing_oslog::OsLogger::new(subsystem_name, "default"))
            .init();
    }
    #[cfg(not(any(windows, target_os = "macos")))]
    {
        Registry::default()
            .with(fmt::layer().pretty().with_filter(filter))
            .init();
    }

    Ok(())
}
