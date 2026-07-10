use design_patterns_rust::patterns::gof::creational::singleton::metrics_registry::{
    increment_metric, metrics_registry, read_metric,
};

#[test]
fn singleton_counts_metrics_in_a_shared_registry() {
    increment_metric("checkout.created");
    increment_metric("checkout.created");
    increment_metric("checkout.paid");

    assert_eq!(read_metric("checkout.created"), 2);
    assert_eq!(read_metric("checkout.paid"), 1);
}

#[test]
fn singleton_returns_the_same_metrics_registry_instance() {
    let first = metrics_registry();
    let second = metrics_registry();

    assert!(std::ptr::eq(first, second));
}
