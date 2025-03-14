use chrono::Local;
use directories::ProjectDirs;
use eyre::{Context, OptionExt, Result};
use tracing::debug;
use tracing_subscriber::{
    fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer, Registry,
};

/// Initializes the logger
pub async fn init_logger() -> Result<()> {
    let project_dirs = ProjectDirs::from("rs", "Raccoon Studios", "rachat")
        .ok_or_eyre("Code bug, please report")?;

    let state_path = project_dirs
        .state_dir()
        .unwrap_or_else(|| project_dirs.cache_dir());

    let log_path = state_path.join("logs");

    tokio::fs::create_dir_all(&log_path)
        .await
        .context("Creating log directory")?;

    let now = Local::now();
    let log_path = log_path.join(format!("rachat-{}.log", now.format("%Y-%m-%d-%H-%M-%S")));

    let writer = std::fs::File::create(log_path).context("Creating log file")?;

    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .context("Initializing the log filter")?;

    let filter2 = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .context("Initializing the log filter")?;

    Registry::default()
        .with(fmt::layer().pretty().with_filter(filter))
        .with(fmt::layer().json().with_writer(writer).with_filter(filter2))
        .init();

    debug!("Initialized Logger!");

    Ok(())
}
