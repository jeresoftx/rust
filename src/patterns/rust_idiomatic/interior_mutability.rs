//! Interior Mutability.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::rust_idiomatic::interior_mutability as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de mutex counter dentro del catalogo.
pub mod mutex_counter;
/// Modulo del ejemplo `refcell_cache` dentro del catalogo de patrones.
pub mod refcell_cache;
/// Modulo del ejemplo `rwlock_config` dentro del catalogo de patrones.
pub mod rwlock_config;
