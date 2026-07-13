//! Patrones GoF de comportamiento.

//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::gof::behavioral as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Reexporta los ejemplos de chain of responsibility dentro del catalogo.
pub mod chain_of_responsibility;
/// Modulo del ejemplo `command` dentro del catalogo de patrones.
pub mod command;
/// Modulo del ejemplo `interpreter` dentro del catalogo de patrones.
pub mod interpreter;
/// Modulo del ejemplo `iterator` dentro del catalogo de patrones.
pub mod iterator;
/// Modulo del ejemplo `mediator` dentro del catalogo de patrones.
pub mod mediator;
/// Modulo del ejemplo `memento` dentro del catalogo de patrones.
pub mod memento;
/// Modulo del ejemplo `observer` dentro del catalogo de patrones.
pub mod observer;
/// Modulo del ejemplo `state` dentro del catalogo de patrones.
pub mod state;
/// Modulo del ejemplo `strategy` dentro del catalogo de patrones.
pub mod strategy;
/// Modulo del ejemplo `template_method` dentro del catalogo de patrones.
pub mod template_method;
/// Modulo del ejemplo `visitor` dentro del catalogo de patrones.
pub mod visitor;
