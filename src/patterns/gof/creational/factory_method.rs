//! Factory Method.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::gof::creational::factory_method as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de file processors dentro del catalogo.
pub mod file_processors;
/// Modulo del ejemplo `http_clients` dentro del catalogo de patrones.
pub mod http_clients;
/// Modulo del ejemplo `payment_methods` dentro del catalogo de patrones.
pub mod payment_methods;
