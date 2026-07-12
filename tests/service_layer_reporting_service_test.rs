use design_patterns_rust::patterns::architecture::service_layer::reporting_service::{
    Customer, OrderRecord, ReportingRepository, ReportingService, SalesReportQuery, TopCustomer,
};

#[test]
fn service_layer_reporting_service_builds_summary_from_multiple_queries() {
    let repository = ReportingRepository::new(
        vec![
            Customer::new("CUS-1", "Ana"),
            Customer::new("CUS-2", "Luis"),
        ],
        vec![
            OrderRecord::new("ORD-1", "CUS-1", 12_000, "paid"),
            OrderRecord::new("ORD-2", "CUS-1", 8_000, "refunded"),
            OrderRecord::new("ORD-3", "CUS-2", 5_000, "paid"),
        ],
    );
    let service = ReportingService::new(repository);

    let report = service.sales_report(SalesReportQuery::paid_only());

    assert_eq!(report.total_revenue_cents(), 17_000);
    assert_eq!(report.order_count(), 2);
    assert_eq!(
        report.top_customer(),
        Some(TopCustomer::new("CUS-1", "Ana", 12_000))
    );
}

#[test]
fn service_layer_reporting_service_can_include_refunds_when_requested() {
    let repository = ReportingRepository::new(
        vec![
            Customer::new("CUS-1", "Ana"),
            Customer::new("CUS-2", "Luis"),
        ],
        vec![
            OrderRecord::new("ORD-1", "CUS-1", 12_000, "paid"),
            OrderRecord::new("ORD-2", "CUS-1", 8_000, "refunded"),
            OrderRecord::new("ORD-3", "CUS-2", 5_000, "paid"),
        ],
    );
    let service = ReportingService::new(repository);

    let report = service.sales_report(SalesReportQuery::include_all());

    assert_eq!(report.total_revenue_cents(), 25_000);
    assert_eq!(report.order_count(), 3);
    assert_eq!(
        report.top_customer(),
        Some(TopCustomer::new("CUS-1", "Ana", 20_000))
    );
}

#[test]
fn service_layer_reporting_service_returns_empty_report_for_no_orders() {
    let repository = ReportingRepository::new(vec![Customer::new("CUS-1", "Ana")], vec![]);
    let service = ReportingService::new(repository);

    let report = service.sales_report(SalesReportQuery::paid_only());

    assert_eq!(report.total_revenue_cents(), 0);
    assert_eq!(report.order_count(), 0);
    assert_eq!(report.top_customer(), None);
}
