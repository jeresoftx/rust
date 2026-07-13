//! Notification Adapters.
//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::hexagonal_architecture::notification_adapters as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Modulo del ejemplo `domain` dentro del catalogo de patrones.
pub mod domain {
    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `ShipmentConfirmation` usado por el ejemplo para expresar el dominio del patron.
    pub struct ShipmentConfirmation {
        order_id: String,
        tracking_code: String,
    }

    impl ShipmentConfirmation {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(order_id: impl Into<String>, tracking_code: impl Into<String>) -> Self {
            Self {
                order_id: order_id.into(),
                tracking_code: tracking_code.into(),
            }
        }

        /// Modela la operacion `order id` dentro del ejemplo del patron.
        pub fn order_id(&self) -> &str {
            &self.order_id
        }

        /// Modela la operacion `tracking code` dentro del ejemplo del patron.
        pub fn tracking_code(&self) -> &str {
            &self.tracking_code
        }
    }
}

/// Modulo del ejemplo `ports` dentro del catalogo de patrones.
pub mod ports {
    use super::domain::ShipmentConfirmation;

    /// Contrato publico `NotificationPort` que desacopla las piezas del ejemplo.
    pub trait NotificationPort {
        /// Operacion `notify shipment` definida por la abstraccion del ejemplo.
        fn notify_shipment(&mut self, recipient: &str, confirmation: &ShipmentConfirmation);
        /// Operacion `sent messages` definida por la abstraccion del ejemplo.
        fn sent_messages(&self) -> Vec<String>;
    }
}

/// Modulo del ejemplo `application` dentro del catalogo de patrones.
pub mod application {
    use super::domain::ShipmentConfirmation;
    use super::ports::NotificationPort;

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `ShipmentRequest` usado por el ejemplo para expresar el dominio del patron.
    pub struct ShipmentRequest {
        order_id: String,
        recipient: String,
        tracking_code: String,
    }

    impl ShipmentRequest {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(
            order_id: impl Into<String>,
            recipient: impl Into<String>,
            tracking_code: impl Into<String>,
        ) -> Self {
            Self {
                order_id: order_id.into(),
                recipient: recipient.into(),
                tracking_code: tracking_code.into(),
            }
        }
    }

    #[derive(Debug, Clone)]
    /// Tipo publico `ConfirmShipment` usado por el ejemplo para expresar el dominio del patron.
    pub struct ConfirmShipment<N> {
        notifier: N,
    }

    impl<N> ConfirmShipment<N>
    where
        N: NotificationPort,
    {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(notifier: N) -> Self {
            Self { notifier }
        }

        /// Ejecuta el caso de uso o comando del ejemplo.
        pub fn execute(&mut self, request: ShipmentRequest) -> ShipmentConfirmation {
            let confirmation = ShipmentConfirmation::new(request.order_id, request.tracking_code);
            self.notifier
                .notify_shipment(&request.recipient, &confirmation);
            confirmation
        }

        /// Modela la operacion `sent messages` dentro del ejemplo del patron.
        pub fn sent_messages(&self) -> Vec<String> {
            self.notifier.sent_messages()
        }
    }
}

/// Modulo del ejemplo `adapters` dentro del catalogo de patrones.
pub mod adapters {
    use super::domain::ShipmentConfirmation;
    use super::ports::NotificationPort;

    #[derive(Debug, Clone, Default)]
    /// Tipo publico `EmailNotificationAdapter` usado por el ejemplo para expresar el dominio del patron.
    pub struct EmailNotificationAdapter {
        sent_messages: Vec<String>,
    }

    impl NotificationPort for EmailNotificationAdapter {
        /// Operacion `notify shipment` definida por la abstraccion del ejemplo.
        fn notify_shipment(&mut self, recipient: &str, confirmation: &ShipmentConfirmation) {
            self.sent_messages.push(format!(
                "email to {recipient}: order {} shipped with {}",
                confirmation.order_id(),
                confirmation.tracking_code()
            ));
        }

        /// Operacion `sent messages` definida por la abstraccion del ejemplo.
        fn sent_messages(&self) -> Vec<String> {
            self.sent_messages.clone()
        }
    }

    #[derive(Debug, Clone, Default)]
    /// Tipo publico `SmsNotificationAdapter` usado por el ejemplo para expresar el dominio del patron.
    pub struct SmsNotificationAdapter {
        sent_messages: Vec<String>,
    }

    impl NotificationPort for SmsNotificationAdapter {
        /// Operacion `notify shipment` definida por la abstraccion del ejemplo.
        fn notify_shipment(&mut self, recipient: &str, confirmation: &ShipmentConfirmation) {
            self.sent_messages.push(format!(
                "sms to {recipient}: {} shipped {}",
                confirmation.order_id(),
                confirmation.tracking_code()
            ));
        }

        /// Operacion `sent messages` definida por la abstraccion del ejemplo.
        fn sent_messages(&self) -> Vec<String> {
            self.sent_messages.clone()
        }
    }
}
