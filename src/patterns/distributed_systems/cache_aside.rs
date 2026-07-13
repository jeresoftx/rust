//! Cache Aside carga datos bajo demanda y mantiene el cache fuera del repositorio.
//!
//! Los ejemplos modelan lecturas cacheadas, invalidación en escritura y TTL con
//! reloj determinista.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::distributed_systems::cache_aside as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de invalidation dentro del catalogo.
pub mod invalidation;
/// Modulo del ejemplo `read_through` dentro del catalogo de patrones.
pub mod read_through;
/// Modulo del ejemplo `ttl_cache` dentro del catalogo de patrones.
pub mod ttl_cache;
