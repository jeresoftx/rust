/// Contrato publico `ExpressionVisitor` que desacopla las piezas del ejemplo.
pub trait ExpressionVisitor {
    /// Operacion `visit number` definida por la abstraccion del ejemplo.
    fn visit_number(&mut self, node: &Number);
    /// Operacion `visit variable` definida por la abstraccion del ejemplo.
    fn visit_variable(&mut self, node: &Variable);
    /// Operacion `visit add` definida por la abstraccion del ejemplo.
    fn visit_add(&mut self, node: &Add);
    /// Operacion `visit multiply` definida por la abstraccion del ejemplo.
    fn visit_multiply(&mut self, node: &Multiply);
}

/// Contrato publico `VisitableExpression` que desacopla las piezas del ejemplo.
pub trait VisitableExpression {
    /// Operacion `accept` definida por la abstraccion del ejemplo.
    fn accept(&self, visitor: &mut dyn ExpressionVisitor);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Tipo publico `Number` usado por el ejemplo para expresar el dominio del patron.
pub struct Number {
    value: i32,
}

impl Number {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    /// Modela la operacion `value` dentro del ejemplo del patron.
    pub fn value(&self) -> i32 {
        self.value
    }
}

impl VisitableExpression for Number {
    /// Operacion `accept` definida por la abstraccion del ejemplo.
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_number(self);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Variable` usado por el ejemplo para expresar el dominio del patron.
pub struct Variable {
    name: String,
}

impl Variable {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Modela la operacion `name` dentro del ejemplo del patron.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl VisitableExpression for Variable {
    /// Operacion `accept` definida por la abstraccion del ejemplo.
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_variable(self);
    }
}

/// Tipo publico `Add` usado por el ejemplo para expresar el dominio del patron.
pub struct Add {
    left: Box<dyn VisitableExpression>,
    right: Box<dyn VisitableExpression>,
}

impl Add {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(
        left: impl VisitableExpression + 'static,
        right: impl VisitableExpression + 'static,
    ) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Modela la operacion `left` dentro del ejemplo del patron.
    pub fn left(&self) -> &dyn VisitableExpression {
        self.left.as_ref()
    }

    /// Modela la operacion `right` dentro del ejemplo del patron.
    pub fn right(&self) -> &dyn VisitableExpression {
        self.right.as_ref()
    }
}

impl VisitableExpression for Add {
    /// Operacion `accept` definida por la abstraccion del ejemplo.
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_add(self);
    }
}

/// Tipo publico `Multiply` usado por el ejemplo para expresar el dominio del patron.
pub struct Multiply {
    left: Box<dyn VisitableExpression>,
    right: Box<dyn VisitableExpression>,
}

impl Multiply {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(
        left: impl VisitableExpression + 'static,
        right: impl VisitableExpression + 'static,
    ) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Modela la operacion `left` dentro del ejemplo del patron.
    pub fn left(&self) -> &dyn VisitableExpression {
        self.left.as_ref()
    }

    /// Modela la operacion `right` dentro del ejemplo del patron.
    pub fn right(&self) -> &dyn VisitableExpression {
        self.right.as_ref()
    }
}

impl VisitableExpression for Multiply {
    /// Operacion `accept` definida por la abstraccion del ejemplo.
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_multiply(self);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// Tipo publico `TextExportVisitor` usado por el ejemplo para expresar el dominio del patron.
pub struct TextExportVisitor {
    stack: Vec<String>,
}

impl TextExportVisitor {
    /// Modela la operacion `output` dentro del ejemplo del patron.
    pub fn output(&self) -> String {
        self.stack.last().cloned().unwrap_or_default()
    }

    /// Operacion `pop pair` definida por la abstraccion del ejemplo.
    fn pop_pair(&mut self) -> (String, String) {
        let right = self.stack.pop().unwrap_or_default();
        let left = self.stack.pop().unwrap_or_default();
        (left, right)
    }
}

impl ExpressionVisitor for TextExportVisitor {
    /// Operacion `visit number` definida por la abstraccion del ejemplo.
    fn visit_number(&mut self, node: &Number) {
        self.stack.push(node.value().to_string());
    }

    /// Operacion `visit variable` definida por la abstraccion del ejemplo.
    fn visit_variable(&mut self, node: &Variable) {
        self.stack.push(node.name().to_string());
    }

    /// Operacion `visit add` definida por la abstraccion del ejemplo.
    fn visit_add(&mut self, node: &Add) {
        node.left().accept(self);
        node.right().accept(self);
        let (left, right) = self.pop_pair();
        self.stack.push(format!("({left} + {right})"));
    }

    /// Operacion `visit multiply` definida por la abstraccion del ejemplo.
    fn visit_multiply(&mut self, node: &Multiply) {
        node.left().accept(self);
        node.right().accept(self);
        let (left, right) = self.pop_pair();
        self.stack.push(format!("({left} * {right})"));
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// Tipo publico `JsonExportVisitor` usado por el ejemplo para expresar el dominio del patron.
pub struct JsonExportVisitor {
    stack: Vec<String>,
}

impl JsonExportVisitor {
    /// Modela la operacion `output` dentro del ejemplo del patron.
    pub fn output(&self) -> String {
        self.stack.last().cloned().unwrap_or_default()
    }

    /// Operacion `pop pair` definida por la abstraccion del ejemplo.
    fn pop_pair(&mut self) -> (String, String) {
        let right = self.stack.pop().unwrap_or_default();
        let left = self.stack.pop().unwrap_or_default();
        (left, right)
    }
}

impl ExpressionVisitor for JsonExportVisitor {
    /// Operacion `visit number` definida por la abstraccion del ejemplo.
    fn visit_number(&mut self, node: &Number) {
        self.stack
            .push(format!(r#"{{"type":"number","value":{}}}"#, node.value()));
    }

    /// Operacion `visit variable` definida por la abstraccion del ejemplo.
    fn visit_variable(&mut self, node: &Variable) {
        self.stack
            .push(format!(r#"{{"type":"variable","name":"{}"}}"#, node.name()));
    }

    /// Operacion `visit add` definida por la abstraccion del ejemplo.
    fn visit_add(&mut self, node: &Add) {
        node.left().accept(self);
        node.right().accept(self);
        let (left, right) = self.pop_pair();
        self.stack
            .push(format!(r#"{{"type":"add","left":{left},"right":{right}}}"#));
    }

    /// Operacion `visit multiply` definida por la abstraccion del ejemplo.
    fn visit_multiply(&mut self, node: &Multiply) {
        node.left().accept(self);
        node.right().accept(self);
        let (left, right) = self.pop_pair();
        self.stack.push(format!(
            r#"{{"type":"multiply","left":{left},"right":{right}}}"#
        ));
    }
}
