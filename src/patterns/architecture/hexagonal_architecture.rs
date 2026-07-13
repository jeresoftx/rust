//! Hexagonal Architecture.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::hexagonal_architecture as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de checkout ports adapters dentro del catalogo.
pub mod checkout_ports_adapters;
/// Modulo del ejemplo `notification_adapters` dentro del catalogo de patrones.
pub mod notification_adapters;
/// Modulo del ejemplo `repository_adapters` dentro del catalogo de patrones.
pub mod repository_adapters;
