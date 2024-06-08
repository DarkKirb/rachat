pub mod cxxqt_object;
pub mod login_window;
pub mod pages;
pub mod select_homeserver;

use std::{fmt::Debug, future::Future, pin::Pin, sync::Arc, time::Duration};

use cxx_qt::CxxQtThread;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QString, QUrl};
use cxxqt_object::qobject::RootWindow;
use eyre::Result;
use once_cell::sync::{Lazy, OnceCell};
use pages::RachatPages;
use parking_lot::Mutex;
use rachat_common::Rachat;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, info_span, warn};

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
        debug!("Requesting root window");
        let queue = self.root_window.lock();
        if let Some(w) = queue.as_ref() {
            w.queue(f)?;
        } else {
            warn!("Lost event due to missing root window");
        }
        Ok(())
    }

    pub fn set_root_window(&self, w: CxxQtThread<RootWindow>) {
        info!("Setting root window");
        *self.root_window.lock() = Some(w);
    }

    pub fn remove_root_window(&self) {
        *self.root_window.lock() = None;
    }

    /// Sets the window title asynchronously.
    pub fn set_window_title<S>(&self, title: S) -> Result<()>
    where
        S: Into<QString> + AsRef<str> + Send + 'static,
    {
        let span = info_span!("set_window_title", title = title.as_ref());
        let span2 = span.clone();
        let _guard = span2.enter();
        self.with_root_window(move |root_window| {
            let _guard = span.enter();
            root_window.set_title_string(title.into());
        })?;
        Ok(())
    }

    /// Navigate to the given URL asynchronously.
    pub fn navigate<S>(&self, url: S) -> Result<()>
    where
        S: Into<QUrl> + AsRef<str> + Send + 'static,
    {
        let span = info_span!("navigate", url = url.as_ref());
        let span2 = span.clone();
        let _guard = span2.enter();
        self.with_root_window(move |root_window| {
            let _guard = span.enter();
            root_window.set_next_url(url.into());
        })?;
        Ok(())
    }

    pub fn spawn<F, Fut>(&self, fun: F)
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        tokio::spawn(async move {
            let result = fun().await;
            if let Err(e) = result {
                warn!("Error in spawned future: {e:?}");
            }
        });
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

static APP_STATE: Lazy<AppState> = Lazy::new(Default::default);

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

static RACHAT: OnceCell<Arc<Rachat>> = OnceCell::new();
pub fn rachat() -> Arc<Rachat> {
    RACHAT.get().unwrap().clone()
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    RACHAT.set(Rachat::new().await?).unwrap();

    tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(5)).await;
        APP_STATE
            .with_root_window(|root_window| {
                root_window.set_title_string(QString::from("Hello, World!"));
            })
            .unwrap();
    });

    // Start the app
    tokio::task::spawn_blocking(|| {
        // bug workaround for missing dark theme on windows
        #[cfg(windows)]
        {
            if std::env::var("QT_QUICK_CONTROLS_STYLE").is_err() {
                std::env::set_var("QT_QUICK_CONTROLS_STYLE", "Fusion");
            }
        }
        // Create the application and engine
        let mut app = QGuiApplication::new();
        let mut engine = QQmlApplicationEngine::new();

        // Load the QML path into the engine
        if let Some(engine) = engine.as_mut() {
            engine.load(&RachatPages::Root.into());
        }
        if let Some(app) = app.as_mut() {
            app.exec();
        }
    })
    .await?;
    Ok(())
}
