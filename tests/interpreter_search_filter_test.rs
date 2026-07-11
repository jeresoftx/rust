use design_patterns_rust::patterns::gof::behavioral::interpreter::search_filter::{
    SearchFilter, SearchRecord,
};

#[test]
fn interpreter_evaluates_equality_filter_against_record() {
    let filter = SearchFilter::parse("status = paid").expect("valid filter");
    let paid = SearchRecord::new("paid", 150);
    let pending = SearchRecord::new("pending", 150);

    assert!(filter.matches(&paid));
    assert!(!filter.matches(&pending));
}

#[test]
fn interpreter_evaluates_greater_than_filter_against_record() {
    let filter = SearchFilter::parse("total > 100").expect("valid filter");
    let large = SearchRecord::new("paid", 150);
    let small = SearchRecord::new("paid", 80);

    assert!(filter.matches(&large));
    assert!(!filter.matches(&small));
}
