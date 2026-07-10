use design_patterns_rust::catalog_families;

#[test]
fn exposes_the_three_learning_families() {
    let families = catalog_families();

    assert_eq!(families, ["gof", "rust_idiomatic", "architecture"]);
}
