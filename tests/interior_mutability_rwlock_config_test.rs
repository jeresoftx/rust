use std::sync::Arc;
use std::thread;

use design_patterns_rust::patterns::rust_idiomatic::interior_mutability::rwlock_config::{
    ConfigEntry, ConfigRegistry,
};

#[test]
fn interior_mutability_rwlock_reads_configuration_from_shared_reference() {
    let registry = ConfigRegistry::new(vec![ConfigEntry::new("checkout.currency", "MXN")]);

    assert_eq!(registry.read("checkout.currency"), Some("MXN".to_string()));
    assert_eq!(registry.read("missing.key"), None);
}

#[test]
fn interior_mutability_rwlock_allows_many_concurrent_readers() {
    let registry = Arc::new(ConfigRegistry::new(vec![ConfigEntry::new(
        "feature.fast_checkout",
        "enabled",
    )]));
    let handles = (0..8)
        .map(|_| {
            let registry = Arc::clone(&registry);
            thread::spawn(move || {
                for _ in 0..50 {
                    assert_eq!(
                        registry.read("feature.fast_checkout"),
                        Some("enabled".to_string())
                    );
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().expect("reader should finish");
    }
}

#[test]
fn interior_mutability_rwlock_updates_configuration_exclusively() {
    let registry = ConfigRegistry::new(vec![ConfigEntry::new("checkout.currency", "MXN")]);

    registry.upsert(ConfigEntry::new("checkout.currency", "USD"));
    registry.upsert(ConfigEntry::new("feature.fast_checkout", "enabled"));

    assert_eq!(
        registry.snapshot(),
        vec![
            ConfigEntry::new("checkout.currency", "USD"),
            ConfigEntry::new("feature.fast_checkout", "enabled")
        ]
    );
}
