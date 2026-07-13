//! Patrones GoF creacionales.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::gof::creational as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de abstract factory dentro del catalogo.
pub mod abstract_factory;
/// Modulo del ejemplo `builder` dentro del catalogo de patrones.
pub mod builder;
/// Modulo del ejemplo `factory_method` dentro del catalogo de patrones.
pub mod factory_method;
/// Modulo del ejemplo `prototype` dentro del catalogo de patrones.
pub mod prototype;
/// Modulo del ejemplo `singleton` dentro del catalogo de patrones.
pub mod singleton;
