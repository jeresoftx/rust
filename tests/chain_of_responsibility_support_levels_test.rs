use design_patterns_rust::patterns::gof::behavioral::chain_of_responsibility::support_levels::{
    SupportChain, SupportTicket, TicketKind,
};

#[test]
fn chain_routes_password_ticket_to_level_one_support() {
    let chain = SupportChain::default();
    let ticket = SupportTicket::new("T-100", TicketKind::PasswordReset);

    let resolution = chain.resolve(&ticket);

    assert_eq!(resolution, "T-100 handled by level 1: password reset");
}

#[test]
fn chain_escalates_infrastructure_ticket_to_level_three_support() {
    let chain = SupportChain::default();
    let ticket = SupportTicket::new("T-900", TicketKind::InfrastructureIncident);

    let resolution = chain.resolve(&ticket);

    assert_eq!(
        resolution,
        "T-900 handled by level 3: infrastructure incident"
    );
}
