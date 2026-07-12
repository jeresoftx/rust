//! Rate Limiting controla cuántas operaciones se aceptan en una ventana.
//!
//! Los ejemplos usan relojes lógicos y contadores deterministas para modelar
//! token buckets, límites por API key y respuestas con tiempo de reintento.

pub mod token_bucket;
