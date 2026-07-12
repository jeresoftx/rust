use design_patterns_rust::patterns::architecture::plugin_architecture::export_plugins::{
    ExportError, ExportRecord, ExportRegistry,
};

#[test]
fn plugin_registry_exports_records_as_json() {
    let registry = ExportRegistry::with_default_plugins();
    let records = vec![
        ExportRecord::new("ORD-1", 2_500),
        ExportRecord::new("ORD-2", 1_000),
    ];

    let output = registry
        .export("json", &records)
        .expect("json plugin should be registered");

    assert_eq!(
        output,
        r#"[{"id":"ORD-1","amount_cents":2500},{"id":"ORD-2","amount_cents":1000}]"#
    );
}

#[test]
fn plugin_registry_exports_records_as_csv_and_text() {
    let registry = ExportRegistry::with_default_plugins();
    let records = vec![
        ExportRecord::new("ORD-1", 2_500),
        ExportRecord::new("ORD-2", 1_000),
    ];

    assert_eq!(
        registry.export("csv", &records),
        Ok("id,amount_cents\nORD-1,2500\nORD-2,1000".to_string())
    );
    assert_eq!(
        registry.export("text", &records),
        Ok("ORD-1: $25.00\nORD-2: $10.00".to_string())
    );
}

#[test]
fn plugin_registry_reports_unknown_export_plugin() {
    let registry = ExportRegistry::with_default_plugins();
    let records = vec![ExportRecord::new("ORD-1", 2_500)];

    let result = registry.export("xml", &records);

    assert_eq!(result, Err(ExportError::PluginNotFound("xml".to_string())));
}
