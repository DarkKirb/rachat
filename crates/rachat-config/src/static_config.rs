//! Static build-time configuration by the packager

use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use eyre::Result;
use rachat_misc::id_generator;
use serde_json::Value;
use tokio::sync::Notify;

use crate::{ConfigSource, WatcherHandle};

/// The static configuration
#[derive(Clone, Debug)]
pub struct StaticConfig {
    /// A reference to itself for the watcher
    own: Weak<Self>,
    /// The platform config
    config: HashMap<String, Value>,
}
impl StaticConfig {
    /// Create a new static configuration
    ///
    /// # Errors
    /// This function returns an error if the config.toml file is malformed.
    pub fn new() -> Result<Arc<Self>> {
        const CONFIG_TOML: &str = include_str!("../config.toml");

        let config_value: Value = toml::de::from_str(CONFIG_TOML)?;

        Ok(Arc::new_cyclic(|arc| Self {
            own: arc.clone(),
            config: crate::de::deserialize(config_value),
        }))
    }
}

impl ConfigSource for StaticConfig {
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
