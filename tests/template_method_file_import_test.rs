use design_patterns_rust::patterns::gof::behavioral::template_method::file_import::{
    CsvLeadImporter, FileImporter, PipeOrderImporter,
};

#[test]
fn template_method_imports_csv_leads_with_shared_pipeline() {
    let importer = CsvLeadImporter;

    let report = importer.import("Ana,ana@example.com\nBob,bob@example.com");

    assert_eq!(report.saved_records(), 2);
    assert_eq!(
        report.steps(),
        [
            "read:csv_leads",
            "parse:2",
            "normalize:2",
            "validate:2",
            "save:leads:2"
        ]
    );
}

#[test]
fn template_method_imports_pipe_orders_with_custom_parser() {
    let importer = PipeOrderImporter;

    let report = importer.import("ORD-1|1200\nORD-2|800\nORD-3|250");

    assert_eq!(report.saved_records(), 3);
    assert_eq!(
        report.steps(),
        [
            "read:pipe_orders",
            "parse:3",
            "normalize:3",
            "validate:3",
            "save:orders:3"
        ]
    );
}

#[test]
fn template_method_rejects_rows_missing_required_fields() {
    let importer = CsvLeadImporter;

    let report = importer.import("Ana,ana@example.com\nIncomplete");

    assert_eq!(report.saved_records(), 0);
    assert_eq!(
        report.steps(),
        [
            "read:csv_leads",
            "parse:2",
            "normalize:2",
            "validate:failed"
        ]
    );
}
