//! Service Layer.
//!
//! El patrón concentra los casos de uso de aplicación para que los controladores,
//! jobs o comandos de consola no carguen con la orquestación del negocio.

pub mod checkout_service;
pub mod reporting_service;
pub mod user_registration;
