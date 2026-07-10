use std::sync::{Mutex, OnceLock};

static RESOURCE_POOL: OnceLock<ResourcePool> = OnceLock::new();

pub type ResourcePool = Mutex<Vec<Connection>>;

#[derive(Debug, PartialEq, Eq)]
pub struct Connection {
    id: u32,
}

impl Connection {
    pub fn summary(&self) -> String {
        format!("connection={} status=checked-out", self.id)
    }
}

pub fn resource_pool() -> &'static ResourcePool {
    RESOURCE_POOL.get_or_init(|| Mutex::new(vec![Connection { id: 2 }, Connection { id: 1 }]))
}

pub fn acquire_connection() -> Option<Connection> {
    let mut pool = resource_pool()
        .lock()
        .expect("resource pool lock must be available");

    pool.pop()
}

pub fn release_connection(connection: Connection) {
    let mut pool = resource_pool()
        .lock()
        .expect("resource pool lock must be available");

    pool.push(connection);
}
