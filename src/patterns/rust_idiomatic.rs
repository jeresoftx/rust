//! Patrones idiomáticos de Rust.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::rust_idiomatic as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de actor like workers dentro del catalogo.
pub mod actor_like_workers;
/// Modulo del ejemplo `error_handling_result` dentro del catalogo de patrones.
pub mod error_handling_result;
/// Modulo del ejemplo `extension_trait` dentro del catalogo de patrones.
pub mod extension_trait;
/// Modulo del ejemplo `interior_mutability` dentro del catalogo de patrones.
pub mod interior_mutability;
/// Modulo del ejemplo `iterator_adapters` dentro del catalogo de patrones.
pub mod iterator_adapters;
/// Modulo del ejemplo `message_passing` dentro del catalogo de patrones.
pub mod message_passing;
/// Modulo del ejemplo `newtype` dentro del catalogo de patrones.
pub mod newtype;
/// Modulo del ejemplo `raii` dentro del catalogo de patrones.
pub mod raii;
/// Modulo del ejemplo `typestate` dentro del catalogo de patrones.
pub mod typestate;
