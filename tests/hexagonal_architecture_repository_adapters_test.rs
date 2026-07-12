use design_patterns_rust::patterns::architecture::hexagonal_architecture::repository_adapters::{
    adapters::{InMemoryCustomerRepository, SimulatedExternalCustomerRepository},
    application::{FindCustomer, RegisterCustomer},
    domain::{Customer, CustomerError},
};

#[test]
fn hexagonal_architecture_uses_in_memory_repository_adapter() {
    let repository =
        InMemoryCustomerRepository::with_customers(vec![Customer::new("CUST-1", "Ana", "gold")]);
    let finder = FindCustomer::new(repository);

    let customer = finder.by_id("CUST-1").expect("customer should exist");

    assert_eq!(customer.name(), "Ana");
    assert_eq!(customer.segment(), "gold");
}

#[test]
fn hexagonal_architecture_swaps_to_simulated_external_repository_adapter() {
    let repository = SimulatedExternalCustomerRepository::online(vec![Customer::new(
        "CUST-2", "Luis", "silver",
    )]);
    let finder = FindCustomer::new(repository);

    let customer = finder.by_id("CUST-2").expect("customer should exist");

    assert_eq!(customer.name(), "Luis");
    assert_eq!(customer.segment(), "silver");
}

#[test]
fn hexagonal_architecture_maps_external_repository_failure_to_domain_error() {
    let repository = SimulatedExternalCustomerRepository::offline();
    let finder = FindCustomer::new(repository);

    let error = finder.by_id("CUST-404").unwrap_err();

    assert_eq!(error, CustomerError::RepositoryUnavailable);
}

#[test]
fn hexagonal_architecture_registers_customer_through_repository_port() {
    let repository = InMemoryCustomerRepository::default();
    let mut register = RegisterCustomer::new(repository);

    let customer = register.register("CUST-3", "Mara", "platinum");

    assert_eq!(customer.name(), "Mara");
    assert_eq!(register.count(), 1);
}
