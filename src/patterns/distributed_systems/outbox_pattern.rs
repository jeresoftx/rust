//! Outbox Pattern guarda cambios de dominio y mensajes pendientes juntos.
//!
//! Los ejemplos usan almacenamiento en memoria para modelar unidad de trabajo,
//! publicación marcada y reintentos de mensajes no enviados.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::distributed_systems::outbox_pattern as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de publisher marks sent dentro del catalogo.
pub mod publisher_marks_sent;
/// Modulo del ejemplo `retry_pending` dentro del catalogo de patrones.
pub mod retry_pending;
/// Modulo del ejemplo `unit_of_work` dentro del catalogo de patrones.
pub mod unit_of_work;
