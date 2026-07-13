//! Retry with Backoff.
//!
//! El patrón reintenta operaciones fallidas esperando cada vez más entre
//! intentos, para reducir presión sobre una dependencia que puede recuperarse.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::distributed_systems::retry_with_backoff as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de deterministic jitter dentro del catalogo.
pub mod deterministic_jitter;
/// Modulo del ejemplo `http_client` dentro del catalogo de patrones.
pub mod http_client;
/// Modulo del ejemplo `transient_errors` dentro del catalogo de patrones.
pub mod transient_errors;
