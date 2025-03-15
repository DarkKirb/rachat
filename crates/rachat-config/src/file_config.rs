//! Mutable configuration file on the disk

use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::{Arc, Weak, mpsc},
    thread,
    time::Duration,
};

use async_trait::async_trait;
use eyre::Result;
use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_full::{DebounceEventResult, Debouncer, RecommendedCache, new_debouncer};
use rachat_misc::id_generator;
use serde_json::Value;
use tokio::sync::{Mutex, Notify, RwLock};
use tracing::error;

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

    /// Notifies all relevant listeners
    async fn notify_path(&self, path: &str) {
        let listener_ids = self.path_listeners.read().await.get(path).cloned();

        if let Some(listener_ids) = listener_ids {
            for listener_id in &listener_ids {
                let l = self.id_to_notifies.read().await.get(listener_id).cloned();

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
        let new_config = Self::read_config(&self.fname).await?;
        let mut old_config = new_config.clone();
        std::mem::swap(&mut old_config, &mut *self.config.write().await);
        let mut keyset = HashSet::new();
        keyset.extend(new_config.keys().cloned());
        keyset.extend(old_config.keys().cloned());
        for key in keyset {
            if new_config.get(&key) != old_config.get(&key) {
                self.notify_path(&key).await;
            }
        }
        Ok(())
    }

    /// Creates a new mutable configuration file
    ///
    pub async fn new(fname: impl Into<PathBuf>) -> Result<Arc<Self>> {
        let fname: PathBuf = fname.into();

        let config = Self::read_config(&fname).await?;

        let (tx, rx) = mpsc::channel::<DebounceEventResult>();

        let mut watcher = new_debouncer(Duration::from_millis(250), None, tx)?;

        watcher.watch(&fname, RecursiveMode::NonRecursive)?;

        Ok(Arc::new_cyclic(|arc: &Weak<Self>| {
            let fname2 = fname.clone();
            let arc2 = arc.clone();

            thread::spawn(move || {
                while let Ok(event) = rx.recv() {
                    if let Err(e) = event {
                        error!("Error watching {fname2:?}: {e:?}");
                    } else if let Some(arc) = arc2.upgrade() {
                        let fname3 = fname2.clone();
                        tokio::spawn(async move {
                            if let Err(e) = arc.notify_change().await {
                                error!("Failed to notify file change to {fname3:?}: {e:?}");
                            }
                        });
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

#[async_trait]
impl ConfigSource for FileConfig {
    fn get_value(&self, key: &str) -> Result<Option<Value>> {
        Ok(self.config.blocking_read().get(key).cloned())
    }

    fn is_writeable(&self) -> bool {
        true
    }

    async fn set_value(&self, key: &str, value: Value) -> Result<()> {
        self.config.write().await.insert(key.to_string(), value);
        self.notify_path(key).await;

        let as_json_value = crate::ser::serialize(&*self.config.read().await)?;

        tokio::fs::write(&self.fname, toml::to_string_pretty(&as_json_value)?).await?;

        Ok(())
    }

    async fn delete_inner(&self, key: &str) -> Result<()> {
        self.config.write().await.remove(key);
        self.notify_path(key).await;

        let as_json_value = crate::ser::serialize(&*self.config.read().await)?;

        tokio::fs::write(&self.fname, toml::to_string_pretty(&as_json_value)?).await?;

        Ok(())
    }

    async fn watch_property_with_notify(&self, key: &str, notify: Arc<Notify>) -> WatcherHandle {
        let id = id_generator::generate();
        self.id_to_notifies
            .write()
            .await
            .insert(id, Arc::clone(&notify));
        self.notifiers.lock().await.insert(id, key.to_string());
        self.path_listeners
            .write()
            .await
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
        let Some(path) = self.notifiers.blocking_lock().remove(&watch_id) else {
            return;
        };
        {
            let mut listeners = self.path_listeners.blocking_write();
            if let Some(listener_ids) = listeners.get_mut(&path) {
                listener_ids.remove(&watch_id);
                if listener_ids.is_empty() {
                    listeners.remove(&path);
                }
            }
        }
        todo!();
    }
}
