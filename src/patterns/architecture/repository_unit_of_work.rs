//! Repository and Unit of Work.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::repository_unit_of_work as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de commit changes dentro del catalogo.
pub mod commit_changes;
/// Modulo del ejemplo `in_memory_repository` dentro del catalogo de patrones.
pub mod in_memory_repository;
/// Modulo del ejemplo `transaction_rollback` dentro del catalogo de patrones.
pub mod transaction_rollback;
