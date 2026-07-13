//! Strategy.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::gof::behavioral::strategy as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de discounts dentro del catalogo.
pub mod discounts;
/// Modulo del ejemplo `result_sorting` dentro del catalogo de patrones.
pub mod result_sorting;
/// Modulo del ejemplo `shipping` dentro del catalogo de patrones.
pub mod shipping;
