pub mod cxxqt_object;

use anyhow::{Context, Result};
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
use directories_next::ProjectDirs;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tracing::debug;

static PROJECT_DIRS: Lazy<ProjectDirs> =
    Lazy::new(|| ProjectDirs::from("rs", "Raccoon Productions", "rachat").unwrap());

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Config {}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    debug!("Ensuring that the config directory exists");
    fs::create_dir_all(PROJECT_DIRS.config_dir())
        .await
        .context("Creating the configuration directory")?;

    // Load the config
    debug!("Loading/Creating the config");
    let config_path = PROJECT_DIRS.config_dir().join("config.json");
    let config: Config = if config_path.exists() {
        let config_str = fs::read_to_string(&config_path)
            .await
            .context("Reading the configuration file")?;
        serde_json::from_str(&config_str).context("Parsing the configuration file")?
    } else {
        let cfg = Config::default();
        // Save the default configuration file
        fs::write(config_path, serde_json::to_string(&cfg)?)
            .await
            .context("Creating the default configuration file")?;
        cfg
    };

    println!("{config:?}");

    // Create the application and engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Load the QML path into the engine
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/com/kdab/cxx_qt/demo/qml/main.qml"));
    }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }
    Ok(())
}
