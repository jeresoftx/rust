use design_patterns_rust::patterns::rust_idiomatic::interior_mutability::refcell_cache::{
    Product, ProductCatalog,
};

#[test]
fn interior_mutability_caches_products_after_first_lookup() {
    let catalog = ProductCatalog::new(vec![Product::new("SKU-1", "Rust en acción", 4_200)]);

    let first = catalog
        .find_product("SKU-1")
        .expect("product should be loaded");
    let second = catalog
        .find_product("SKU-1")
        .expect("product should come from cache");

    assert_eq!(first.name(), "Rust en acción");
    assert_eq!(second.price_cents(), 4_200);
    assert_eq!(catalog.source_reads(), 1);
    assert_eq!(catalog.cached_skus(), vec!["SKU-1"]);
}

#[test]
fn interior_mutability_does_not_cache_missing_products() {
    let catalog = ProductCatalog::new(vec![Product::new("SKU-1", "Rust en acción", 4_200)]);

    assert!(catalog.find_product("SKU-404").is_none());
    assert!(catalog.find_product("SKU-404").is_none());

    assert_eq!(catalog.source_reads(), 2);
    assert!(catalog.cached_skus().is_empty());
}

#[test]
fn interior_mutability_can_refresh_cached_products() {
    let catalog = ProductCatalog::new(vec![Product::new("SKU-1", "Rust en acción", 4_200)]);

    catalog.find_product("SKU-1").expect("product should exist");
    catalog.replace_product(Product::new("SKU-1", "Rust práctico", 4_900));

    let refreshed = catalog
        .find_product("SKU-1")
        .expect("product should be refreshed");

    assert_eq!(refreshed.name(), "Rust práctico");
    assert_eq!(refreshed.price_cents(), 4_900);
    assert_eq!(catalog.source_reads(), 2);
}
