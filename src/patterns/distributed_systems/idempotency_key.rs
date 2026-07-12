//! Idempotency Key evita duplicar efectos cuando una petición se repite.
//!
//! Los ejemplos modelan pagos, respuestas cacheadas y conflictos de payload
//! con almacenamiento en memoria determinista.

pub mod cached_responses;
pub mod payload_conflict;
pub mod payment_deduplication;
