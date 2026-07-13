//! Patrones GoF estructurales.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::gof::structural as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de adapter dentro del catalogo.
pub mod adapter;
/// Modulo del ejemplo `bridge` dentro del catalogo de patrones.
pub mod bridge;
/// Modulo del ejemplo `composite` dentro del catalogo de patrones.
pub mod composite;
/// Modulo del ejemplo `decorator` dentro del catalogo de patrones.
pub mod decorator;
/// Modulo del ejemplo `facade` dentro del catalogo de patrones.
pub mod facade;
/// Modulo del ejemplo `flyweight` dentro del catalogo de patrones.
pub mod flyweight;
/// Modulo del ejemplo `proxy` dentro del catalogo de patrones.
pub mod proxy;
