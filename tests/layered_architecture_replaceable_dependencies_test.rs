use design_patterns_rust::patterns::architecture::layered_architecture::replaceable_dependencies::{
    application::{AssignTicket, AssignTicketCommand},
    domain::{SupportTicket, TicketPriority},
    infrastructure::{InMemoryTicketRepository, RecordingTicketNotifier},
    presentation::{AssignTicketRequest, TicketController},
};

#[test]
fn layered_architecture_tests_domain_layer_without_infrastructure() {
    let ticket = SupportTicket::open("TCK-1", "No puedo iniciar sesión", TicketPriority::High)
        .expect("ticket should be valid");

    assert_eq!(ticket.id(), "TCK-1");
    assert_eq!(ticket.priority(), TicketPriority::High);
    assert_eq!(ticket.assigned_agent(), None);
}

#[test]
fn layered_architecture_tests_application_layer_with_replaceable_dependencies() {
    let repository = InMemoryTicketRepository::default();
    let notifier = RecordingTicketNotifier::default();
    let mut use_case = AssignTicket::new(repository, notifier);

    let ticket = use_case
        .execute(AssignTicketCommand::new(
            "TCK-2",
            "Error al pagar",
            TicketPriority::Urgent,
            "Ana",
        ))
        .expect("ticket should be assigned");

    assert_eq!(ticket.assigned_agent(), Some("Ana"));
    assert_eq!(use_case.saved_count(), 1);
    assert_eq!(use_case.notifications(), vec!["TCK-2 assigned to Ana"]);
}

#[test]
fn layered_architecture_tests_presentation_layer_as_translation_boundary() {
    let repository = InMemoryTicketRepository::default();
    let notifier = RecordingTicketNotifier::default();
    let use_case = AssignTicket::new(repository, notifier);
    let mut controller = TicketController::new(use_case);

    let response = controller.assign_ticket(AssignTicketRequest::new(
        "TCK-3",
        "No llega el correo",
        "medium",
        "Luis",
    ));

    assert_eq!(response.status_code(), 201);
    assert_eq!(response.body().ticket_id(), "TCK-3");
    assert_eq!(response.body().summary(), "TCK-3 assigned to Luis");
}
