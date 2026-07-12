//! Cache Aside carga datos bajo demanda y mantiene el cache fuera del repositorio.
//!
//! Los ejemplos modelan lecturas cacheadas, invalidación en escritura y TTL con
//! reloj determinista.

pub mod invalidation;
pub mod read_through;
