//! Health Checks y Readiness exponen estado operativo para monitoreo.
//!
//! Los ejemplos separan dependencias críticas, liveness/readiness y reportes
//! agregados para orquestadores.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::distributed_systems::health_checks_readiness as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de critical dependencies dentro del catalogo.
pub mod critical_dependencies;
/// Modulo del ejemplo `orchestrator_report` dentro del catalogo de patrones.
pub mod orchestrator_report;
/// Modulo del ejemplo `readiness_liveness` dentro del catalogo de patrones.
pub mod readiness_liveness;
