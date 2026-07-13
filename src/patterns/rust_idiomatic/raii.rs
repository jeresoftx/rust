//! RAII.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::rust_idiomatic::raii as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de lock guard dentro del catalogo.
pub mod lock_guard;
/// Modulo del ejemplo `temporary_file_cleanup` dentro del catalogo de patrones.
pub mod temporary_file_cleanup;
/// Modulo del ejemplo `transaction_rollback` dentro del catalogo de patrones.
pub mod transaction_rollback;
