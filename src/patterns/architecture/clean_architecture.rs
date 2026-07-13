//! Clean Architecture.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::clean_architecture as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de framework independent rules dentro del catalogo.
pub mod framework_independent_rules;
/// Modulo del ejemplo `presenters` dentro del catalogo de patrones.
pub mod presenters;
/// Modulo del ejemplo `registration_flow` dentro del catalogo de patrones.
pub mod registration_flow;
