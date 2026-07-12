//! Rate Limiting controla cuántas operaciones se aceptan en una ventana.
//!
//! Los ejemplos usan relojes lógicos y contadores deterministas para modelar
//! token buckets, límites por API key y respuestas con tiempo de reintento.

pub mod api_key_limit;
pub mod retry_after_response;
pub mod token_bucket;
