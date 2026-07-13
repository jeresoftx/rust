//! Service Layer.
//!
//! El patrón concentra los casos de uso de aplicación para que los controladores,
//! jobs o comandos de consola no carguen con la orquestación del negocio.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::service_layer as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de checkout service dentro del catalogo.
pub mod checkout_service;
/// Modulo del ejemplo `reporting_service` dentro del catalogo de patrones.
pub mod reporting_service;
/// Modulo del ejemplo `user_registration` dentro del catalogo de patrones.
pub mod user_registration;
