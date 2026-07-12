use design_patterns_rust::patterns::rust_idiomatic::message_passing::pipeline::{
    FulfillmentTask, OrderLine, RawOrder, run_fulfillment_pipeline,
};

#[test]
fn message_passing_pipeline_transforms_orders_through_stages() {
    let tasks = run_fulfillment_pipeline(vec![
        RawOrder::new("ORD-1", vec![OrderLine::new("keyboard", 2)]),
        RawOrder::new(
            "ORD-2",
            vec![OrderLine::new("mouse", 1), OrderLine::new("monitor", 2)],
        ),
    ]);

    assert_eq!(
        tasks,
        vec![
            FulfillmentTask::new("ORD-1", 2, "standard"),
            FulfillmentTask::new("ORD-2", 3, "standard")
        ]
    );
}

#[test]
fn message_passing_pipeline_filters_invalid_orders() {
    let tasks = run_fulfillment_pipeline(vec![
        RawOrder::new("ORD-EMPTY", vec![]),
        RawOrder::new("ORD-ZERO", vec![OrderLine::new("keyboard", 0)]),
        RawOrder::new("ORD-OK", vec![OrderLine::new("notebook", 1)]),
    ]);

    assert_eq!(tasks, vec![FulfillmentTask::new("ORD-OK", 1, "standard")]);
}

#[test]
fn message_passing_pipeline_marks_large_orders_as_bulk() {
    let tasks = run_fulfillment_pipeline(vec![RawOrder::new(
        "ORD-BULK",
        vec![OrderLine::new("usb-cable", 12)],
    )]);

    assert_eq!(tasks, vec![FulfillmentTask::new("ORD-BULK", 12, "bulk")]);
}
