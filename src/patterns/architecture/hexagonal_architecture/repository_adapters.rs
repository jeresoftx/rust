//! Repository Adapters.
//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::hexagonal_architecture::repository_adapters as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Modulo del ejemplo `domain` dentro del catalogo de patrones.
pub mod domain {
    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `Customer` usado por el ejemplo para expresar el dominio del patron.
    pub struct Customer {
        id: String,
        name: String,
        segment: String,
    }

    impl Customer {
        /// Crea una instancia valida para el ejemplo del patron.
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

        /// Modela la operacion `id` dentro del ejemplo del patron.
        pub fn id(&self) -> &str {
            &self.id
        }

        /// Modela la operacion `name` dentro del ejemplo del patron.
        pub fn name(&self) -> &str {
            &self.name
        }

        /// Modela la operacion `segment` dentro del ejemplo del patron.
        pub fn segment(&self) -> &str {
            &self.segment
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Conjunto de estados o errores publicos de `CustomerError` dentro del ejemplo.
    pub enum CustomerError {
        /// Variante `NotFound` del estado o error del ejemplo.
        NotFound {
            /// Valor publico `id` asociado a la variante `NotFound`.
            id: String,
        },
        /// Variante `RepositoryUnavailable` del estado o error del ejemplo.
        RepositoryUnavailable,
    }
}

/// Modulo del ejemplo `ports` dentro del catalogo de patrones.
pub mod ports {
    use super::domain::{Customer, CustomerError};

    /// Contrato publico `CustomerRepository` que desacopla las piezas del ejemplo.
    pub trait CustomerRepository {
        /// Operacion `find by id` definida por la abstraccion del ejemplo.
        fn find_by_id(&self, id: &str) -> Result<Customer, CustomerError>;
        /// Operacion `save` definida por la abstraccion del ejemplo.
        fn save(&mut self, customer: Customer);
        /// Operacion `count` definida por la abstraccion del ejemplo.
        fn count(&self) -> usize;
    }
}

/// Modulo del ejemplo `application` dentro del catalogo de patrones.
pub mod application {
    use super::domain::{Customer, CustomerError};
    use super::ports::CustomerRepository;

    #[derive(Debug, Clone)]
    /// Tipo publico `FindCustomer` usado por el ejemplo para expresar el dominio del patron.
    pub struct FindCustomer<R> {
        repository: R,
    }

    impl<R> FindCustomer<R>
    where
        R: CustomerRepository,
    {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(repository: R) -> Self {
            Self { repository }
        }

        /// Modela la operacion `by id` dentro del ejemplo del patron.
        pub fn by_id(&self, id: &str) -> Result<Customer, CustomerError> {
            self.repository.find_by_id(id)
        }
    }

    #[derive(Debug, Clone)]
    /// Tipo publico `RegisterCustomer` usado por el ejemplo para expresar el dominio del patron.
    pub struct RegisterCustomer<R> {
        repository: R,
    }

    impl<R> RegisterCustomer<R>
    where
        R: CustomerRepository,
    {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(repository: R) -> Self {
            Self { repository }
        }

        /// Modela la operacion `register` dentro del ejemplo del patron.
        pub fn register(&mut self, id: &str, name: &str, segment: &str) -> Customer {
            let customer = Customer::new(id, name, segment);
            self.repository.save(customer.clone());
            customer
        }

        /// Modela la operacion `count` dentro del ejemplo del patron.
        pub fn count(&self) -> usize {
            self.repository.count()
        }
    }
}

/// Modulo del ejemplo `adapters` dentro del catalogo de patrones.
pub mod adapters {
    use std::collections::HashMap;

    use super::domain::{Customer, CustomerError};
    use super::ports::CustomerRepository;

    #[derive(Debug, Clone, Default)]
    /// Tipo publico `InMemoryCustomerRepository` usado por el ejemplo para expresar el dominio del patron.
    pub struct InMemoryCustomerRepository {
        customers: HashMap<String, Customer>,
    }

    impl InMemoryCustomerRepository {
        /// Modela la operacion `with customers` dentro del ejemplo del patron.
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
        /// Operacion `find by id` definida por la abstraccion del ejemplo.
        fn find_by_id(&self, id: &str) -> Result<Customer, CustomerError> {
            self.customers
                .get(id)
                .cloned()
                .ok_or_else(|| CustomerError::NotFound { id: id.to_string() })
        }

        /// Operacion `save` definida por la abstraccion del ejemplo.
        fn save(&mut self, customer: Customer) {
            self.customers.insert(customer.id().to_string(), customer);
        }

        /// Operacion `count` definida por la abstraccion del ejemplo.
        fn count(&self) -> usize {
            self.customers.len()
        }
    }

    #[derive(Debug, Clone, Default)]
    /// Tipo publico `SimulatedExternalCustomerRepository` usado por el ejemplo para expresar el dominio del patron.
    pub struct SimulatedExternalCustomerRepository {
        online: bool,
        customers: HashMap<String, Customer>,
    }

    impl SimulatedExternalCustomerRepository {
        /// Modela la operacion `online` dentro del ejemplo del patron.
        pub fn online(customers: Vec<Customer>) -> Self {
            Self {
                online: true,
                customers: customers
                    .into_iter()
                    .map(|customer| (customer.id().to_string(), customer))
                    .collect(),
            }
        }

        /// Modela la operacion `offline` dentro del ejemplo del patron.
        pub fn offline() -> Self {
            Self {
                online: false,
                customers: HashMap::new(),
            }
        }
    }

    impl CustomerRepository for SimulatedExternalCustomerRepository {
        /// Operacion `find by id` definida por la abstraccion del ejemplo.
        fn find_by_id(&self, id: &str) -> Result<Customer, CustomerError> {
            if !self.online {
                return Err(CustomerError::RepositoryUnavailable);
            }

            self.customers
                .get(id)
                .cloned()
                .ok_or_else(|| CustomerError::NotFound { id: id.to_string() })
        }

        /// Operacion `save` definida por la abstraccion del ejemplo.
        fn save(&mut self, customer: Customer) {
            self.customers.insert(customer.id().to_string(), customer);
        }

        /// Operacion `count` definida por la abstraccion del ejemplo.
        fn count(&self) -> usize {
            self.customers.len()
        }
    }
}
