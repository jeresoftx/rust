//! Domain-Driven Design táctico.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::domain_driven_design_tactical as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de discount service dentro del catalogo.
pub mod discount_service;
/// Modulo del ejemplo `domain_events` dentro del catalogo de patrones.
pub mod domain_events;
/// Modulo del ejemplo `order_aggregate` dentro del catalogo de patrones.
pub mod order_aggregate;
