pub mod domain {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Customer {
        id: String,
        name: String,
        segment: String,
    }

    impl Customer {
        pub fn new(
            id: impl Into<String>,
            name: impl Into<String>,
            segment: impl Into<String>,
        ) -> Self {
            Self {
                id: id.into(),
                name: name.into(),
                segment: segment.into(),
            }
        }

        pub fn id(&self) -> &str {
            &self.id
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn segment(&self) -> &str {
            &self.segment
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum CustomerError {
        NotFound { id: String },
        RepositoryUnavailable,
    }
}

pub mod ports {
    use super::domain::{Customer, CustomerError};

    pub trait CustomerRepository {
        fn find_by_id(&self, id: &str) -> Result<Customer, CustomerError>;
        fn save(&mut self, customer: Customer);
        fn count(&self) -> usize;
    }
}

pub mod application {
    use super::domain::{Customer, CustomerError};
    use super::ports::CustomerRepository;

    #[derive(Debug, Clone)]
    pub struct FindCustomer<R> {
        repository: R,
    }

    impl<R> FindCustomer<R>
    where
        R: CustomerRepository,
    {
        pub fn new(repository: R) -> Self {
            Self { repository }
        }

        pub fn by_id(&self, id: &str) -> Result<Customer, CustomerError> {
            self.repository.find_by_id(id)
        }
    }

    #[derive(Debug, Clone)]
    pub struct RegisterCustomer<R> {
        repository: R,
    }

    impl<R> RegisterCustomer<R>
    where
        R: CustomerRepository,
    {
        pub fn new(repository: R) -> Self {
            Self { repository }
        }

        pub fn register(&mut self, id: &str, name: &str, segment: &str) -> Customer {
            let customer = Customer::new(id, name, segment);
            self.repository.save(customer.clone());
            customer
        }

        pub fn count(&self) -> usize {
            self.repository.count()
        }
    }
}

pub mod adapters {
    use std::collections::HashMap;

    use super::domain::{Customer, CustomerError};
    use super::ports::CustomerRepository;

    #[derive(Debug, Clone, Default)]
    pub struct InMemoryCustomerRepository {
        customers: HashMap<String, Customer>,
    }

    impl InMemoryCustomerRepository {
        pub fn with_customers(customers: Vec<Customer>) -> Self {
            Self {
                customers: customers
                    .into_iter()
                    .map(|customer| (customer.id().to_string(), customer))
                    .collect(),
            }
        }
    }

    impl CustomerRepository for InMemoryCustomerRepository {
        fn find_by_id(&self, id: &str) -> Result<Customer, CustomerError> {
            self.customers
                .get(id)
                .cloned()
                .ok_or_else(|| CustomerError::NotFound { id: id.to_string() })
        }

        fn save(&mut self, customer: Customer) {
            self.customers.insert(customer.id().to_string(), customer);
        }

        fn count(&self) -> usize {
            self.customers.len()
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct SimulatedExternalCustomerRepository {
        online: bool,
        customers: HashMap<String, Customer>,
    }

    impl SimulatedExternalCustomerRepository {
        pub fn online(customers: Vec<Customer>) -> Self {
            Self {
                online: true,
                customers: customers
                    .into_iter()
                    .map(|customer| (customer.id().to_string(), customer))
                    .collect(),
            }
        }

        pub fn offline() -> Self {
            Self {
                online: false,
                customers: HashMap::new(),
            }
        }
    }

    impl CustomerRepository for SimulatedExternalCustomerRepository {
        fn find_by_id(&self, id: &str) -> Result<Customer, CustomerError> {
            if !self.online {
                return Err(CustomerError::RepositoryUnavailable);
            }

            self.customers
                .get(id)
                .cloned()
                .ok_or_else(|| CustomerError::NotFound { id: id.to_string() })
        }

        fn save(&mut self, customer: Customer) {
            self.customers.insert(customer.id().to_string(), customer);
        }

        fn count(&self) -> usize {
            self.customers.len()
        }
    }
}
