use design_patterns_rust::patterns::distributed_systems::cache_aside::read_through::{
    CacheAsideService, Product, ProductRepository,
};

#[test]
fn cache_miss_loads_from_repository_and_stores_value() {
    let repository = ProductRepository::with_products([Product::new("sku-1", "Keyboard")]);
    let mut service = CacheAsideService::new(repository);

    let product = service.get_product("sku-1");

    assert_eq!(product, Some(Product::new("sku-1", "Keyboard")));
    assert_eq!(service.repository_reads(), 1);
    assert_eq!(service.cached_keys(), vec!["sku-1".to_string()]);
}

#[test]
fn cache_hit_does_not_read_repository_again() {
    let repository = ProductRepository::with_products([Product::new("sku-1", "Keyboard")]);
    let mut service = CacheAsideService::new(repository);

    service.get_product("sku-1");
    service.get_product("sku-1");

    assert_eq!(service.repository_reads(), 1);
}

#[test]
fn missing_repository_value_is_not_cached() {
    let repository = ProductRepository::with_products([]);
    let mut service = CacheAsideService::new(repository);

    assert_eq!(service.get_product("missing"), None);
    assert!(service.cached_keys().is_empty());
}
