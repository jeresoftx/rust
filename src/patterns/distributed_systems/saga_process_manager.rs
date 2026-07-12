//! Saga / Process Manager coordina procesos largos entre varios pasos.
//!
//! Los ejemplos modelan reserva, pago, envío, compensaciones y estado
//! persistente de forma determinista.

pub mod checkout_flow;
pub mod compensation;
