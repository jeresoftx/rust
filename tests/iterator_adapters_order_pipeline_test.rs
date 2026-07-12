use design_patterns_rust::patterns::rust_idiomatic::iterator_adapters::order_pipeline::{
    Order, OrderStatus, billable_order_lines,
};

#[test]
fn iterator_adapters_filter_and_transform_billable_orders() {
    let orders = vec![
        Order::new("ORD-1", "  Ana  ", 12_000, OrderStatus::Paid),
        Order::new("ORD-2", "Luis", 8_500, OrderStatus::Pending),
        Order::new("ORD-3", "Marta", 0, OrderStatus::Paid),
        Order::new("ORD-4", "Sofía", 2_500, OrderStatus::Paid),
    ];

    let lines = billable_order_lines(&orders);

    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].summary(), "ORD-1 Ana $120.00");
    assert_eq!(lines[1].summary(), "ORD-4 Sofía $25.00");
}

#[test]
fn iterator_adapters_keeps_invoice_line_totals_in_cents() {
    let orders = vec![
        Order::new("ORD-10", "Cobranza", 1_999, OrderStatus::Paid),
        Order::new("ORD-11", "Soporte", 4_001, OrderStatus::Cancelled),
    ];

    let lines = billable_order_lines(&orders);

    assert_eq!(lines[0].order_id(), "ORD-10");
    assert_eq!(lines[0].total_cents(), 1_999);
}

#[test]
fn iterator_adapters_returns_empty_lines_when_no_order_is_billable() {
    let orders = vec![
        Order::new("ORD-20", "Ventas", 8_000, OrderStatus::Pending),
        Order::new("ORD-21", "Operaciones", 0, OrderStatus::Paid),
    ];

    assert!(billable_order_lines(&orders).is_empty());
}
