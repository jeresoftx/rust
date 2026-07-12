#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    id: String,
    priority: u32,
    responsive: bool,
}

impl Node {
    pub fn new(id: impl Into<String>, priority: u32) -> Self {
        Self {
            id: id.into(),
            priority,
            responsive: true,
        }
    }
}

#[derive(Debug)]
pub struct FailoverCluster {
    nodes: Vec<Node>,
    leader: Option<String>,
}

impl FailoverCluster {
    pub fn new(nodes: Vec<Node>) -> Self {
        let mut cluster = Self {
            nodes,
            leader: None,
        };
        cluster.elect();
        cluster
    }

    pub fn mark_unresponsive(&mut self, id: &str) {
        if let Some(node) = self.nodes.iter_mut().find(|node| node.id == id) {
            node.responsive = false;
        }
    }

    pub fn heartbeat(&mut self, id: &str) {
        if let Some(node) = self.nodes.iter_mut().find(|node| node.id == id) {
            node.responsive = true;
        }
    }

    pub fn elect(&mut self) {
        self.leader = self
            .nodes
            .iter()
            .filter(|node| node.responsive)
            .max_by_key(|node| node.priority)
            .map(|node| node.id.clone());
    }

    pub fn current_leader(&self) -> Option<String> {
        self.leader.clone()
    }
}
