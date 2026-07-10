use design_patterns_rust::patterns::gof::structural::bridge::notifications::{
    AlertNotification, EmailChannel, PushChannel, SmsChannel,
};

#[test]
fn bridge_sends_the_same_alert_through_different_channels() {
    let email_alert = AlertNotification::new(EmailChannel::new("ops@example.com"));
    let sms_alert = AlertNotification::new(SmsChannel::new("+5215550100"));
    let push_alert = AlertNotification::new(PushChannel::new("device-77"));

    assert_eq!(
        email_alert.send("database down"),
        "email to=ops@example.com subject=ALERT body=database down"
    );
    assert_eq!(
        sms_alert.send("database down"),
        "sms to=+5215550100 message=ALERT: database down"
    );
    assert_eq!(
        push_alert.send("database down"),
        "push device=device-77 title=ALERT body=database down"
    );
}

#[test]
fn bridge_changes_notification_abstraction_without_changing_channel() {
    let alert = AlertNotification::new(EmailChannel::new("security@example.com"));

    assert_eq!(
        alert.send("login spike"),
        "email to=security@example.com subject=ALERT body=login spike"
    );
}
