//! Idempotency Key evita duplicar efectos cuando una petición se repite.
//!
//! Los ejemplos modelan pagos, respuestas cacheadas y conflictos de payload
//! con almacenamiento en memoria determinista.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::distributed_systems::idempotency_key as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de cached responses dentro del catalogo.
pub mod cached_responses;
/// Modulo del ejemplo `payload_conflict` dentro del catalogo de patrones.
pub mod payload_conflict;
/// Modulo del ejemplo `payment_deduplication` dentro del catalogo de patrones.
pub mod payment_deduplication;
