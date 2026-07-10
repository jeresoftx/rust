use design_patterns_rust::patterns::gof::structural::facade::notifications::{
    EmailSender, NotificationFacade, PushSender, SmsSender,
};

#[test]
fn facade_sends_notification_to_multiple_channels_with_one_call() {
    let facade = NotificationFacade::new(EmailSender, SmsSender, PushSender);

    assert_eq!(
        facade.notify_user("user-7", "deploy finished"),
        vec![
            "email user=user-7 body=deploy finished".to_string(),
            "sms user=user-7 body=deploy finished".to_string(),
            "push user=user-7 body=deploy finished".to_string(),
        ]
    );
}

#[test]
fn facade_sends_critical_alert_without_exposing_channels() {
    let facade = NotificationFacade::new(EmailSender, SmsSender, PushSender);

    assert_eq!(
        facade.critical_alert("ops-1", "database down"),
        "critical user=ops-1 delivered=email,sms,push message=database down"
    );
}
