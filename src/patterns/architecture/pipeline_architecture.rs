//! Pipeline Architecture.
//!
//! El patrón divide un flujo de trabajo en etapas pequeñas y ordenadas. Cada
//! etapa recibe una entrada, la transforma o valida, y entrega el resultado a la
//! siguiente etapa.

pub mod csv_etl;
pub mod event_enrichment;
pub mod image_processing;
