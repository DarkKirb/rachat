//! The platform config source
//!
//! It is read-only and contains the default/fallback values for various configuration options

use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use eyre::Result;
use rachat_misc::id_generator;
use serde_json::Value;
use tokio::sync::Notify;

use crate::{ConfigSource, WatcherHandle};

/// The platform configuration
#[derive(Clone, Debug)]
pub struct PlatformConfig {
    /// A reference to itself for the watcher
    own: Weak<Self>,
    /// The platform config
    config: HashMap<String, Value>,
}

impl PlatformConfig {
    /// Add windows-specific properties to the platform racconfiguration
    #[cfg(windows)]
    fn add_windows_properties(hm: &mut HashMap<String, Value>) {
        use winver::WindowsVersion;
        if std::env::var("QT_QUICK_CONTROLS_STYLE").is_ok() {
            return;
        }
        if std::env::args().any(|x| &x == "-style") {
            return;
        }

        // Obtain Windows Version
        let Some(version) = WindowsVersion::detect() else {
            return;
        };

        //  The default is probably fine on Windows 7 and earlier
        if version < WindowsVersion::new(6, 2, 0) {
            return;
        }

        hm.insert(
            "gui.qt.style".to_string(),
            Value::String("Fusion".to_string()),
        );

        #[cfg(any(
            cxxqt_qt_version_at_least_5_7,
            cxxqt_qt_version_at_least_6,
            cxxqt_qt_version_at_least_7
        ))]
        if version >= WindowsVersion::new(10, 0, 0) {
            // Windows 10 and later
            hm.insert(
                "gui.qt.style".to_string(),
                Value::String("Universal".to_string()),
            );
        }

        #[cfg(any(cxxqt_qt_version_at_least_6_8, cxxqt_qt_version_at_least_7))]
        if version >= WindowsVersion::new(10, 0, 22000) {
            // Windows 11 and later
            hm.insert(
                "gui.qt.style".to_string(),
                Value::String("FluentWinUI3".to_string()),
            );
        }
    }

    fn get_langs_value() -> Value {
        let mut langs = Vec::new();
        for lang in sys_locale::get_locales() {
            langs.push(Value::String(lang));
        }
        Value::Array(langs)
    }

    /// Creates a new platform configuration
    pub fn new() -> Arc<Self> {
        let mut config = HashMap::new();

        config.insert(
            "profile.default".to_string(),
            Value::String("default".to_string()),
        );

        config.insert("i18n.langs".to_string(), Self::get_langs_value());

        #[cfg(windows)]
        Self::add_windows_properties(&mut config);

        Arc::new_cyclic(|arc| Self {
            own: arc.clone(),
            config,
        })
    }
}

impl ConfigSource for PlatformConfig {
    fn get_value(&self, key: &str) -> Result<Option<Value>> {
        Ok(self.config.get(key).cloned())
    }

    fn watch_property_with_notify(&self, _key: &str, notify: Arc<Notify>) -> WatcherHandle {
        WatcherHandle {
            watch_id: id_generator::generate(),
            config: self.own.clone(),
            notify,
        }
    }

    fn delete_watcher(&self, _watch_id: u128) {}
}
