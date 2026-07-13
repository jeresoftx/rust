use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ConfigEntry` usado por el ejemplo para expresar el dominio del patron.
pub struct ConfigEntry {
    key: String,
    value: String,
}

impl ConfigEntry {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Modela la operacion `key` dentro del ejemplo del patron.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Modela la operacion `value` dentro del ejemplo del patron.
    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Default)]
/// Tipo publico `ConfigRegistry` usado por el ejemplo para expresar el dominio del patron.
pub struct ConfigRegistry {
    entries: RwLock<HashMap<String, String>>,
}

impl ConfigRegistry {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(entries: Vec<ConfigEntry>) -> Self {
        Self {
            entries: RwLock::new(
                entries
                    .into_iter()
                    .map(|entry| (entry.key, entry.value))
                    .collect(),
            ),
        }
    }

    /// Modela la operacion `read` dentro del ejemplo del patron.
    pub fn read(&self, key: &str) -> Option<String> {
        let entries = self
            .entries
            .read()
            .expect("config registry rwlock should not be poisoned");
        entries.get(key).cloned()
    }

    /// Modela la operacion `upsert` dentro del ejemplo del patron.
    pub fn upsert(&self, entry: ConfigEntry) {
        let mut entries = self
            .entries
            .write()
            .expect("config registry rwlock should not be poisoned");
        entries.insert(entry.key, entry.value);
    }

    /// Modela la operacion `snapshot` dentro del ejemplo del patron.
    pub fn snapshot(&self) -> Vec<ConfigEntry> {
        let entries = self
            .entries
            .read()
            .expect("config registry rwlock should not be poisoned");
        let mut snapshot = entries
            .iter()
            .map(|(key, value)| ConfigEntry::new(key.clone(), value.clone()))
            .collect::<Vec<_>>();

        snapshot.sort_by(|left, right| left.key.cmp(&right.key));
        snapshot
    }
}
