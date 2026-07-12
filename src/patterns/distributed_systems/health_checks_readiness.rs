//! Health Checks y Readiness exponen estado operativo para monitoreo.
//!
//! Los ejemplos separan dependencias críticas, liveness/readiness y reportes
//! agregados para orquestadores.

pub mod critical_dependencies;
pub mod orchestrator_report;
pub mod readiness_liveness;
