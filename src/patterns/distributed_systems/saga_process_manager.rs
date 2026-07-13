//! Saga / Process Manager coordina procesos largos entre varios pasos.
//!
//! Los ejemplos modelan reserva, pago, envío, compensaciones y estado
//! persistente de forma determinista.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::distributed_systems::saga_process_manager as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de checkout flow dentro del catalogo.
pub mod checkout_flow;
/// Modulo del ejemplo `compensation` dentro del catalogo de patrones.
pub mod compensation;
/// Modulo del ejemplo `persistent_state` dentro del catalogo de patrones.
pub mod persistent_state;
