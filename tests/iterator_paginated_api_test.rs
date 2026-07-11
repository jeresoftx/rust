use design_patterns_rust::patterns::gof::behavioral::iterator::paginated_api::{
    ApiPageSource, PaginatedResults,
};

#[test]
fn iterator_walks_api_results_across_pages() {
    let source = ApiPageSource::new(vec![
        vec!["order-1".to_string(), "order-2".to_string()],
        vec!["order-3".to_string()],
    ]);

    let results: Vec<String> = PaginatedResults::new(source).collect();

    assert_eq!(results, vec!["order-1", "order-2", "order-3"]);
}

#[test]
fn iterator_fetches_pages_lazily() {
    let source = ApiPageSource::new(vec![vec!["first".to_string()], vec!["second".to_string()]]);
    let mut results = PaginatedResults::new(source);

    assert_eq!(results.fetched_pages(), 0);
    assert_eq!(results.next(), Some("first".to_string()));
    assert_eq!(results.fetched_pages(), 1);
    assert_eq!(results.next(), Some("second".to_string()));
    assert_eq!(results.fetched_pages(), 2);
}
