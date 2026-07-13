//! Abstract Factory.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::gof::creational::abstract_factory as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de database connectors dentro del catalogo.
pub mod database_connectors;
/// Modulo del ejemplo `payment_providers` dentro del catalogo de patrones.
pub mod payment_providers;
/// Modulo del ejemplo `ui_components` dentro del catalogo de patrones.
pub mod ui_components;
