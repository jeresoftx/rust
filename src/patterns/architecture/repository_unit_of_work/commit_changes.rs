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
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Order` usado por el ejemplo para expresar el dominio del patron.
pub struct Order {
    id: String,
    customer_id: String,
    total_cents: u32,
}

impl Order {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>, customer_id: impl Into<String>, total_cents: u32) -> Self {
        Self {
            id: id.into(),
            customer_id: customer_id.into(),
            total_cents,
        }
    }

    /// Modela la operacion `id` dentro del ejemplo del patron.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Modela la operacion `total cents` dentro del ejemplo del patron.
    pub fn total_cents(&self) -> u32 {
        self.total_cents
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// Tipo publico `UnitOfWorkStore` usado por el ejemplo para expresar el dominio del patron.
pub struct UnitOfWorkStore {
    customers: HashMap<String, Customer>,
    orders: HashMap<String, Order>,
}

impl UnitOfWorkStore {
    /// Modela la operacion `find customer` dentro del ejemplo del patron.
    pub fn find_customer(&self, id: &str) -> Option<Customer> {
        self.customers.get(id).cloned()
    }

    /// Modela la operacion `find order` dentro del ejemplo del patron.
    pub fn find_order(&self, id: &str) -> Option<Order> {
        self.orders.get(id).cloned()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// Tipo publico `PendingCustomerRepository` usado por el ejemplo para expresar el dominio del patron.
pub struct PendingCustomerRepository {
    customers: HashMap<String, Customer>,
}

impl PendingCustomerRepository {
    /// Modela la operacion `save` dentro del ejemplo del patron.
    pub fn save(&mut self, customer: Customer) {
        self.customers.insert(customer.id().to_string(), customer);
    }

    /// Modela la operacion `find` dentro del ejemplo del patron.
    pub fn find(&self, id: &str) -> Option<Customer> {
        self.customers.get(id).cloned()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// Tipo publico `PendingOrderRepository` usado por el ejemplo para expresar el dominio del patron.
pub struct PendingOrderRepository {
    orders: HashMap<String, Order>,
}

impl PendingOrderRepository {
    /// Modela la operacion `save` dentro del ejemplo del patron.
    pub fn save(&mut self, order: Order) {
        self.orders.insert(order.id().to_string(), order);
    }

    /// Modela la operacion `find` dentro del ejemplo del patron.
    pub fn find(&self, id: &str) -> Option<Order> {
        self.orders.get(id).cloned()
    }
}

#[derive(Debug)]
/// Tipo publico `UnitOfWork` usado por el ejemplo para expresar el dominio del patron.
pub struct UnitOfWork<'a> {
    store: &'a mut UnitOfWorkStore,
    customers: PendingCustomerRepository,
    orders: PendingOrderRepository,
}

impl<'a> UnitOfWork<'a> {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(store: &'a mut UnitOfWorkStore) -> Self {
        Self {
            store,
            customers: PendingCustomerRepository::default(),
            orders: PendingOrderRepository::default(),
        }
    }

    /// Modela la operacion `customers` dentro del ejemplo del patron.
    pub fn customers(&mut self) -> &mut PendingCustomerRepository {
        &mut self.customers
    }

    /// Modela la operacion `orders` dentro del ejemplo del patron.
    pub fn orders(&mut self) -> &mut PendingOrderRepository {
        &mut self.orders
    }

    /// Modela la operacion `commit` dentro del ejemplo del patron.
    pub fn commit(self) {
        for (id, customer) in self.customers.customers {
            self.store.customers.insert(id, customer);
        }

        for (id, order) in self.orders.orders {
            self.store.orders.insert(id, order);
        }
    }

    /// Modela la operacion `rollback` dentro del ejemplo del patron.
    pub fn rollback(self) {}
}
