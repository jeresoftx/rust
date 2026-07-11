use design_patterns_rust::patterns::gof::behavioral::strategy::result_sorting::{
    NewestFirst, PriceLowToHigh, RelevanceFirst, ResultItem, ResultSorter,
};

fn sample_results() -> Vec<ResultItem> {
    vec![
        ResultItem::new("Basic plan", 0.82, 19, 1_720_001),
        ResultItem::new("Enterprise plan", 0.93, 299, 1_720_003),
        ResultItem::new("Starter plan", 0.76, 9, 1_720_002),
    ]
}

#[test]
fn strategy_sorts_results_by_relevance() {
    let sorter = ResultSorter::new(RelevanceFirst);

    let titles = sorter.sort_titles(sample_results());

    assert_eq!(titles, ["Enterprise plan", "Basic plan", "Starter plan"]);
}

#[test]
fn strategy_sorts_results_by_lowest_price() {
    let sorter = ResultSorter::new(PriceLowToHigh);

    let titles = sorter.sort_titles(sample_results());

    assert_eq!(titles, ["Starter plan", "Basic plan", "Enterprise plan"]);
}

#[test]
fn strategy_sorts_results_by_newest_first() {
    let sorter = ResultSorter::new(NewestFirst);

    let titles = sorter.sort_titles(sample_results());

    assert_eq!(titles, ["Enterprise plan", "Starter plan", "Basic plan"]);
}
