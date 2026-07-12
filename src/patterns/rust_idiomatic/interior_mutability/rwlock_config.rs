use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigEntry {
    key: String,
    value: String,
}

impl ConfigEntry {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Default)]
pub struct ConfigRegistry {
    entries: RwLock<HashMap<String, String>>,
}

impl ConfigRegistry {
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

    pub fn read(&self, key: &str) -> Option<String> {
        let entries = self
            .entries
            .read()
            .expect("config registry rwlock should not be poisoned");
        entries.get(key).cloned()
    }

    pub fn upsert(&self, entry: ConfigEntry) {
        let mut entries = self
            .entries
            .write()
            .expect("config registry rwlock should not be poisoned");
        entries.insert(entry.key, entry.value);
    }

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
