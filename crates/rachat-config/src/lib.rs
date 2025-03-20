//! Configuration code for rachat
//!
//! Rachat has a complex configuration system consisting of several layers:
//!
//! 1. Platform Configuration
//!    
//!    This is the default configuration for the current system. These provide the fallback values and are system-dependent.
//!
//! 2. Build-time Configuration
//!
//!    Distributors may override the platform defaults with their own values.
//!    By default, it is empty.

use std::{
    collections::BTreeMap,
    fmt::Debug,
    path::Path,
    sync::{Arc, Weak},
};

use eyre::Result;
use file_config::FileConfig;
use parking_lot::Mutex;
use platform_config::PlatformConfig;
use rachat_misc::id_generator;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use static_config::StaticConfig;
use tokio::sync::Notify;

mod de;
mod file_config;
mod platform_config;
mod ser;
mod static_config;

/// A handle for the watcher
///
/// Dropping it will automatically end the notifications from being delivered
#[derive(Debug)]
pub struct WatcherHandle {
    /// The unique Watch ID
    watch_id: u128,
    /// The config this watch belongs to
    config: Weak<dyn ConfigSource + Send + Sync>,
    /// The notifier
    notify: Arc<Notify>,
}

impl WatcherHandle {
    /// Await a configuration value change
    pub async fn notified(&self) {
        self.notify.notified().await;
    }
}

impl Drop for WatcherHandle {
    fn drop(&mut self) {
        if let Some(arc) = self.config.upgrade() {
            arc.delete_watcher(self.watch_id);
        }
    }
}

/// A single configuration source
pub trait ConfigSource: Debug {
    /// Retrieves a configuration value from this source
    ///
    /// This returns an untyped [`Value`] object
    ///
    /// [`Value`]: serde_json::Value
    ///
    /// # Errors
    ///
    /// This function returns an error if the configuration value could not be deserialized into a serde value
    fn get_value(&self, key: &str) -> Result<Option<Value>>;

    /// Returns true if the config store is writeable
    fn is_writeable(&self) -> bool {
        false
    }

    /// Writes a configuration value to this source
    ///
    /// # Errors
    ///
    /// This function returns an error if the configuration value could not be serialized, or the configuration file couldn’t be saved, or the configuration file doesn’t support writing.
    fn set_value(&self, _key: &str, _value: Value) -> Result<()> {
        eyre::bail!("Configuration store is not writeable")
    }

    /// Deletes a configuration value from this store.
    ///
    /// # Errors
    ///
    /// This function returns an error if the configuration source is not modifiable.
    fn delete_inner(&self, _key: &str) -> Result<()> {
        eyre::bail!("Configuration store is not writeable")
    }

    /// Adds a watcher for a specific property name
    ///
    /// This watcher uses [`Notify`] to resume execution of tasks that wait for config values to change.
    ///
    /// [`Notify`]: tokio::sync::Notify
    fn watch_property(&self, key: &str) -> WatcherHandle {
        let notify = Arc::new(Notify::new());
        self.watch_property_with_notify(key, notify)
    }

    /// Adds a watcher for a specific property name
    ///
    /// This watcher uses [`Notify`] to resume execution of tasks that wait for config values to change.
    ///
    /// [`Notify`]: tokio::sync::Notify
    fn watch_property_with_notify(&self, key: &str, notify: Arc<Notify>) -> WatcherHandle;

    /// Deletes a watcher by ID.
    ///
    /// This is not intended to be directly used. It is for use by the `Drop` impl of [`WatcherHandle`]
    ///
    /// [`WatcherHandle`]: WatcherHandle
    fn delete_watcher(&self, watch_id: u128);
}

