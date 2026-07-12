use design_patterns_rust::patterns::rust_idiomatic::typestate::authenticated_connection::Connection;

#[test]
fn typestate_executes_query_after_authentication() {
    let mut connection = Connection::connect("primary-db")
        .authenticate("svc_orders", "secret")
        .unwrap();

    let result = connection.query("select * from orders");

    assert_eq!(
        result.summary(),
        "primary-db:svc_orders -> select * from orders"
    );
    assert_eq!(connection.executed_queries(), ["select * from orders"]);
}

#[test]
fn typestate_rejects_bad_credentials_before_authenticated_state() {
    let error = Connection::connect("primary-db")
        .authenticate("svc_orders", "")
        .unwrap_err();

    assert_eq!(error, "password is required");
}

#[test]
fn typestate_can_disconnect_after_authenticated_work() {
    let disconnected = Connection::connect("analytics-db")
        .authenticate("analyst", "secret")
        .unwrap()
        .disconnect();

    assert_eq!(disconnected.database(), "analytics-db");
    assert_eq!(disconnected.state(), "disconnected");
}
