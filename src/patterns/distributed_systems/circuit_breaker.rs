//! Circuit Breaker evita llamar repetidamente a una dependencia degradada.
//!
//! Los ejemplos de este módulo modelan circuitos deterministas para pruebas:
//! conteo de fallas consecutivas, estado half-open y métricas de rechazos.

pub mod consecutive_failures;
pub mod half_open_recovery;
pub mod open_rejection_metrics;
