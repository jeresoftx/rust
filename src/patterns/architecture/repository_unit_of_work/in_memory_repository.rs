use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Customer {
    id: String,
    name: String,
    email: String,
}

impl Customer {
    pub fn new(id: impl Into<String>, name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            email: email.into(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }
}

pub trait CustomerRepository {
    fn save(&mut self, customer: Customer);
    fn find(&self, id: &str) -> Option<Customer>;
    fn all(&self) -> Vec<Customer>;
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InMemoryCustomerRepository {
    customers: HashMap<String, Customer>,
    insertion_order: Vec<String>,
}

impl CustomerRepository for InMemoryCustomerRepository {
    fn save(&mut self, customer: Customer) {
        if !self.customers.contains_key(customer.id()) {
            self.insertion_order.push(customer.id().to_string());
        }

        self.customers.insert(customer.id().to_string(), customer);
    }

    fn find(&self, id: &str) -> Option<Customer> {
        self.customers.get(id).cloned()
    }

    fn all(&self) -> Vec<Customer> {
        self.insertion_order
            .iter()
            .filter_map(|id| self.customers.get(id).cloned())
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct CustomerService<R> {
    repository: R,
}

impl<R> CustomerService<R>
where
    R: CustomerRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn register(
        &mut self,
        id: impl Into<String>,
        name: impl Into<String>,
        email: impl Into<String>,
    ) {
        self.repository.save(Customer::new(id, name, email));
    }

    pub fn customer_email(&self, id: &str) -> Option<String> {
        self.repository.find(id).map(|customer| customer.email)
    }
}
