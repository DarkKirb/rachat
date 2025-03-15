//! Entrypoint for the QT GUI
pub mod cxxqt_object;

use std::sync::Arc;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QQuickStyle, QString, QUrl};
use eyre::Result;
use rachat::Rachat;
use rachat_config::ConfigSourceExt;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let rachat = Rachat::new().await?;
    tokio::task::spawn_blocking(|| start(rachat));
    Ok(())
}

/// Start the QT Application
#[allow(clippy::needless_pass_by_value)]
fn start(rachat: Arc<Rachat>) -> Result<()> {
    if let Some(qml_style) = rachat.config().get::<_, String>("qt.style")? {
        info!("Setting QML Style to: {qml_style}");
        QQuickStyle::set_style(&QString::from(&*qml_style));
    }
    // Create the application and engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Load the QML path into the engine
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/com/kdab/cxx_qt/demo/qml/main.qml"));
    }

    if let Some(engine) = engine.as_mut() {
        // Listen to a signal from the QML Engine
        engine
            .as_qqmlengine()
            .on_quit(|_| {
                println!("QML Quit!");
            })
            .release();
    }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }

    Ok(())
}
