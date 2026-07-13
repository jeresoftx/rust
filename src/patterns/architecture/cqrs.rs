//! CQRS.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::cqrs as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de dashboard read model dentro del catalogo.
pub mod dashboard_read_model;
/// Modulo del ejemplo `inventory_commands_queries` dentro del catalogo de patrones.
pub mod inventory_commands_queries;
/// Modulo del ejemplo `write_read_sync` dentro del catalogo de patrones.
pub mod write_read_sync;
