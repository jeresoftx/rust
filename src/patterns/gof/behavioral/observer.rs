//! Observer.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::gof::behavioral::observer as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de inventory notifications dentro del catalogo.
pub mod inventory_notifications;
/// Modulo del ejemplo `metrics_logs` dentro del catalogo de patrones.
pub mod metrics_logs;
/// Modulo del ejemplo `order_events` dentro del catalogo de patrones.
pub mod order_events;
