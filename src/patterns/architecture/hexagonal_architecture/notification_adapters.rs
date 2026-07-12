pub mod domain {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ShipmentConfirmation {
        order_id: String,
        tracking_code: String,
    }

    impl ShipmentConfirmation {
        pub fn new(order_id: impl Into<String>, tracking_code: impl Into<String>) -> Self {
            Self {
                order_id: order_id.into(),
                tracking_code: tracking_code.into(),
            }
        }

        pub fn order_id(&self) -> &str {
            &self.order_id
        }

        pub fn tracking_code(&self) -> &str {
            &self.tracking_code
        }
    }
}

pub mod ports {
    use super::domain::ShipmentConfirmation;

    pub trait NotificationPort {
        fn notify_shipment(&mut self, recipient: &str, confirmation: &ShipmentConfirmation);
        fn sent_messages(&self) -> Vec<String>;
    }
}

pub mod application {
    use super::domain::ShipmentConfirmation;
    use super::ports::NotificationPort;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ShipmentRequest {
        order_id: String,
        recipient: String,
        tracking_code: String,
    }

    impl ShipmentRequest {
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
    pub struct ConfirmShipment<N> {
        notifier: N,
    }

    impl<N> ConfirmShipment<N>
    where
        N: NotificationPort,
    {
        pub fn new(notifier: N) -> Self {
            Self { notifier }
        }

        pub fn execute(&mut self, request: ShipmentRequest) -> ShipmentConfirmation {
            let confirmation = ShipmentConfirmation::new(request.order_id, request.tracking_code);
            self.notifier
                .notify_shipment(&request.recipient, &confirmation);
            confirmation
        }

        pub fn sent_messages(&self) -> Vec<String> {
            self.notifier.sent_messages()
        }
    }
}

pub mod adapters {
    use super::domain::ShipmentConfirmation;
    use super::ports::NotificationPort;

    #[derive(Debug, Clone, Default)]
    pub struct EmailNotificationAdapter {
        sent_messages: Vec<String>,
    }

    impl NotificationPort for EmailNotificationAdapter {
        fn notify_shipment(&mut self, recipient: &str, confirmation: &ShipmentConfirmation) {
            self.sent_messages.push(format!(
                "email to {recipient}: order {} shipped with {}",
                confirmation.order_id(),
                confirmation.tracking_code()
            ));
        }

        fn sent_messages(&self) -> Vec<String> {
            self.sent_messages.clone()
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct SmsNotificationAdapter {
        sent_messages: Vec<String>,
    }

    impl NotificationPort for SmsNotificationAdapter {
        fn notify_shipment(&mut self, recipient: &str, confirmation: &ShipmentConfirmation) {
            self.sent_messages.push(format!(
                "sms to {recipient}: {} shipped {}",
                confirmation.order_id(),
                confirmation.tracking_code()
            ));
        }

        fn sent_messages(&self) -> Vec<String> {
            self.sent_messages.clone()
        }
    }
}
