use design_patterns_rust::patterns::gof::structural::flyweight::product_metadata::{
    ProductMetadataCache, ProductVariant,
};

#[test]
fn flyweight_reuses_product_metadata_for_many_variants() {
    let mut cache = ProductMetadataCache::new();
    let keyboard_meta = cache.metadata("keyboard", "Mechanical keyboard");

    let black = ProductVariant::new("kb-black", "black", 10, keyboard_meta.clone());
    let white = ProductVariant::new(
        "kb-white",
        "white",
        8,
        cache.metadata("keyboard", "ignored"),
    );

    assert!(black.shares_metadata_with(&white));
    assert_eq!(
        black.summary(),
        "sku=kb-black color=black stock=10 product=keyboard description=Mechanical keyboard"
    );
}

#[test]
fn flyweight_keeps_variant_state_outside_shared_metadata() {
    let mut cache = ProductMetadataCache::new();
    let metadata = cache.metadata("mouse", "Wireless mouse");

    let variant = ProductVariant::new("mouse-blue", "blue", 4, metadata);

    assert_eq!(
        variant.summary(),
        "sku=mouse-blue color=blue stock=4 product=mouse description=Wireless mouse"
    );
}
