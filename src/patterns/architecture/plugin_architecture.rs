//! Plugin Architecture.
//!
//! El patrón define contratos estables para agregar capacidades al sistema sin
//! modificar el núcleo de la aplicación.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::plugin_architecture as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de configured strategies dentro del catalogo.
pub mod configured_strategies;
/// Modulo del ejemplo `export_plugins` dentro del catalogo de patrones.
pub mod export_plugins;
/// Modulo del ejemplo `internal_extensions` dentro del catalogo de patrones.
pub mod internal_extensions;
