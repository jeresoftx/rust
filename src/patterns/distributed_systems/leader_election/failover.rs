#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Node` usado por el ejemplo para expresar el dominio del patron.
pub struct Node {
    id: String,
    priority: u32,
    responsive: bool,
}

impl Node {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>, priority: u32) -> Self {
        Self {
            id: id.into(),
            priority,
            responsive: true,
        }
    }
}

#[derive(Debug)]
/// Tipo publico `FailoverCluster` usado por el ejemplo para expresar el dominio del patron.
pub struct FailoverCluster {
    nodes: Vec<Node>,
    leader: Option<String>,
}

impl FailoverCluster {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(nodes: Vec<Node>) -> Self {
        let mut cluster = Self {
            nodes,
            leader: None,
        };
        cluster.elect();
        cluster
    }

    /// Modela la operacion `mark unresponsive` dentro del ejemplo del patron.
    pub fn mark_unresponsive(&mut self, id: &str) {
        if let Some(node) = self.nodes.iter_mut().find(|node| node.id == id) {
            node.responsive = false;
        }
    }

    /// Modela la operacion `heartbeat` dentro del ejemplo del patron.
    pub fn heartbeat(&mut self, id: &str) {
        if let Some(node) = self.nodes.iter_mut().find(|node| node.id == id) {
            node.responsive = true;
        }
    }

    /// Modela la operacion `elect` dentro del ejemplo del patron.
    pub fn elect(&mut self) {
        self.leader = self
            .nodes
            .iter()
            .filter(|node| node.responsive)
            .max_by_key(|node| node.priority)
            .map(|node| node.id.clone());
    }

    /// Modela la operacion `current leader` dentro del ejemplo del patron.
    pub fn current_leader(&self) -> Option<String> {
        self.leader.clone()
    }
}
