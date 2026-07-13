//! Message Passing.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::rust_idiomatic::message_passing as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de event fanout dentro del catalogo.
pub mod event_fanout;
/// Modulo del ejemplo `job_worker` dentro del catalogo de patrones.
pub mod job_worker;
/// Modulo del ejemplo `pipeline` dentro del catalogo de patrones.
pub mod pipeline;
