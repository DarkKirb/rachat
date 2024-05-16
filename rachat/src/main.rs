pub mod cxxqt_object;

use std::{env, pin::Pin, time::Duration};

use anyhow::Result;
use cxx_qt::CxxQtThread;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QString, QUrl};
use cxxqt_object::qobject::RootWindow;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rachat_common::Rachat;
use serde::{Deserialize, Serialize};

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

    pub fn remove_root_window(&self) {
        *self.root_window.lock() = None;
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

    Rachat::new().await?;

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
    // bug workaround for missing dark theme on windows
    #[cfg(windows)]
    {
        if env::var("QT_QUICK_CONTROLS_STYLE").is_err() {
            env::set_var("QT_QUICK_CONTROLS_STYLE", "Fusion");
        }
    }
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
