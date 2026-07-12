use design_patterns_rust::patterns::distributed_systems::leader_election::priority_election::{
    Cluster, Node,
};

#[test]
fn elects_alive_node_with_highest_priority() {
    let cluster = Cluster::new(vec![
        Node::new("node-a", 10, true),
        Node::new("node-b", 30, true),
        Node::new("node-c", 20, true),
    ]);

    assert_eq!(cluster.elect_leader(), Some("node-b".to_string()));
}

#[test]
fn ignores_nodes_that_are_not_alive() {
    let cluster = Cluster::new(vec![
        Node::new("node-a", 10, true),
        Node::new("node-b", 30, false),
        Node::new("node-c", 20, true),
    ]);

    assert_eq!(cluster.elect_leader(), Some("node-c".to_string()));
}

#[test]
fn returns_none_when_no_node_is_alive() {
    let cluster = Cluster::new(vec![Node::new("node-a", 10, false)]);

    assert_eq!(cluster.elect_leader(), None);
}
