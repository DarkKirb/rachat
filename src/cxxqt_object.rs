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

        #[qinvokable]
        #[cxx_name = "transArgs"]
        fn trans_args(self: &MyObject, msgid: &QString, data: &QString) -> QString;
    }
}

use crate::info;
use core::pin::Pin;
use cxx_qt_lib::QString;
use fluent_bundle::FluentValue;
use serde_json::Value;
use std::{borrow::Cow, collections::HashMap};

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
        let res = crate::i18n::ඞ::localize(&key, None);
        QString::from(res)
    }

    /// Translate a string with arguments
    pub fn trans_args(&self, string: &QString, data: &QString) -> QString {
        let key = string.to_string();
        let data = data.to_string();

        let Ok(data): Result<HashMap<String, Value>, _> = serde_json::from_str(&data) else {
            return QString::from(format!("[!!! Error !!! {string}, {data}]"));
        };

        let mut fixed_data = HashMap::with_capacity(data.len());

        for (k, v) in data {
            let v = match v {
                Value::String(s) => FluentValue::String(Cow::Owned(s)),
                Value::Number(s) => s.as_u128().map_or_else(
                    || {
                        s.as_i128().map_or_else(
                            || {
                                s.as_f64().map_or_else(
                                    || FluentValue::Error,
                                    |v| FluentValue::Number(v.into()),
                                )
                            },
                            |v| FluentValue::Number(v.into()),
                        )
                    },
                    |v| FluentValue::Number(v.into()),
                ),
                Value::Null => FluentValue::None,
                _ => FluentValue::Error,
            };

            fixed_data.insert(Cow::Owned(k), v);
        }

        let res = crate::i18n::ඞ::localize(&key, Some(&fixed_data));
        QString::from(res)
    }
}
