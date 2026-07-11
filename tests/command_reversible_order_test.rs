use design_patterns_rust::patterns::gof::behavioral::command::reversible_order::{
    AddNoteCommand, ApplyDiscountCommand, Order, OrderHistory,
};

#[test]
fn command_executes_reversible_order_actions() {
    let mut order = Order::new("ORD-1", 100);
    let mut history = OrderHistory::new();

    history.execute(Box::new(ApplyDiscountCommand::new(15)), &mut order);
    history.execute(Box::new(AddNoteCommand::new("VIP customer")), &mut order);

    assert_eq!(order.summary(), "ORD-1 total=85 notes=VIP customer");
}

#[test]
fn command_undoes_last_order_action() {
    let mut order = Order::new("ORD-2", 200);
    let mut history = OrderHistory::new();

    history.execute(Box::new(ApplyDiscountCommand::new(30)), &mut order);
    history.execute(Box::new(AddNoteCommand::new("manual review")), &mut order);
    history.undo_last(&mut order);

    assert_eq!(order.summary(), "ORD-2 total=170 notes=");
}
