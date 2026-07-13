//! Rate Limiting controla cuántas operaciones se aceptan en una ventana.
//!
//! Los ejemplos usan relojes lógicos y contadores deterministas para modelar
//! token buckets, límites por API key y respuestas con tiempo de reintento.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::distributed_systems::rate_limiting as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de api key limit dentro del catalogo.
pub mod api_key_limit;
/// Modulo del ejemplo `retry_after_response` dentro del catalogo de patrones.
pub mod retry_after_response;
/// Modulo del ejemplo `token_bucket` dentro del catalogo de patrones.
pub mod token_bucket;
