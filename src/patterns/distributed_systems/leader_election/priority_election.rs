#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Node` usado por el ejemplo para expresar el dominio del patron.
pub struct Node {
    id: String,
    priority: u32,
    alive: bool,
}

impl Node {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>, priority: u32, alive: bool) -> Self {
        Self {
            id: id.into(),
            priority,
            alive,
        }
    }
}

#[derive(Debug)]
/// Tipo publico `Cluster` usado por el ejemplo para expresar el dominio del patron.
pub struct Cluster {
    nodes: Vec<Node>,
}

impl Cluster {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(nodes: Vec<Node>) -> Self {
        Self { nodes }
    }

    /// Modela la operacion `elect leader` dentro del ejemplo del patron.
    pub fn elect_leader(&self) -> Option<String> {
        self.nodes
            .iter()
            .filter(|node| node.alive)
            .max_by_key(|node| node.priority)
            .map(|node| node.id.clone())
    }
}
