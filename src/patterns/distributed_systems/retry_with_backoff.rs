//! Retry with Backoff.
//!
//! El patrón reintenta operaciones fallidas esperando cada vez más entre
//! intentos, para reducir presión sobre una dependencia que puede recuperarse.

pub mod deterministic_jitter;
pub mod http_client;
pub mod transient_errors;
