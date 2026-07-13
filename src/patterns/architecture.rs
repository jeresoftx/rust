//! Patrones de arquitectura aplicados con Rust.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de clean architecture dentro del catalogo.
pub mod clean_architecture;
/// Modulo del ejemplo `cqrs` dentro del catalogo de patrones.
pub mod cqrs;
/// Modulo del ejemplo `domain_driven_design_tactical` dentro del catalogo de patrones.
pub mod domain_driven_design_tactical;
/// Modulo del ejemplo `event_sourcing` dentro del catalogo de patrones.
pub mod event_sourcing;
/// Modulo del ejemplo `hexagonal_architecture` dentro del catalogo de patrones.
pub mod hexagonal_architecture;
/// Modulo del ejemplo `layered_architecture` dentro del catalogo de patrones.
pub mod layered_architecture;
/// Modulo del ejemplo `pipeline_architecture` dentro del catalogo de patrones.
pub mod pipeline_architecture;
/// Modulo del ejemplo `plugin_architecture` dentro del catalogo de patrones.
pub mod plugin_architecture;
/// Modulo del ejemplo `repository_unit_of_work` dentro del catalogo de patrones.
pub mod repository_unit_of_work;
/// Modulo del ejemplo `service_layer` dentro del catalogo de patrones.
pub mod service_layer;
