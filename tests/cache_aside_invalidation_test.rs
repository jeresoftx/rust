use design_patterns_rust::patterns::distributed_systems::cache_aside::invalidation::{
    CatalogService, Product,
};

#[test]
fn update_invalidates_cached_product() {
    let mut service = CatalogService::with_products([Product::new("sku-1", "Old")]);

    assert_eq!(service.get("sku-1"), Some(Product::new("sku-1", "Old")));
    service.update(Product::new("sku-1", "New"));

    assert!(!service.is_cached("sku-1"));
    assert_eq!(service.get("sku-1"), Some(Product::new("sku-1", "New")));
}

#[test]
fn update_of_one_product_keeps_other_cached_values() {
    let mut service = CatalogService::with_products([
        Product::new("sku-1", "Keyboard"),
        Product::new("sku-2", "Mouse"),
    ]);

    service.get("sku-1");
    service.get("sku-2");
    service.update(Product::new("sku-1", "Keyboard Pro"));

    assert!(!service.is_cached("sku-1"));
    assert!(service.is_cached("sku-2"));
}

#[test]
fn update_writes_to_repository_even_when_value_was_not_cached() {
    let mut service = CatalogService::with_products([]);

    service.update(Product::new("sku-1", "Keyboard"));

    assert_eq!(
        service.get("sku-1"),
        Some(Product::new("sku-1", "Keyboard"))
    );
}
