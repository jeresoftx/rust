//! Pipeline Architecture.
//!
//! El patrón divide un flujo de trabajo en etapas pequeñas y ordenadas. Cada
//! etapa recibe una entrada, la transforma o valida, y entrega el resultado a la
//! siguiente etapa.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::pipeline_architecture as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de csv etl dentro del catalogo.
pub mod csv_etl;
/// Modulo del ejemplo `event_enrichment` dentro del catalogo de patrones.
pub mod event_enrichment;
/// Modulo del ejemplo `image_processing` dentro del catalogo de patrones.
pub mod image_processing;
