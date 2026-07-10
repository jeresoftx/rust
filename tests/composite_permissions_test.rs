use design_patterns_rust::patterns::gof::structural::composite::permissions::{
    Permission, PermissionGroup,
};

#[test]
fn composite_checks_permissions_across_nested_groups() {
    let billing = PermissionGroup::new("billing")
        .with(Permission::action("invoices", "read"))
        .with(Permission::action("invoices", "pay"));
    let users = PermissionGroup::new("users").with(Permission::action("profiles", "read"));
    let role = PermissionGroup::new("admin").with(billing).with(users);

    assert!(role.allows("invoices", "pay"));
    assert!(role.allows("profiles", "read"));
    assert!(!role.allows("profiles", "delete"));
}

#[test]
fn composite_lists_permission_tree_as_paths() {
    let role = PermissionGroup::new("support")
        .with(Permission::action("tickets", "read"))
        .with(PermissionGroup::new("billing").with(Permission::action("refunds", "create")));

    assert_eq!(
        role.paths(),
        vec![
            "support/tickets:read".to_string(),
            "support/billing/refunds:create".to_string(),
        ]
    );
}
