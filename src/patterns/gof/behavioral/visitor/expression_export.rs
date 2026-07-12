pub trait ExpressionVisitor {
    fn visit_number(&mut self, node: &Number);
    fn visit_variable(&mut self, node: &Variable);
    fn visit_add(&mut self, node: &Add);
    fn visit_multiply(&mut self, node: &Multiply);
}

pub trait VisitableExpression {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Number {
    value: i32,
}

impl Number {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

impl VisitableExpression for Number {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_number(self);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    name: String,
}

impl Variable {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl VisitableExpression for Variable {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_variable(self);
    }
}

pub struct Add {
    left: Box<dyn VisitableExpression>,
    right: Box<dyn VisitableExpression>,
}

impl Add {
    pub fn new(
        left: impl VisitableExpression + 'static,
        right: impl VisitableExpression + 'static,
    ) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn left(&self) -> &dyn VisitableExpression {
        self.left.as_ref()
    }

    pub fn right(&self) -> &dyn VisitableExpression {
        self.right.as_ref()
    }
}

impl VisitableExpression for Add {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_add(self);
    }
}

pub struct Multiply {
    left: Box<dyn VisitableExpression>,
    right: Box<dyn VisitableExpression>,
}

impl Multiply {
    pub fn new(
        left: impl VisitableExpression + 'static,
        right: impl VisitableExpression + 'static,
    ) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn left(&self) -> &dyn VisitableExpression {
        self.left.as_ref()
    }

    pub fn right(&self) -> &dyn VisitableExpression {
        self.right.as_ref()
    }
}

impl VisitableExpression for Multiply {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_multiply(self);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TextExportVisitor {
    stack: Vec<String>,
}

impl TextExportVisitor {
    pub fn output(&self) -> String {
        self.stack.last().cloned().unwrap_or_default()
    }

    fn pop_pair(&mut self) -> (String, String) {
        let right = self.stack.pop().unwrap_or_default();
        let left = self.stack.pop().unwrap_or_default();
        (left, right)
    }
}

impl ExpressionVisitor for TextExportVisitor {
    fn visit_number(&mut self, node: &Number) {
        self.stack.push(node.value().to_string());
    }

    fn visit_variable(&mut self, node: &Variable) {
        self.stack.push(node.name().to_string());
    }

    fn visit_add(&mut self, node: &Add) {
        node.left().accept(self);
        node.right().accept(self);
        let (left, right) = self.pop_pair();
        self.stack.push(format!("({left} + {right})"));
    }

    fn visit_multiply(&mut self, node: &Multiply) {
        node.left().accept(self);
        node.right().accept(self);
        let (left, right) = self.pop_pair();
        self.stack.push(format!("({left} * {right})"));
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct JsonExportVisitor {
    stack: Vec<String>,
}

impl JsonExportVisitor {
    pub fn output(&self) -> String {
        self.stack.last().cloned().unwrap_or_default()
    }

    fn pop_pair(&mut self) -> (String, String) {
        let right = self.stack.pop().unwrap_or_default();
        let left = self.stack.pop().unwrap_or_default();
        (left, right)
    }
}

impl ExpressionVisitor for JsonExportVisitor {
    fn visit_number(&mut self, node: &Number) {
        self.stack
            .push(format!(r#"{{"type":"number","value":{}}}"#, node.value()));
    }

    fn visit_variable(&mut self, node: &Variable) {
        self.stack
            .push(format!(r#"{{"type":"variable","name":"{}"}}"#, node.name()));
    }

    fn visit_add(&mut self, node: &Add) {
        node.left().accept(self);
        node.right().accept(self);
        let (left, right) = self.pop_pair();
        self.stack
            .push(format!(r#"{{"type":"add","left":{left},"right":{right}}}"#));
    }

    fn visit_multiply(&mut self, node: &Multiply) {
        node.left().accept(self);
        node.right().accept(self);
        let (left, right) = self.pop_pair();
        self.stack.push(format!(
            r#"{{"type":"multiply","left":{left},"right":{right}}}"#
        ));
    }
}
