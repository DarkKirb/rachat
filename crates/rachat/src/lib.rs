use eyre::{Context, Result};
use tracing::debug;
use tracing_subscriber::{
    fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer, Registry,
};

/// Initializes the logger
pub async fn init_logger() -> Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .context("Initializing the log filter")?;

    #[cfg(windows)]
    {
        Registry::default()
            .with(fmt::layer().pretty().with_filter(filter))
            .with(tracing_etw::LayerBuilder::new("rachat").build()?)
            .init();
    }
    #[cfg(target_os = "macos")]
    {
        Registry::default()
            .with(fmt::layer().pretty().with_filter(filter))
            .with(tracing_oslog::OsLogger::new("rs.chir.rachat", "default"))
            .init();
    }
    #[cfg(not(any(windows, target_os = "macos")))]
    {
        Registry::default()
            .with(fmt::layer().pretty().with_filter(filter))
            .init();
    }

    debug!("Initialized Logger!");

    Ok(())
}
