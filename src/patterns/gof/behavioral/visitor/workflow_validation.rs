use std::collections::HashSet;

/// Contrato publico `WorkflowVisitor` que desacopla las piezas del ejemplo.
pub trait WorkflowVisitor {
    /// Operacion `visit workflow` definida por la abstraccion del ejemplo.
    fn visit_workflow(&mut self, workflow: &Workflow);
    /// Operacion `visit start` definida por la abstraccion del ejemplo.
    fn visit_start(&mut self, node: &StartNode);
    /// Operacion `visit task` definida por la abstraccion del ejemplo.
    fn visit_task(&mut self, node: &TaskNode);
    /// Operacion `visit approval` definida por la abstraccion del ejemplo.
    fn visit_approval(&mut self, node: &ApprovalNode);
    /// Operacion `visit end` definida por la abstraccion del ejemplo.
    fn visit_end(&mut self, node: &EndNode);
    /// Operacion `finish workflow` definida por la abstraccion del ejemplo.
    fn finish_workflow(&mut self);
}

/// Contrato publico `WorkflowNode` que desacopla las piezas del ejemplo.
pub trait WorkflowNode {
    /// Operacion `id` definida por la abstraccion del ejemplo.
    fn id(&self) -> &str;
    /// Operacion `accept` definida por la abstraccion del ejemplo.
    fn accept(&self, visitor: &mut dyn WorkflowVisitor);
}

/// Tipo publico `Workflow` usado por el ejemplo para expresar el dominio del patron.
pub struct Workflow {
    name: String,
    nodes: Vec<Box<dyn WorkflowNode>>,
}

