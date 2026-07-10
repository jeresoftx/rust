use design_patterns_rust::patterns::gof::creational::builder::search_query::{
    SearchQuery, SearchQueryError,
};

#[test]
fn builder_creates_search_query_with_optional_filters() {
    let query = SearchQuery::builder("hotels")
        .category("lodging")
        .min_score(80)
        .tag("pool")
        .tag("wifi")
        .sort_by("rating")
        .build()
        .expect("valid query");

    assert_eq!(
        query.to_query_string(),
        "q=hotels&category=lodging&min_score=80&tags=pool,wifi&sort=rating"
    );
}

#[test]
fn builder_rejects_blank_search_terms() {
    let error = SearchQuery::builder("  ")
        .build()
        .expect_err("blank terms should be rejected");

    assert_eq!(error, SearchQueryError::TermRequired);
}
