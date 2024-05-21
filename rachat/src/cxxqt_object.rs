/// The bridge definition for our QObject
#[cxx_qt::bridge]
pub mod qobject {

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        /// An alias to the QString type
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qurl.h");
        /// An alias to the QString type
        type QUrl = cxx_qt_lib::QUrl;
    }

    unsafe extern "RustQt" {
        // The QObject definition
        // We tell CXX-Qt that we want a QObject class with the name RootWindow
        // based on the Rust struct RootWindow.
        #[qobject]
        #[qml_element]
        #[qproperty(QString, title_string)]
        #[qproperty(QUrl, next_url)]
        type RootWindow = super::RootWindowRust;

        #[qobject]
        #[qml_element]
        #[qproperty(QString, error_string)]
        type SelectHomeserver = super::SelectHomeserverRust;
    }

    impl cxx_qt::Threading for RootWindow {}
    impl cxx_qt::Constructor<()> for RootWindow {}
    impl cxx_qt::Threading for SelectHomeserver {}
    impl cxx_qt::Constructor<()> for SelectHomeserver {}

    unsafe extern "RustQt" {
        #[qinvokable]
        fn select_homeserver(self: &SelectHomeserver, homeserver: QString);
        #[qinvokable]
        fn on_homeserver_text_changed(self: Pin<&mut SelectHomeserver>, homeserver: QString);
    }
}

use core::pin::Pin;
use cxx_qt::{Initialize, Threading};
use cxx_qt_lib::{QString, QUrl};

/// The Rust struct for the QObject
#[derive(Default)]
pub struct RootWindowRust {
    title_string: QString,
    next_url: QUrl,
}

impl Initialize for qobject::RootWindow {
    fn initialize(self: Pin<&mut Self>) {
        let thread = self.qt_thread();
        crate::APP_STATE.set_root_window(thread);
        tokio::spawn(async move {
            if !crate::rachat().data_store().has_client().await {
                crate::APP_STATE.with_root_window(|root_window| {
                    root_window.set_next_url(QUrl::from(
                        "qrc:/qt/qml/rs/chir/rachat/qml/select-homeserver.qml",
                    ));
                })?;
            }
            Ok::<(), anyhow::Error>(())
        });
    }
}

impl Drop for RootWindowRust {
    fn drop(&mut self) {
        crate::APP_STATE.remove_root_window();
        println!("Dropping RootWindowRust");
    }
}

pub use crate::select_homeserver::SelectHomeserverRust;
