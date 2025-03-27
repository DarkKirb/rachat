//! CXX-QT bridge module
#![expect(unsafe_code, reason = "Needed for c++ interop")]
#![expect(clippy::unnecessary_box_returns, reason = "generated code")]
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
        // We tell CXX-Qt that we want a QObject class with the name MyObject
        // based on the Rust struct MyObjectRust.
        #[qobject]
        #[qml_element]
        #[qproperty(i32, number)]
        #[qproperty(QString, string)]
        #[namespace = "my_object"]
        type MyObject = super::MyObjectRust;
    }

    unsafe extern "RustQt" {
        // Declare the invokable methods we want to expose on the QObject
        #[qinvokable]
        #[cxx_name = "incrementNumber"]
        fn increment_number(self: Pin<&mut MyObject>);

        #[qinvokable]
        #[cxx_name = "sayHi"]
        fn say_hi(self: &MyObject, string: &QString, number: i32);

        #[qinvokable]
        #[cxx_name = "trans"]
        fn trans(self: &MyObject, string: &QString) -> QString;
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;
use rachat_i18n::info;

/// The Rust struct for the `QObject`
#[derive(Default)]
pub struct MyObjectRust {
    /// The stored number
    number: i32,
    /// The formatted string
    string: QString,
}

impl qobject::MyObject {
    /// Increment the number `Q_PROPERTY`
    pub fn increment_number(self: Pin<&mut Self>) {
        let previous = *self.number();
        self.set_number(previous + 1);
    }

    /// Print a log message with the given string and number
    pub fn say_hi(&self, string: &QString, number: i32) {
        let string = string.to_string();
        info!(rust_test_hello, string = string, number = number);
    }

    /// Translates a string
    pub fn trans(&self, string: &QString) -> QString {
        let key = string.to_string();
        let res = rachat_i18n::ඞ::localize(&key, None);
        QString::from(res)
    }
}
