//! Patrones de sistemas distribuidos y resiliencia aplicados con Rust.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::distributed_systems as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de bulkhead dentro del catalogo.
pub mod bulkhead;
/// Modulo del ejemplo `cache_aside` dentro del catalogo de patrones.
pub mod cache_aside;
/// Modulo del ejemplo `circuit_breaker` dentro del catalogo de patrones.
pub mod circuit_breaker;
/// Modulo del ejemplo `health_checks_readiness` dentro del catalogo de patrones.
pub mod health_checks_readiness;
/// Modulo del ejemplo `idempotency_key` dentro del catalogo de patrones.
pub mod idempotency_key;
/// Modulo del ejemplo `leader_election` dentro del catalogo de patrones.
pub mod leader_election;
/// Modulo del ejemplo `outbox_pattern` dentro del catalogo de patrones.
pub mod outbox_pattern;
/// Modulo del ejemplo `rate_limiting` dentro del catalogo de patrones.
pub mod rate_limiting;
/// Modulo del ejemplo `retry_with_backoff` dentro del catalogo de patrones.
pub mod retry_with_backoff;
/// Modulo del ejemplo `saga_process_manager` dentro del catalogo de patrones.
pub mod saga_process_manager;
