//! Event Sourcing.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::event_sourcing as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de bank account dentro del catalogo.
pub mod bank_account;
/// Modulo del ejemplo `order_audit` dentro del catalogo de patrones.
pub mod order_audit;
/// Modulo del ejemplo `snapshots` dentro del catalogo de patrones.
pub mod snapshots;
