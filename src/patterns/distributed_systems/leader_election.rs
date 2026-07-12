//! Leader Election simulado elige un nodo coordinador de forma determinista.
//!
//! Los ejemplos cubren prioridad, failover y prevención de dos líderes activos
//! por ronda.

pub mod failover;
pub mod priority_election;
pub mod single_leader_round;
