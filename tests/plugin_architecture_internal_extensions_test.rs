use design_patterns_rust::patterns::architecture::plugin_architecture::internal_extensions::{
    AuditExtension, RequestContext, RequestProcessor, TenantHeaderExtension, TraceIdExtension,
};

#[test]
fn processor_runs_internal_extensions_in_order() {
    let mut processor = RequestProcessor::default();
    processor.add_extension(Box::new(TraceIdExtension::new("trace-123")));
    processor.add_extension(Box::new(TenantHeaderExtension::new("tenant-a")));
    processor.add_extension(Box::new(AuditExtension::new("accepted")));

    let context = processor.process(RequestContext::new("/orders"));

    assert_eq!(context.path(), "/orders");
    assert_eq!(
        context.headers(),
        vec![
            ("x-trace-id".to_string(), "trace-123".to_string()),
            ("x-tenant".to_string(), "tenant-a".to_string())
        ]
    );
    assert_eq!(context.audit_log(), vec!["accepted:/orders".to_string()]);
}

#[test]
fn processor_can_run_without_extensions() {
    let processor = RequestProcessor::default();

    let context = processor.process(RequestContext::new("/health"));

    assert_eq!(context.path(), "/health");
    assert!(context.headers().is_empty());
    assert!(context.audit_log().is_empty());
}

#[test]
fn processor_allows_reusing_extension_contract_for_custom_ordering() {
    let mut processor = RequestProcessor::default();
    processor.add_extension(Box::new(TenantHeaderExtension::new("tenant-b")));
    processor.add_extension(Box::new(TraceIdExtension::new("trace-999")));

    let context = processor.process(RequestContext::new("/reports"));

    assert_eq!(
        context.headers(),
        vec![
            ("x-tenant".to_string(), "tenant-b".to_string()),
            ("x-trace-id".to_string(), "trace-999".to_string())
        ]
    );
}
