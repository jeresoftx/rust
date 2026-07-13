//! Leader Election simulado elige un nodo coordinador de forma determinista.
//!
//! Los ejemplos cubren prioridad, failover y prevención de dos líderes activos
//! por ronda.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::distributed_systems::leader_election as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de failover dentro del catalogo.
pub mod failover;
/// Modulo del ejemplo `priority_election` dentro del catalogo de patrones.
pub mod priority_election;
/// Modulo del ejemplo `single_leader_round` dentro del catalogo de patrones.
pub mod single_leader_round;
