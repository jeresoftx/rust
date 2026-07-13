//! Layered Architecture.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::layered_architecture as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de dto entity repository dentro del catalogo.
pub mod dto_entity_repository;
/// Modulo del ejemplo `replaceable_dependencies` dentro del catalogo de patrones.
pub mod replaceable_dependencies;
/// Modulo del ejemplo `user_api` dentro del catalogo de patrones.
pub mod user_api;
