//! Logging module
//!
//! Contains global logger initialization code

use eyre::Result;
use tracing_subscriber::{
    EnvFilter, Layer, Registry, fmt, layer::SubscriberExt, util::SubscriberInitExt,
};

/// Initializes the logger
///
/// # Errors
///
/// On Windows, initializing the Event Tracing for Windows logger may fail.
///
/// # Panics
///
/// This function will panic if, due to a bug in this code, the default log filter is invalid.
pub fn init() -> Result<()> {
    #[expect(clippy::expect_used, reason = "This should never happen")]
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .expect("The default log filter should be valid!");

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
