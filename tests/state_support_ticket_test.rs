use design_patterns_rust::patterns::gof::behavioral::state::support_ticket::{
    SupportTicket, TicketState,
};

#[test]
fn state_moves_support_ticket_through_lifecycle() {
    let mut ticket = SupportTicket::new("TCK-1", "Login broken");

    assert_eq!(ticket.state(), TicketState::Open);

    ticket.assign("ana").expect("open ticket can be assigned");
    assert_eq!(ticket.state(), TicketState::Assigned);

    ticket
        .resolve("reset password")
        .expect("assigned ticket can be resolved");
    assert_eq!(ticket.state(), TicketState::Resolved);

    ticket.close().expect("resolved ticket can be closed");
    assert_eq!(ticket.state(), TicketState::Closed);
    assert_eq!(
        ticket.history(),
        vec![
            "opened:TCK-1:Login broken".to_string(),
            "assigned:TCK-1:ana".to_string(),
            "resolved:TCK-1:reset password".to_string(),
            "closed:TCK-1".to_string(),
        ]
    );
}

#[test]
fn state_reopens_resolved_ticket_before_closing() {
    let mut ticket = SupportTicket::new("TCK-2", "Payment failed");
    ticket.assign("luis").unwrap();
    ticket.resolve("gateway retry").unwrap();

    ticket.reopen("customer still blocked").unwrap();

    assert_eq!(ticket.state(), TicketState::Open);
    assert_eq!(
        ticket.close().unwrap_err(),
        "only resolved tickets can be closed"
    );
}

#[test]
fn state_rejects_invalid_support_ticket_transitions() {
    let mut ticket = SupportTicket::new("TCK-3", "Missing invoice");

    assert_eq!(
        ticket.resolve("sent invoice").unwrap_err(),
        "only assigned tickets can be resolved"
    );

    ticket.assign("marta").unwrap();
    ticket.resolve("sent invoice").unwrap();
    ticket.close().unwrap();

    assert_eq!(
        ticket.assign("ana").unwrap_err(),
        "closed tickets cannot be assigned"
    );
}
