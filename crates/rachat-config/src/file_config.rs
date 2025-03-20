//! Mutable configuration file on the disk

use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::{Arc, Weak},
    time::Duration,
};

use eyre::Result;
use notify::{
    EventKind, RecommendedWatcher, RecursiveMode,
    event::{AccessKind, AccessMode},
};
use notify_debouncer_full::{DebounceEventResult, Debouncer, RecommendedCache, new_debouncer};
use parking_lot::{Mutex, RwLock};
use rachat_misc::id_generator;
use serde_json::Value;
use tokio::sync::Notify;
use tracing::{error, info};

use crate::{ConfigSource, WatcherHandle};

/// The mutable configuration file
#[derive(Debug)]
pub struct FileConfig {
    /// A reference to itself for the watcher
    own: Weak<Self>,
    /// The file name of the configuration
    fname: PathBuf,
    /// The platform config
    config: RwLock<HashMap<String, Value>>,
    /// The file system watcher to check for changes
    _watcher: Debouncer<RecommendedWatcher, RecommendedCache>,
    /// Map of paths to listener IDs
    path_listeners: RwLock<HashMap<String, HashSet<u128>>>,
    /// Map of listener IDs to paths
    notifiers: Mutex<HashMap<u128, String>>,
    /// Map of listener IDs to notifies
    id_to_notifies: RwLock<HashMap<u128, Arc<Notify>>>,
}

impl FileConfig {
    /// Reads the configuration file and returns the deserialized value
    async fn read_config(fname: &Path) -> Result<HashMap<String, Value>> {
        if !tokio::fs::try_exists(fname).await? {
            tokio::fs::write(fname, b"").await?;
        }
        let content = tokio::fs::read_to_string(fname).await?;
        let toml: Value = toml::de::from_str(&content)?;
        Ok(crate::de::deserialize(toml))
    }

    /// Writes the configuration file
    async fn write_config(this: Weak<Self>) {
        if let Some(arc) = this.upgrade() {
            let as_json_value = match crate::ser::serialize(&arc.config.read()) {
                Ok(v) => v,
                Err(e) => {
                    error!("Failed serializing updated configuration file: {e:?}");
                    return;
                }
            };
            let toml_string = match toml::to_string_pretty(&as_json_value) {
                Ok(v) => v,
                Err(e) => {
                    error!("Failed serializing updated configuration file: {e:?}");
                    return;
                }
            };
            if let Err(e) = tokio::fs::write(&arc.fname, toml_string).await {
                error!("Failed writing updated configuration: {e:?}")
            }
        }
    }

    /// Notifies all relevant listeners
    fn notify_path(&self, path: &str) {
        let listener_ids = self.path_listeners.read().get(path).cloned();

        if let Some(listener_ids) = listener_ids {
            for listener_id in &listener_ids {
                let l = self.id_to_notifies.read().get(listener_id).cloned();

                if let Some(l) = l {
                    l.notify_one();
                } else {
                    error!("No notifier for listener ID {}", listener_id);
                }
            }
        }
    }

    /// Notifies for a change
    async fn notify_change(&self) -> Result<()> {
        info!("Reloading config file {:?}", self.fname);
        let new_config = Self::read_config(&self.fname).await?;
        let mut old_config = new_config.clone();
        std::mem::swap(&mut old_config, &mut *self.config.write());
        let mut keyset = HashSet::new();
        keyset.extend(new_config.keys().cloned());
        keyset.extend(old_config.keys().cloned());
        for key in keyset {
            if new_config.get(&key) != old_config.get(&key) {
                self.notify_path(&key);
            }
        }
        Ok(())
    }

    /// Creates a new mutable configuration file
    ///
    pub async fn new(fname: impl Into<PathBuf>) -> Result<Arc<Self>> {
        let fname: PathBuf = fname.into();

        let config = Self::read_config(&fname).await?;

        let event = Arc::new(Notify::new());
        let event2 = Arc::clone(&event);

        let mut watcher = new_debouncer(
            Duration::from_millis(250),
            None,
            move |e: DebounceEventResult| match e {
                Ok(evs) => {
                    for ev in evs {
                        if ev.event.kind == EventKind::Access(AccessKind::Close(AccessMode::Write))
                        {
                            event.notify_one();
                        }
                    }
                }
                Err(errs) => {
                    for err in errs {
                        error!("Error while listening to fs notifications: {err:?}");
                    }
                }
            },
        )?;

        watcher.watch(&fname, RecursiveMode::NonRecursive)?;

        Ok(Arc::new_cyclic(|arc: &Weak<Self>| {
            let fname2 = fname.clone();
            let arc2 = arc.clone();

            tokio::task::spawn(async move {
                loop {
                    event2.notified().await;
                    if let Some(arc) = arc2.upgrade() {
                        if let Err(e) = arc.notify_change().await {
                            error!("Failed to notify file change to {fname2:?}: {e:?}");
                        }
                    } else {
                        break;
                    }
                }
            });

            Self {
                own: arc.clone(),
                fname,
                config: RwLock::new(config),
                _watcher: watcher,
                path_listeners: RwLock::new(HashMap::new()),
                notifiers: Mutex::new(HashMap::new()),
                id_to_notifies: RwLock::new(HashMap::new()),
            }
        }))
    }
}

impl ConfigSource for FileConfig {
    fn get_value(&self, key: &str) -> Result<Option<Value>> {
        Ok(self.config.read().get(key).cloned())
    }

    fn is_writeable(&self) -> bool {
        true
    }

    fn set_value(&self, key: &str, value: Value) -> Result<()> {
        self.config.write().insert(key.to_string(), value);
        self.notify_path(key);

        tokio::spawn(Self::write_config(self.own.clone()));

        Ok(())
    }

    fn delete_inner(&self, key: &str) -> Result<()> {
        self.config.write().remove(key);
        self.notify_path(key);

        tokio::spawn(Self::write_config(self.own.clone()));

        Ok(())
    }

    fn watch_property_with_notify(&self, key: &str, notify: Arc<Notify>) -> WatcherHandle {
        let id = id_generator::generate();
        self.id_to_notifies.write().insert(id, Arc::clone(&notify));
        self.notifiers.lock().insert(id, key.to_string());
        self.path_listeners
            .write()
            .entry(key.to_string())
            .or_default()
            .insert(id);
        WatcherHandle {
            watch_id: id,
            config: self.own.clone(),
            notify,
        }
    }

    fn delete_watcher(&self, watch_id: u128) {
        let Some(path) = self.notifiers.lock().remove(&watch_id) else {
            return;
        };
        {
            let mut listeners = self.path_listeners.write();
            if let Some(listener_ids) = listeners.get_mut(&path) {
                listener_ids.remove(&watch_id);
                if listener_ids.is_empty() {
                    listeners.remove(&path);
                }
            }
        }
    }
}
