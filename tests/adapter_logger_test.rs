use design_patterns_rust::patterns::gof::structural::adapter::logger::{
    AppLogger, ThirdPartyLogger, ThirdPartyLoggerAdapter, log_login_event,
};

#[test]
fn adapter_logs_application_events_through_internal_trait() {
    let logger = ThirdPartyLoggerAdapter::new(ThirdPartyLogger::new("auth-service"));

    let entry = log_login_event(&logger, "user-7");

    assert_eq!(entry, "[auth-service] INFO user-login user_id=user-7");
}

#[test]
fn adapter_exposes_third_party_logger_as_app_logger() {
    let logger: Box<dyn AppLogger> = Box::new(ThirdPartyLoggerAdapter::new(ThirdPartyLogger::new(
        "billing",
    )));

    assert_eq!(
        logger.warn("invoice-delay", "invoice_id=inv-77"),
        "[billing] WARN invoice-delay invoice_id=inv-77"
    );
}
