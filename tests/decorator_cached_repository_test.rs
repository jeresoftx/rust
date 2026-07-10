use design_patterns_rust::patterns::gof::structural::decorator::cached_repository::{
    CachedProductRepository, InMemoryProductRepository, ProductRepository, read_product_twice,
};

#[test]
fn decorator_caches_repository_reads_without_changing_interface() {
    let repository = CachedProductRepository::new(InMemoryProductRepository::new(vec![
        ("sku-1", "Keyboard"),
        ("sku-2", "Mouse"),
    ]));

    assert_eq!(
        read_product_twice(&repository, "sku-1"),
        "first=Keyboard second=Keyboard lookups=1"
    );
}

#[test]
fn decorator_keeps_base_repository_usable_without_cache() {
    let repository = InMemoryProductRepository::new(vec![("sku-1", "Keyboard")]);

    assert_eq!(repository.find_name("sku-1"), Some("Keyboard".to_string()));
    assert_eq!(repository.find_name("missing"), None);
}
