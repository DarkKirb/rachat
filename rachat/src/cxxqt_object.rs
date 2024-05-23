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

        #[qobject]
        #[qml_element]
        #[qproperty(QString, homeserver)]
        type LoginWindow = super::LoginWindowRust;
    }

    impl cxx_qt::Threading for RootWindow {}
    impl cxx_qt::Constructor<()> for RootWindow {}
    impl cxx_qt::Threading for SelectHomeserver {}
    impl cxx_qt::Constructor<()> for SelectHomeserver {}
    impl cxx_qt::Threading for LoginWindow {}
    impl cxx_qt::Constructor<()> for LoginWindow {}

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
            let has_no_client = rachat()
                .data_store()
                .with_client(|client| async move {
                    if !client.logged_in() {
                        APP_STATE.navigate(RachatPages::Login)?;
                    } else {
                        todo!();
                    }
                    Ok(())
                })
                .await?
                .is_none();
            if has_no_client {
                APP_STATE.navigate(RachatPages::SelectHomeserver)?;
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

pub use crate::{login_window::LoginWindowRust, select_homeserver::SelectHomeserverRust};
use crate::{pages::RachatPages, rachat, APP_STATE};
