//! Bulkhead aísla recursos para que una falla no consuma toda la capacidad.
//!
//! Los ejemplos modelan compartimentos deterministas: pools separados,
//! límites de concurrencia simulados y aislamiento entre operaciones críticas.

pub mod provider_pools;
