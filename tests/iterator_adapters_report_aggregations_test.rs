use design_patterns_rust::patterns::rust_idiomatic::iterator_adapters::report_aggregations::{
    SaleEvent, SaleStatus, summarize_sales,
};

#[test]
fn iterator_adapters_aggregates_report_metrics_with_fold() {
    let events = vec![
        SaleEvent::new("web", 10_000, SaleStatus::Completed),
        SaleEvent::new("store", 2_500, SaleStatus::Refunded),
        SaleEvent::new("web", 4_000, SaleStatus::Completed),
        SaleEvent::new("marketplace", 9_999, SaleStatus::Pending),
    ];

    let summary = summarize_sales(&events);

    assert_eq!(summary.completed_sales(), 2);
    assert_eq!(summary.refunds(), 1);
    assert_eq!(summary.net_revenue_cents(), 11_500);
    assert_eq!(summary.average_completed_ticket_cents(), 7_000);
    assert_eq!(summary.largest_completed_sale_cents(), Some(10_000));
    assert_eq!(summary.summary(), "2 sales, 1 refunds, net $115.00");
}

#[test]
fn iterator_adapters_returns_zero_summary_for_empty_reports() {
    let summary = summarize_sales(&[]);

    assert_eq!(summary.completed_sales(), 0);
    assert_eq!(summary.refunds(), 0);
    assert_eq!(summary.net_revenue_cents(), 0);
    assert_eq!(summary.average_completed_ticket_cents(), 0);
    assert_eq!(summary.largest_completed_sale_cents(), None);
}

#[test]
fn iterator_adapters_pending_sales_do_not_affect_financial_totals() {
    let events = vec![
        SaleEvent::new("web", 30_000, SaleStatus::Pending),
        SaleEvent::new("store", 5_500, SaleStatus::Completed),
    ];

    let summary = summarize_sales(&events);

    assert_eq!(summary.completed_sales(), 1);
    assert_eq!(summary.net_revenue_cents(), 5_500);
    assert_eq!(summary.average_completed_ticket_cents(), 5_500);
}