impl Workflow {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            nodes: Vec::new(),
        }
    }

    /// Modela la operacion `name` dentro del ejemplo del patron.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Modela la operacion `with start` dentro del ejemplo del patron.
    pub fn with_start(mut self, id: impl Into<String>) -> Self {
        self.nodes.push(Box::new(StartNode::new(id)));
        self
    }

    /// Modela la operacion `with task` dentro del ejemplo del patron.
    pub fn with_task(mut self, id: impl Into<String>, owner: impl Into<String>) -> Self {
        self.nodes.push(Box::new(TaskNode::new(id, owner)));
        self
    }

    /// Modela la operacion `with approval` dentro del ejemplo del patron.
    pub fn with_approval(mut self, id: impl Into<String>, approver: impl Into<String>) -> Self {
        self.nodes.push(Box::new(ApprovalNode::new(id, approver)));
        self
    }

    /// Modela la operacion `with end` dentro del ejemplo del patron.
    pub fn with_end(mut self, id: impl Into<String>) -> Self {
        self.nodes.push(Box::new(EndNode::new(id)));
        self
    }

    /// Modela la operacion `accept` dentro del ejemplo del patron.
    pub fn accept(&self, visitor: &mut dyn WorkflowVisitor) {
        visitor.visit_workflow(self);
        for node in &self.nodes {
            node.accept(visitor);
        }
        visitor.finish_workflow();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `StartNode` usado por el ejemplo para expresar el dominio del patron.
pub struct StartNode {
    id: String,
}

impl StartNode {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

impl WorkflowNode for StartNode {
    /// Operacion `id` definida por la abstraccion del ejemplo.
    fn id(&self) -> &str {
        &self.id
    }

    /// Operacion `accept` definida por la abstraccion del ejemplo.
    fn accept(&self, visitor: &mut dyn WorkflowVisitor) {
        visitor.visit_start(self);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `TaskNode` usado por el ejemplo para expresar el dominio del patron.
pub struct TaskNode {
    id: String,
    owner: String,
}

impl TaskNode {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>, owner: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            owner: owner.into(),
        }
    }

    /// Modela la operacion `owner` dentro del ejemplo del patron.
    pub fn owner(&self) -> &str {
        &self.owner
    }
}

impl WorkflowNode for TaskNode {
    /// Operacion `id` definida por la abstraccion del ejemplo.
    fn id(&self) -> &str {
        &self.id
    }

    /// Operacion `accept` definida por la abstraccion del ejemplo.
    fn accept(&self, visitor: &mut dyn WorkflowVisitor) {
        visitor.visit_task(self);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ApprovalNode` usado por el ejemplo para expresar el dominio del patron.
pub struct ApprovalNode {
    id: String,
    approver: String,
}

impl ApprovalNode {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>, approver: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            approver: approver.into(),
        }
    }

    /// Modela la operacion `approver` dentro del ejemplo del patron.
    pub fn approver(&self) -> &str {
        &self.approver
    }
}

impl WorkflowNode for ApprovalNode {
    /// Operacion `id` definida por la abstraccion del ejemplo.
    fn id(&self) -> &str {
        &self.id
    }

    /// Operacion `accept` definida por la abstraccion del ejemplo.
    fn accept(&self, visitor: &mut dyn WorkflowVisitor) {
        visitor.visit_approval(self);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `EndNode` usado por el ejemplo para expresar el dominio del patron.
pub struct EndNode {
    id: String,
}

impl EndNode {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

impl WorkflowNode for EndNode {
    /// Operacion `id` definida por la abstraccion del ejemplo.
    fn id(&self) -> &str {
        &self.id
    }

    /// Operacion `accept` definida por la abstraccion del ejemplo.
    fn accept(&self, visitor: &mut dyn WorkflowVisitor) {
        visitor.visit_end(self);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// Tipo publico `WorkflowValidationVisitor` usado por el ejemplo para expresar el dominio del patron.
pub struct WorkflowValidationVisitor {
    has_start: bool,
    has_end: bool,
    seen_ids: HashSet<String>,
    errors: Vec<String>,
    visited_nodes: Vec<String>,
}

impl WorkflowValidationVisitor {
    /// Modela la operacion `is valid` dentro del ejemplo del patron.
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Modela la operacion `errors` dentro del ejemplo del patron.
    pub fn errors(&self) -> Vec<&str> {
        self.errors.iter().map(String::as_str).collect()
    }

    /// Modela la operacion `visited nodes` dentro del ejemplo del patron.
    pub fn visited_nodes(&self) -> Vec<&str> {
        self.visited_nodes.iter().map(String::as_str).collect()
    }

    /// Operacion `track id` definida por la abstraccion del ejemplo.
    fn track_id(&mut self, id: &str) {
        if !self.seen_ids.insert(id.to_string()) {
            self.errors.push(format!("duplicate node id {id}"));
        }
    }
}

impl WorkflowVisitor for WorkflowValidationVisitor {
    /// Operacion `visit workflow` definida por la abstraccion del ejemplo.
    fn visit_workflow(&mut self, workflow: &Workflow) {
        self.visited_nodes
            .push(format!("workflow:{}", workflow.name()));
    }

    /// Operacion `visit start` definida por la abstraccion del ejemplo.
    fn visit_start(&mut self, node: &StartNode) {
        self.has_start = true;
        self.track_id(node.id());
        self.visited_nodes.push(format!("start:{}", node.id()));
    }

    /// Operacion `visit task` definida por la abstraccion del ejemplo.
    fn visit_task(&mut self, node: &TaskNode) {
        self.track_id(node.id());
        if node.owner().is_empty() {
            self.errors
                .push(format!("task {} missing owner", node.id()));
        }
        self.visited_nodes.push(format!("task:{}", node.id()));
    }

    /// Operacion `visit approval` definida por la abstraccion del ejemplo.
    fn visit_approval(&mut self, node: &ApprovalNode) {
        self.track_id(node.id());
        if node.approver().is_empty() {
            self.errors
                .push(format!("approval {} missing approver", node.id()));
        }
        self.visited_nodes.push(format!("approval:{}", node.id()));
    }

    /// Operacion `visit end` definida por la abstraccion del ejemplo.
    fn visit_end(&mut self, node: &EndNode) {
        self.has_end = true;
        self.track_id(node.id());
        self.visited_nodes.push(format!("end:{}", node.id()));
    }

    /// Operacion `finish workflow` definida por la abstraccion del ejemplo.
    fn finish_workflow(&mut self) {
        if !self.has_start {
            self.errors
                .insert(0, "workflow missing start node".to_string());
        }
        if !self.has_end {
            let index = usize::from(!self.has_start);
            self.errors
                .insert(index, "workflow missing end node".to_string());
        }
    }
}
