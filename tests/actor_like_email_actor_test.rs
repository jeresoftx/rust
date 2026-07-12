use design_patterns_rust::patterns::rust_idiomatic::actor_like_workers::email_actor::{
    EmailActor, EmailDelivery, EmailRequest, EmailStatus,
};

#[test]
fn actor_like_email_actor_sends_messages_from_commands() {
    let actor = EmailActor::start();

    let first = actor.send(EmailRequest::new("ana@example.com", "Bienvenida"));
    let second = actor.send(EmailRequest::new("cto@example.com", "Reporte diario"));

    assert_eq!(
        first,
        EmailDelivery::new("ana@example.com", "Bienvenida", EmailStatus::Sent)
    );
    assert_eq!(
        second,
        EmailDelivery::new("cto@example.com", "Reporte diario", EmailStatus::Sent)
    );
    assert_eq!(actor.sent_count(), 2);

    actor.shutdown();
}

#[test]
fn actor_like_email_actor_keeps_delivery_history_in_order() {
    let actor = EmailActor::start();

    actor.send(EmailRequest::new("ana@example.com", "Bienvenida"));
    actor.send(EmailRequest::new("ops@example.com", "Alerta"));

    assert_eq!(
        actor.history(),
        vec![
            EmailDelivery::new("ana@example.com", "Bienvenida", EmailStatus::Sent),
            EmailDelivery::new("ops@example.com", "Alerta", EmailStatus::Sent)
        ]
    );

    actor.shutdown();
}

#[test]
fn actor_like_email_actor_reports_rejected_messages() {
    let actor = EmailActor::start();

    let rejected = actor.send(EmailRequest::new("correo-invalido", "Sin arroba"));

    assert_eq!(
        rejected,
        EmailDelivery::new("correo-invalido", "Sin arroba", EmailStatus::Rejected)
    );
    assert_eq!(actor.sent_count(), 0);
    assert_eq!(actor.rejected_count(), 1);

    actor.shutdown();
}
