use design_patterns_rust::patterns::architecture::repository_unit_of_work::in_memory_repository::{
    Customer, CustomerRepository, CustomerService, InMemoryCustomerRepository,
};

#[test]
fn repository_in_memory_saves_and_finds_entities_for_tests() {
    let mut repository = InMemoryCustomerRepository::default();
    let customer = Customer::new("CUS-1", "Ana", "ana@example.com");

    repository.save(customer.clone());

    assert_eq!(repository.find("CUS-1"), Some(customer));
}

#[test]
fn repository_in_memory_lets_application_service_run_without_database() {
    let repository = InMemoryCustomerRepository::default();
    let mut service = CustomerService::new(repository);

    service.register("CUS-2", "Luis", "luis@example.com");

    assert_eq!(
        service.customer_email("CUS-2"),
        Some("luis@example.com".to_string())
    );
}

#[test]
fn repository_in_memory_can_list_customers_in_insertion_order() {
    let mut repository = InMemoryCustomerRepository::default();
    repository.save(Customer::new("CUS-1", "Ana", "ana@example.com"));
    repository.save(Customer::new("CUS-2", "Luis", "luis@example.com"));

    let names = repository
        .all()
        .into_iter()
        .map(|customer| customer.name().to_string())
        .collect::<Vec<_>>();

    assert_eq!(names, vec!["Ana", "Luis"]);
}
