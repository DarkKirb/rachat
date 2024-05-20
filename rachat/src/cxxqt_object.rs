/// The bridge definition for our QObject
#[cxx_qt::bridge]
pub mod qobject {

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        /// An alias to the QString type
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        // The QObject definition
        // We tell CXX-Qt that we want a QObject class with the name RootWindow
        // based on the Rust struct RootWindow.
        #[qobject]
        #[qml_element]
        #[qproperty(QString, title_string)]
        type RootWindow = super::RootWindowRust;

        #[qobject]
        #[qml_element]
        type SelectHomeserver = super::SelectHomeserverRust;
    }

    impl cxx_qt::Threading for RootWindow {}
    impl cxx_qt::Constructor<()> for RootWindow {}

    unsafe extern "RustQt" {
        #[qinvokable]
        fn select_homeserver(self: &SelectHomeserver, homeserver: QString);
    }
}

use core::pin::Pin;
use cxx_qt::{Initialize, Threading};
use cxx_qt_lib::QString;

/// The Rust struct for the QObject
#[derive(Default)]
pub struct RootWindowRust {
    title_string: QString,
}

impl qobject::RootWindow {}

impl Initialize for qobject::RootWindow {
    fn initialize(self: Pin<&mut Self>) {
        let thread = self.qt_thread();
        crate::APP_STATE.set_root_window(thread);
    }
}

impl Drop for RootWindowRust {
    fn drop(&mut self) {
        crate::APP_STATE.remove_root_window();
        println!("Dropping RootWindowRust");
    }
}

pub use crate::select_homeserver::SelectHomeserverRust;
