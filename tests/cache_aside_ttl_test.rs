use design_patterns_rust::patterns::distributed_systems::cache_aside::ttl_cache::{
    Product, TtlCacheAsideService,
};

#[test]
fn cache_hit_before_ttl_expires_avoids_repository_read() {
    let mut service = TtlCacheAsideService::with_products([Product::new("sku-1", "Keyboard")], 10);

    service.get_at("sku-1", 0);
    service.get_at("sku-1", 9);

    assert_eq!(service.repository_reads(), 1);
}

#[test]
fn expired_entry_is_reloaded_from_repository() {
    let mut service = TtlCacheAsideService::with_products([Product::new("sku-1", "Keyboard")], 10);

    service.get_at("sku-1", 0);
    service.get_at("sku-1", 10);

    assert_eq!(service.repository_reads(), 2);
}

#[test]
fn ttl_cache_stores_expiration_tick() {
    let mut service = TtlCacheAsideService::with_products([Product::new("sku-1", "Keyboard")], 10);

    service.get_at("sku-1", 5);

    assert_eq!(service.expires_at("sku-1"), Some(15));
}
