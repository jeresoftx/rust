use design_patterns_rust::patterns::rust_idiomatic::extension_trait::collection_pagination::CollectionPaginationExt;

#[test]
fn extension_trait_paginates_collection_results() {
    let orders = ["ORD-1", "ORD-2", "ORD-3", "ORD-4", "ORD-5"];

    let page = orders.paginate(2, 2);

    assert_eq!(page.items(), ["ORD-3", "ORD-4"]);
    assert_eq!(page.page(), 2);
    assert_eq!(page.per_page(), 2);
    assert_eq!(page.total_items(), 5);
    assert_eq!(page.total_pages(), 3);
    assert!(page.has_next());
}

#[test]
fn extension_trait_returns_empty_page_when_requested_page_is_out_of_range() {
    let users = ["Ana", "Luis", "Marta"];

    let page = users.paginate(4, 2);

    assert!(page.items().is_empty());
    assert_eq!(page.total_pages(), 2);
    assert!(!page.has_next());
}

#[test]
fn extension_trait_treats_zero_page_size_as_empty_page() {
    let products = ["SKU-1", "SKU-2"];

    let page = products.paginate(1, 0);

    assert!(page.items().is_empty());
    assert_eq!(page.total_pages(), 0);
    assert_eq!(page.total_items(), 2);
}
