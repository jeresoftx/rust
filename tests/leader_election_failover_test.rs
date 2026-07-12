use design_patterns_rust::patterns::distributed_systems::leader_election::failover::{
    FailoverCluster, Node,
};

#[test]
fn promotes_next_node_when_leader_misses_heartbeat() {
    let mut cluster = FailoverCluster::new(vec![
        Node::new("node-a", 30),
        Node::new("node-b", 20),
        Node::new("node-c", 10),
    ]);

    assert_eq!(cluster.current_leader(), Some("node-a".to_string()));

    cluster.mark_unresponsive("node-a");
    cluster.elect();

    assert_eq!(cluster.current_leader(), Some("node-b".to_string()));
}

#[test]
fn keeps_leader_when_heartbeat_is_seen() {
    let mut cluster = FailoverCluster::new(vec![Node::new("node-a", 30), Node::new("node-b", 20)]);

    cluster.heartbeat("node-a");
    cluster.elect();

    assert_eq!(cluster.current_leader(), Some("node-a".to_string()));
}

#[test]
fn returns_none_when_all_nodes_are_unresponsive() {
    let mut cluster = FailoverCluster::new(vec![Node::new("node-a", 30)]);

    cluster.mark_unresponsive("node-a");
    cluster.elect();

    assert_eq!(cluster.current_leader(), None);
}
