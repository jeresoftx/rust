use design_patterns_rust::patterns::gof::creational::builder::transactional_email::{
    EmailBuildError, EmailPayload,
};

#[test]
fn builder_creates_transactional_email_payload() {
    let email = EmailPayload::builder()
        .to("customer@example.com")
        .subject("Your order is ready")
        .body("Order #1001 is ready for pickup.")
        .cc("ops@example.com")
        .header("X-Template", "order-ready")
        .build()
        .expect("valid email");

    assert_eq!(
        email.summary(),
        "to=customer@example.com | cc=ops@example.com | subject=Your order is ready | headers=X-Template:order-ready"
    );
}

#[test]
fn builder_rejects_email_without_recipient() {
    let error = EmailPayload::builder()
        .subject("Missing recipient")
        .body("This cannot be sent.")
        .build()
        .expect_err("recipient is required");

    assert_eq!(error, EmailBuildError::RecipientRequired);
}
