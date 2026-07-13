//! Facade.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::gof::structural::facade as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de checkout dentro del catalogo.
pub mod checkout;
/// Modulo del ejemplo `notifications` dentro del catalogo de patrones.
pub mod notifications;
/// Modulo del ejemplo `report_generator` dentro del catalogo de patrones.
pub mod report_generator;
