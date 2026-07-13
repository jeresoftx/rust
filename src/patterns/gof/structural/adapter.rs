//! Adapter.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::gof::structural::adapter as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de legacy user dentro del catalogo.
pub mod legacy_user;
/// Modulo del ejemplo `logger` dentro del catalogo de patrones.
pub mod logger;
/// Modulo del ejemplo `payment_gateway` dentro del catalogo de patrones.
pub mod payment_gateway;
