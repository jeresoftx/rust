//! Circuit Breaker evita llamar repetidamente a una dependencia degradada.
//!
//! Los ejemplos de este módulo modelan circuitos deterministas para pruebas:
//! conteo de fallas consecutivas, estado half-open y métricas de rechazos.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::distributed_systems::circuit_breaker as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de consecutive failures dentro del catalogo.
pub mod consecutive_failures;
/// Modulo del ejemplo `half_open_recovery` dentro del catalogo de patrones.
pub mod half_open_recovery;
/// Modulo del ejemplo `open_rejection_metrics` dentro del catalogo de patrones.
pub mod open_rejection_metrics;
