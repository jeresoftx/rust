use design_patterns_rust::patterns::gof::creational::abstract_factory::database_connectors::{
    NoSqlDatabaseFactory, SqlDatabaseFactory, describe_database_stack,
};

#[test]
fn abstract_factory_builds_a_sql_database_family() {
    let factory = SqlDatabaseFactory;

    let description = describe_database_stack(&factory, "orders");

    assert_eq!(
        description,
        "connection=postgres://primary | query=SELECT * FROM orders;"
    );
}

#[test]
fn abstract_factory_builds_a_nosql_database_family() {
    let factory = NoSqlDatabaseFactory;

    let description = describe_database_stack(&factory, "orders");

    assert_eq!(
        description,
        "connection=mongodb://cluster | query=db.orders.find({})"
    );
}
