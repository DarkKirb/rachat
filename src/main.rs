pub mod cxxqt_object;

use std::{pin::Pin, sync::Arc, time::Duration};

use anyhow::{Context, Result};
use cxx_qt::CxxQtThread;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QString, QUrl};
use cxxqt_object::qobject::RootWindow;
use directories_next::ProjectDirs;
use once_cell::sync::Lazy;
use parking_lot::{Mutex, RwLock};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tracing::debug;

static PROJECT_DIRS: Lazy<ProjectDirs> =
    Lazy::new(|| ProjectDirs::from("rs", "Raccoon Productions", "rachat").unwrap());

pub struct AppState {
    root_window: Mutex<Option<CxxQtThread<RootWindow>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            root_window: Mutex::new(None),
        }
    }

    pub fn with_root_window<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce(Pin<&mut RootWindow>) + Send + 'static,
    {
        let queue = self.root_window.lock();
        if let Some(w) = queue.as_ref() {
            w.queue(f)?;
        }
        Ok(())
    }

    pub fn set_root_window(&self, w: CxxQtThread<RootWindow>) {
        *self.root_window.lock() = Some(w);
    }
}

static APP_STATE: Lazy<AppState> = Lazy::new(AppState::new);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub default_profile: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_profile: "default".to_string(),
        }
    }
}

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
        Config::default()
    };

    // Rewrite the configuration file
    fs::write(config_path, serde_json::to_string(&config)?)
        .await
        .context("Creating the default configuration file")?;

    println!("{config:?}");

    tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(5)).await;
        APP_STATE
            .with_root_window(|root_window| {
                root_window.set_title_string(QString::from("Hello, World!"));
            })
            .unwrap();
    });

    // Create the application and engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Load the QML path into the engine
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/rs/chir/rachat/qml/main.qml"));
    }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }
    Ok(())
}
