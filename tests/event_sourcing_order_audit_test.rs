use design_patterns_rust::patterns::architecture::event_sourcing::order_audit::{
    AuditEntry, OrderAuditTrail, OrderEvent, OrderStatus,
};

#[test]
fn event_sourcing_order_audit_builds_human_readable_timeline() {
    let events = vec![
        OrderEvent::created("ORD-9", "Ana", "2026-07-12T09:00:00Z"),
        OrderEvent::status_changed(
            "ORD-9",
            OrderStatus::Draft,
            OrderStatus::Paid,
            "Luis",
            "2026-07-12T09:15:00Z",
        ),
        OrderEvent::note_added("ORD-9", "address verified", "Mia", "2026-07-12T09:20:00Z"),
    ];

    let trail = OrderAuditTrail::from_events(&events);

    assert_eq!(
        trail.entries(),
        vec![
            AuditEntry::new("2026-07-12T09:00:00Z", "Ana", "created order ORD-9"),
            AuditEntry::new(
                "2026-07-12T09:15:00Z",
                "Luis",
                "changed status from draft to paid"
            ),
            AuditEntry::new(
                "2026-07-12T09:20:00Z",
                "Mia",
                "added note: address verified"
            ),
        ]
    );
}

#[test]
fn event_sourcing_order_audit_reconstructs_current_order_state() {
    let events = vec![
        OrderEvent::created("ORD-10", "Ana", "2026-07-12T09:00:00Z"),
        OrderEvent::status_changed(
            "ORD-10",
            OrderStatus::Draft,
            OrderStatus::Paid,
            "Luis",
            "2026-07-12T09:15:00Z",
        ),
        OrderEvent::status_changed(
            "ORD-10",
            OrderStatus::Paid,
            OrderStatus::Shipped,
            "Mia",
            "2026-07-12T10:00:00Z",
        ),
    ];

    let trail = OrderAuditTrail::from_events(&events);

    assert_eq!(trail.order_id(), Some("ORD-10"));
    assert_eq!(trail.current_status(), Some(OrderStatus::Shipped));
}

#[test]
fn event_sourcing_order_audit_filters_entries_by_actor() {
    let events = vec![
        OrderEvent::created("ORD-11", "Ana", "2026-07-12T09:00:00Z"),
        OrderEvent::note_added(
            "ORD-11",
            "fraud check passed",
            "Luis",
            "2026-07-12T09:30:00Z",
        ),
        OrderEvent::note_added("ORD-11", "packed", "Ana", "2026-07-12T10:00:00Z"),
    ];

    let trail = OrderAuditTrail::from_events(&events);

    assert_eq!(
        trail.entries_by_actor("Ana"),
        vec![
            AuditEntry::new("2026-07-12T09:00:00Z", "Ana", "created order ORD-11"),
            AuditEntry::new("2026-07-12T10:00:00Z", "Ana", "added note: packed"),
        ]
    );
}
