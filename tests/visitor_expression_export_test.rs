use design_patterns_rust::patterns::gof::behavioral::visitor::expression_export::{
    Add, JsonExportVisitor, Multiply, Number, TextExportVisitor, Variable, VisitableExpression,
};

fn sample_expression() -> Multiply {
    Multiply::new(
        Add::new(Number::new(2), Variable::new("price")),
        Add::new(Number::new(3), Number::new(4)),
    )
}

#[test]
fn visitor_exports_expression_tree_to_text() {
    let expression = sample_expression();
    let mut visitor = TextExportVisitor::default();

    expression.accept(&mut visitor);

    assert_eq!(visitor.output(), "((2 + price) * (3 + 4))");
}

#[test]
fn visitor_exports_expression_tree_to_json() {
    let expression = sample_expression();
    let mut visitor = JsonExportVisitor::default();

    expression.accept(&mut visitor);

    assert_eq!(
        visitor.output(),
        r#"{"type":"multiply","left":{"type":"add","left":{"type":"number","value":2},"right":{"type":"variable","name":"price"}},"right":{"type":"add","left":{"type":"number","value":3},"right":{"type":"number","value":4}}}"#
    );
}

#[test]
fn visitor_reuses_exporters_for_smaller_expression() {
    let expression = Add::new(Variable::new("tax"), Number::new(5));
    let mut visitor = TextExportVisitor::default();

    expression.accept(&mut visitor);

    assert_eq!(visitor.output(), "(tax + 5)");
}