/// Config source extension trait
///
/// This is used for more type-safe wrappers to the [`ConfigSource`] items
///
/// [`ConfigSource`]: ConfigSource
pub trait ConfigSourceExt: ConfigSource + Send + Sync {
    /// Retrieves a configuration value from this source
    ///
    /// # Errors
    ///
    /// This function returns an error if the configuration value could not be deserialized into the provided type
    fn get<'de, N: AsRef<str> + Send + Sync, D: Deserialize<'de> + 'de>(
        &self,
        key: N,
    ) -> Result<Option<D>> {
        match self.get_value(key.as_ref()) {
            Ok(None) => Ok(None),
            Ok(Some(v)) => Ok(Some(D::deserialize(v)?)),
            Err(e) => Err(eyre::eyre!(e)),
        }
    }

    /// Sets a configuration value on this source
    ///
    /// # Errors
    ///
    /// This function returns an error if the configuration value could not be serialized, or the configuration file couldn’t be saved, or the configuration file doesn’t support writing.
    fn set<N: AsRef<str> + Send + Sync, S: Serialize + Send>(
        &self,
        key: N,
        value: S,
    ) -> Result<()> {
        let value = value.serialize(serde_json::value::Serializer)?;
        self.set_value(key.as_ref(), value)
    }

    /// Deletes a configuration value from this store.
    ///
    /// # Errors
    ///
    /// This function returns an error if the configuration source is not modifiable.
    fn delete<N: AsRef<str> + Send + Sync>(&self, key: N) -> Result<()> {
        self.delete_inner(key.as_ref())
    }
}

impl<T: ConfigSource + Send + Sync + ?Sized> ConfigSourceExt for T {}

/// A configuration overlay, a configuration source that overlays on top of some other configuration overlay
#[derive(Debug)]
pub struct ConfigurationOverlay<P, S>
where
    P: ConfigSource,
    S: ConfigSource,
{
    /// A reference to itself for the watcher
    own: Weak<Self>,
    /// The parent configuration source
    parent: Arc<P>,
    /// The main configuration source
    source: Arc<S>,
    /// A mapper of subscriber IDs to notifies
    notifiers: Mutex<BTreeMap<u128, (WatcherHandle, WatcherHandle)>>,
}

impl<P, S> ConfigurationOverlay<P, S>
where
    P: ConfigSource,
    S: ConfigSource,
{
    /// Creates a new layer configuration source
    pub fn new(parent: Arc<P>, source: Arc<S>) -> Arc<Self> {
        Arc::new_cyclic(|arc| Self {
            own: arc.clone(),
            parent,
            source,
            notifiers: Mutex::new(BTreeMap::new()),
        })
    }
}

impl<P, S> ConfigSource for ConfigurationOverlay<P, S>
where
    P: ConfigSource + Send + Sync + 'static,
    S: ConfigSource + Send + Sync + 'static,
{
    fn get_value(&self, key: &str) -> Result<Option<Value>> {
        match self.source.get_value(key) {
            Ok(Some(v)) => Ok(Some(v)),
            _ => self.parent.get_value(key),
        }
    }

    fn is_writeable(&self) -> bool {
        self.source.is_writeable()
    }

    fn set_value(&self, key: &str, value: Value) -> Result<()> {
        self.source.set_value(key, value)
    }

    fn delete_inner(&self, key: &str) -> Result<()> {
        self.source.delete_inner(key)
    }

    fn watch_property_with_notify(&self, key: &str, notify: Arc<Notify>) -> WatcherHandle {
        let parent = self
            .parent
            .watch_property_with_notify(key, Arc::clone(&notify));
        let child = self
            .source
            .watch_property_with_notify(key, Arc::clone(&notify));
        let id = id_generator::generate();

        self.notifiers.lock().insert(id, (parent, child));

        WatcherHandle {
            watch_id: id,
            config: self.own.clone(),
            notify,
        }
    }

    fn delete_watcher(&self, watch_id: u128) {
        self.notifiers.lock().remove(&watch_id);
    }
}

/// Returns the global configuration for rachat, given its config location
///
/// # Errors
///
/// This function returns an error if the configuration is invalid
pub async fn global_config(
    config_location: impl AsRef<Path>,
) -> Result<Arc<dyn ConfigSource + Send + Sync>> {
    let platform_config = PlatformConfig::new();
    let static_config = StaticConfig::new()?;
    let file_config = FileConfig::new(config_location.as_ref()).await?;
    Ok(ConfigurationOverlay::new(
        ConfigurationOverlay::new(platform_config, static_config),
        file_config,
    ))
}
