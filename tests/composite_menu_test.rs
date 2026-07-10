use design_patterns_rust::patterns::gof::structural::composite::menu::{MenuItem, MenuSection};

#[test]
fn composite_renders_nested_menu_items() {
    let menu = MenuSection::new("Admin")
        .with(MenuItem::link("Users", "/admin/users"))
        .with(MenuSection::new("Billing").with(MenuItem::link("Invoices", "/admin/invoices")));

    assert_eq!(
        menu.render(),
        vec![
            "Admin".to_string(),
            "  Users -> /admin/users".to_string(),
            "  Billing".to_string(),
            "    Invoices -> /admin/invoices".to_string(),
        ]
    );
}

#[test]
fn composite_finds_urls_inside_submenus() {
    let menu = MenuSection::new("Root")
        .with(MenuItem::link("Dashboard", "/dashboard"))
        .with(MenuSection::new("Reports").with(MenuItem::link("Sales", "/reports/sales")));

    assert_eq!(menu.find_url("Sales"), Some("/reports/sales".to_string()));
    assert_eq!(menu.find_url("Missing"), None);
}
