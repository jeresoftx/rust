use design_patterns_rust::patterns::distributed_systems::leader_election::single_leader_round::{
    ElectionError, RoundElection,
};

#[test]
fn grants_first_leader_claim_in_round() {
    let mut election = RoundElection::new(1);

    let claim = election.claim_leadership("node-a");

    assert_eq!(claim, Ok("node-a".to_string()));
    assert_eq!(election.current_leader(), Some("node-a".to_string()));
}

#[test]
fn rejects_second_leader_claim_in_same_round() {
    let mut election = RoundElection::new(1);

    election.claim_leadership("node-a").unwrap();
    let second = election.claim_leadership("node-b");

    assert_eq!(second, Err(ElectionError::LeaderAlreadyElected));
    assert_eq!(election.current_leader(), Some("node-a".to_string()));
}

#[test]
fn new_round_allows_new_leader_claim() {
    let mut election = RoundElection::new(1);

    election.claim_leadership("node-a").unwrap();
    election.start_round(2);

    assert_eq!(
        election.claim_leadership("node-b"),
        Ok("node-b".to_string())
    );
    assert_eq!(election.current_leader(), Some("node-b".to_string()));
}
