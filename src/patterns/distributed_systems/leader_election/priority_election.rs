#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    id: String,
    priority: u32,
    alive: bool,
}

impl Node {
    pub fn new(id: impl Into<String>, priority: u32, alive: bool) -> Self {
        Self {
            id: id.into(),
            priority,
            alive,
        }
    }
}

#[derive(Debug)]
pub struct Cluster {
    nodes: Vec<Node>,
}

impl Cluster {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self { nodes }
    }

    pub fn elect_leader(&self) -> Option<String> {
        self.nodes
            .iter()
            .filter(|node| node.alive)
            .max_by_key(|node| node.priority)
            .map(|node| node.id.clone())
    }
}
