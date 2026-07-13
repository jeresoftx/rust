//! Bridge.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::gof::structural::bridge as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de notifications dentro del catalogo.
pub mod notifications;
/// Modulo del ejemplo `reports` dentro del catalogo de patrones.
pub mod reports;
/// Modulo del ejemplo `storage` dentro del catalogo de patrones.
pub mod storage;
