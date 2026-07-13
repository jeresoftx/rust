//! Decorator.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::gof::structural::decorator as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de cached repository dentro del catalogo.
pub mod cached_repository;
/// Modulo del ejemplo `http_client` dentro del catalogo de patrones.
pub mod http_client;
/// Modulo del ejemplo `order_validation` dentro del catalogo de patrones.
pub mod order_validation;
