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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    id: String,
    customer_id: String,
    total_cents: u32,
}

impl Order {
    pub fn new(id: impl Into<String>, customer_id: impl Into<String>, total_cents: u32) -> Self {
        Self {
            id: id.into(),
            customer_id: customer_id.into(),
            total_cents,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn total_cents(&self) -> u32 {
        self.total_cents
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct UnitOfWorkStore {
    customers: HashMap<String, Customer>,
    orders: HashMap<String, Order>,
}

impl UnitOfWorkStore {
    pub fn find_customer(&self, id: &str) -> Option<Customer> {
        self.customers.get(id).cloned()
    }

    pub fn find_order(&self, id: &str) -> Option<Order> {
        self.orders.get(id).cloned()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PendingCustomerRepository {
    customers: HashMap<String, Customer>,
}

impl PendingCustomerRepository {
    pub fn save(&mut self, customer: Customer) {
        self.customers.insert(customer.id().to_string(), customer);
    }

    pub fn find(&self, id: &str) -> Option<Customer> {
        self.customers.get(id).cloned()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PendingOrderRepository {
    orders: HashMap<String, Order>,
}

impl PendingOrderRepository {
    pub fn save(&mut self, order: Order) {
        self.orders.insert(order.id().to_string(), order);
    }

    pub fn find(&self, id: &str) -> Option<Order> {
        self.orders.get(id).cloned()
    }
}

#[derive(Debug)]
pub struct UnitOfWork<'a> {
    store: &'a mut UnitOfWorkStore,
    customers: PendingCustomerRepository,
    orders: PendingOrderRepository,
}

impl<'a> UnitOfWork<'a> {
    pub fn new(store: &'a mut UnitOfWorkStore) -> Self {
        Self {
            store,
            customers: PendingCustomerRepository::default(),
            orders: PendingOrderRepository::default(),
        }
    }

    pub fn customers(&mut self) -> &mut PendingCustomerRepository {
        &mut self.customers
    }

    pub fn orders(&mut self) -> &mut PendingOrderRepository {
        &mut self.orders
    }

    pub fn commit(self) {
        for (id, customer) in self.customers.customers {
            self.store.customers.insert(id, customer);
        }

        for (id, order) in self.orders.orders {
            self.store.orders.insert(id, order);
        }
    }

    pub fn rollback(self) {}
}
