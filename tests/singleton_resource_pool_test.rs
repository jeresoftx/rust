use design_patterns_rust::patterns::gof::creational::singleton::resource_pool::{
    acquire_connection, release_connection, resource_pool,
};

#[test]
fn singleton_acquires_and_releases_shared_connections() {
    let connection = acquire_connection().expect("connection should be available");

    assert_eq!(connection.summary(), "connection=1 status=checked-out");

    release_connection(connection);

    let reused = acquire_connection().expect("released connection should be reusable");
    assert_eq!(reused.summary(), "connection=1 status=checked-out");
}

#[test]
fn singleton_returns_the_same_resource_pool_instance() {
    let first = resource_pool();
    let second = resource_pool();

    assert!(std::ptr::eq(first, second));
}
