use std::sync::{Mutex, OnceLock};

static RESOURCE_POOL: OnceLock<ResourcePool> = OnceLock::new();

/// Alias publico `ResourcePool` usado por el ejemplo.
pub type ResourcePool = Mutex<Vec<Connection>>;

#[derive(Debug, PartialEq, Eq)]
/// Tipo publico `Connection` usado por el ejemplo para expresar el dominio del patron.
pub struct Connection {
    id: u32,
}

impl Connection {
    /// Devuelve un resumen legible del estado actual.
    pub fn summary(&self) -> String {
        format!("connection={} status=checked-out", self.id)
    }
}

/// Modela la operacion `resource pool` dentro del ejemplo del patron.
pub fn resource_pool() -> &'static ResourcePool {
    RESOURCE_POOL.get_or_init(|| Mutex::new(vec![Connection { id: 2 }, Connection { id: 1 }]))
}

/// Modela la operacion `acquire connection` dentro del ejemplo del patron.
pub fn acquire_connection() -> Option<Connection> {
    let mut pool = resource_pool()
        .lock()
        .expect("resource pool lock must be available");

    pool.pop()
}

/// Modela la operacion `release connection` dentro del ejemplo del patron.
pub fn release_connection(connection: Connection) {
    let mut pool = resource_pool()
        .lock()
        .expect("resource pool lock must be available");

    pool.push(connection);
}
