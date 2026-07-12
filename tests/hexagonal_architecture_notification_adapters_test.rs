use design_patterns_rust::patterns::architecture::hexagonal_architecture::notification_adapters::{
    adapters::{EmailNotificationAdapter, SmsNotificationAdapter},
    application::{ConfirmShipment, ShipmentRequest},
    domain::ShipmentConfirmation,
};

#[test]
fn hexagonal_architecture_sends_confirmation_with_email_adapter() {
    let notifier = EmailNotificationAdapter::default();
    let mut confirm = ConfirmShipment::new(notifier);

    let confirmation = confirm.execute(ShipmentRequest::new("ORD-1", "ana@example.com", "TRACK-1"));

    assert_eq!(confirmation, ShipmentConfirmation::new("ORD-1", "TRACK-1"));
    assert_eq!(
        confirm.sent_messages(),
        vec!["email to ana@example.com: order ORD-1 shipped with TRACK-1"]
    );
}

#[test]
fn hexagonal_architecture_swaps_to_sms_adapter_without_changing_use_case() {
    let notifier = SmsNotificationAdapter::default();
    let mut confirm = ConfirmShipment::new(notifier);

    confirm.execute(ShipmentRequest::new("ORD-2", "+526671234567", "TRACK-2"));

    assert_eq!(
        confirm.sent_messages(),
        vec!["sms to +526671234567: ORD-2 shipped TRACK-2"]
    );
}

#[test]
fn hexagonal_architecture_notification_adapter_keeps_core_response_stable() {
    let email = EmailNotificationAdapter::default();
    let sms = SmsNotificationAdapter::default();
    let mut email_confirm = ConfirmShipment::new(email);
    let mut sms_confirm = ConfirmShipment::new(sms);

    let email_confirmation =
        email_confirm.execute(ShipmentRequest::new("ORD-3", "ops@example.com", "TRACK-3"));
    let sms_confirmation =
        sms_confirm.execute(ShipmentRequest::new("ORD-3", "+526670000000", "TRACK-3"));

    assert_eq!(email_confirmation, sms_confirmation);
}
