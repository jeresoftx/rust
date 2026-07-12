use std::collections::HashSet;

pub trait WorkflowVisitor {
    fn visit_workflow(&mut self, workflow: &Workflow);
    fn visit_start(&mut self, node: &StartNode);
    fn visit_task(&mut self, node: &TaskNode);
    fn visit_approval(&mut self, node: &ApprovalNode);
    fn visit_end(&mut self, node: &EndNode);
    fn finish_workflow(&mut self);
}

pub trait WorkflowNode {
    fn id(&self) -> &str;
    fn accept(&self, visitor: &mut dyn WorkflowVisitor);
}

pub struct Workflow {
    name: String,
    nodes: Vec<Box<dyn WorkflowNode>>,
}

impl Workflow {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            nodes: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn with_start(mut self, id: impl Into<String>) -> Self {
        self.nodes.push(Box::new(StartNode::new(id)));
        self
    }

    pub fn with_task(mut self, id: impl Into<String>, owner: impl Into<String>) -> Self {
        self.nodes.push(Box::new(TaskNode::new(id, owner)));
        self
    }

    pub fn with_approval(mut self, id: impl Into<String>, approver: impl Into<String>) -> Self {
        self.nodes.push(Box::new(ApprovalNode::new(id, approver)));
        self
    }

    pub fn with_end(mut self, id: impl Into<String>) -> Self {
        self.nodes.push(Box::new(EndNode::new(id)));
        self
    }

    pub fn accept(&self, visitor: &mut dyn WorkflowVisitor) {
        visitor.visit_workflow(self);
        for node in &self.nodes {
            node.accept(visitor);
        }
        visitor.finish_workflow();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartNode {
    id: String,
}

impl StartNode {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

impl WorkflowNode for StartNode {
    fn id(&self) -> &str {
        &self.id
    }

    fn accept(&self, visitor: &mut dyn WorkflowVisitor) {
        visitor.visit_start(self);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskNode {
    id: String,
    owner: String,
}

impl TaskNode {
    pub fn new(id: impl Into<String>, owner: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            owner: owner.into(),
        }
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }
}

impl WorkflowNode for TaskNode {
    fn id(&self) -> &str {
        &self.id
    }

    fn accept(&self, visitor: &mut dyn WorkflowVisitor) {
        visitor.visit_task(self);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalNode {
    id: String,
    approver: String,
}

impl ApprovalNode {
    pub fn new(id: impl Into<String>, approver: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            approver: approver.into(),
        }
    }

    pub fn approver(&self) -> &str {
        &self.approver
    }
}

impl WorkflowNode for ApprovalNode {
    fn id(&self) -> &str {
        &self.id
    }

    fn accept(&self, visitor: &mut dyn WorkflowVisitor) {
        visitor.visit_approval(self);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndNode {
    id: String,
}

impl EndNode {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

impl WorkflowNode for EndNode {
    fn id(&self) -> &str {
        &self.id
    }

    fn accept(&self, visitor: &mut dyn WorkflowVisitor) {
        visitor.visit_end(self);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WorkflowValidationVisitor {
    has_start: bool,
    has_end: bool,
    seen_ids: HashSet<String>,
    errors: Vec<String>,
    visited_nodes: Vec<String>,
}

impl WorkflowValidationVisitor {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn errors(&self) -> Vec<&str> {
        self.errors.iter().map(String::as_str).collect()
    }

    pub fn visited_nodes(&self) -> Vec<&str> {
        self.visited_nodes.iter().map(String::as_str).collect()
    }

    fn track_id(&mut self, id: &str) {
        if !self.seen_ids.insert(id.to_string()) {
            self.errors.push(format!("duplicate node id {id}"));
        }
    }
}

impl WorkflowVisitor for WorkflowValidationVisitor {
    fn visit_workflow(&mut self, workflow: &Workflow) {
        self.visited_nodes
            .push(format!("workflow:{}", workflow.name()));
    }

    fn visit_start(&mut self, node: &StartNode) {
        self.has_start = true;
        self.track_id(node.id());
        self.visited_nodes.push(format!("start:{}", node.id()));
    }

    fn visit_task(&mut self, node: &TaskNode) {
        self.track_id(node.id());
        if node.owner().is_empty() {
            self.errors
                .push(format!("task {} missing owner", node.id()));
        }
        self.visited_nodes.push(format!("task:{}", node.id()));
    }

    fn visit_approval(&mut self, node: &ApprovalNode) {
        self.track_id(node.id());
        if node.approver().is_empty() {
            self.errors
                .push(format!("approval {} missing approver", node.id()));
        }
        self.visited_nodes.push(format!("approval:{}", node.id()));
    }

    fn visit_end(&mut self, node: &EndNode) {
        self.has_end = true;
        self.track_id(node.id());
        self.visited_nodes.push(format!("end:{}", node.id()));
    }

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
