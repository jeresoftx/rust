use design_patterns_rust::patterns::gof::behavioral::interpreter::permission_language::{
    PermissionExpression, PermissionSet,
};

#[test]
fn interpreter_evaluates_role_and_scope_permission_expression() {
    let expression =
        PermissionExpression::parse("role:admin AND scope:orders.write").expect("valid rule");
    let permissions = PermissionSet::new("admin", ["orders.write", "orders.read"]);

    assert!(expression.allows(&permissions));
}

#[test]
fn interpreter_evaluates_alternative_permission_expression() {
    let expression =
        PermissionExpression::parse("role:owner OR scope:billing.read").expect("valid rule");
    let permissions = PermissionSet::new("support", ["billing.read"]);

    assert!(expression.allows(&permissions));
}
