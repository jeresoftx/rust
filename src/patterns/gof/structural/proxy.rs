//! Proxy.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::gof::structural::proxy as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de authorized repository dentro del catalogo.
pub mod authorized_repository;
/// Modulo del ejemplo `cached_api` dentro del catalogo de patrones.
pub mod cached_api;
/// Modulo del ejemplo `lazy_file` dentro del catalogo de patrones.
pub mod lazy_file;
