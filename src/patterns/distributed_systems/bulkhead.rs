//! Bulkhead aísla recursos para que una falla no consuma toda la capacidad.
//!
//! Los ejemplos modelan compartimentos deterministas: pools separados,
//! límites de concurrencia simulados y aislamiento entre operaciones críticas.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::distributed_systems::bulkhead as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de critical isolation dentro del catalogo.
pub mod critical_isolation;
/// Modulo del ejemplo `provider_pools` dentro del catalogo de patrones.
pub mod provider_pools;
/// Modulo del ejemplo `resource_concurrency_limit` dentro del catalogo de patrones.
pub mod resource_concurrency_limit;
