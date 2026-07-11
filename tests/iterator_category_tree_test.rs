use design_patterns_rust::patterns::gof::behavioral::iterator::category_tree::Category;

#[test]
fn iterator_traverses_category_tree_depth_first() {
    let tree = Category::new("Store")
        .with_child(Category::new("Books").with_child(Category::new("Rust")))
        .with_child(Category::new("Games"));

    let names: Vec<String> = tree.depth_first().map(str::to_string).collect();

    assert_eq!(names, vec!["Store", "Books", "Rust", "Games"]);
}

#[test]
fn iterator_can_start_from_nested_category() {
    let category = Category::new("Electronics").with_child(Category::new("Laptops"));

    let names: Vec<String> = category.depth_first().map(str::to_string).collect();

    assert_eq!(names, vec!["Electronics", "Laptops"]);
}
