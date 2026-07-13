use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Customer` usado por el ejemplo para expresar el dominio del patron.
pub struct Customer {
    id: String,
    name: String,
    email: String,
}

impl Customer {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>, name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            email: email.into(),
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

    /// Modela la operacion `email` dentro del ejemplo del patron.
    pub fn email(&self) -> &str {
        &self.email
    }
}

/// Contrato publico `CustomerRepository` que desacopla las piezas del ejemplo.
pub trait CustomerRepository {
    /// Operacion `save` definida por la abstraccion del ejemplo.
    fn save(&mut self, customer: Customer);
    /// Operacion `find` definida por la abstraccion del ejemplo.
    fn find(&self, id: &str) -> Option<Customer>;
    /// Operacion `all` definida por la abstraccion del ejemplo.
    fn all(&self) -> Vec<Customer>;
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// Tipo publico `InMemoryCustomerRepository` usado por el ejemplo para expresar el dominio del patron.
pub struct InMemoryCustomerRepository {
    customers: HashMap<String, Customer>,
    insertion_order: Vec<String>,
}

impl CustomerRepository for InMemoryCustomerRepository {
    /// Operacion `save` definida por la abstraccion del ejemplo.
    fn save(&mut self, customer: Customer) {
        if !self.customers.contains_key(customer.id()) {
            self.insertion_order.push(customer.id().to_string());
        }

        self.customers.insert(customer.id().to_string(), customer);
    }

    /// Operacion `find` definida por la abstraccion del ejemplo.
    fn find(&self, id: &str) -> Option<Customer> {
        self.customers.get(id).cloned()
    }

    /// Operacion `all` definida por la abstraccion del ejemplo.
    fn all(&self) -> Vec<Customer> {
        self.insertion_order
            .iter()
            .filter_map(|id| self.customers.get(id).cloned())
            .collect()
    }
}

#[derive(Debug, Clone)]
/// Tipo publico `CustomerService` usado por el ejemplo para expresar el dominio del patron.
pub struct CustomerService<R> {
    repository: R,
}

impl<R> CustomerService<R>
where
    R: CustomerRepository,
{
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Modela la operacion `register` dentro del ejemplo del patron.
    pub fn register(
        &mut self,
        id: impl Into<String>,
        name: impl Into<String>,
        email: impl Into<String>,
    ) {
        self.repository.save(Customer::new(id, name, email));
    }

    /// Modela la operacion `customer email` dentro del ejemplo del patron.
    pub fn customer_email(&self, id: &str) -> Option<String> {
        self.repository.find(id).map(|customer| customer.email)
    }
}
