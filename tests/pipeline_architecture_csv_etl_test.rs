use design_patterns_rust::patterns::architecture::pipeline_architecture::csv_etl::{
    CsvImportPipeline, ImportError, ImportedCustomer,
};

#[test]
fn pipeline_etl_imports_valid_customers_from_csv() {
    let input = "\
id,name,email
CUS-1,Ana, ANA@EXAMPLE.COM 
CUS-2,Luis,luis@example.com
";
    let pipeline = CsvImportPipeline::default();

    let report = pipeline.import(input);

    assert_eq!(
        report.imported(),
        vec![
            ImportedCustomer::new("CUS-1", "Ana", "ana@example.com"),
            ImportedCustomer::new("CUS-2", "Luis", "luis@example.com")
        ]
    );
    assert!(report.errors().is_empty());
}

#[test]
fn pipeline_etl_skips_invalid_rows_and_keeps_row_errors() {
    let input = "\
id,name,email
CUS-1,Ana,ana@example.com
,Sin Id,no-id@example.com
CUS-3,Correo Malo,correo-invalido
";
    let pipeline = CsvImportPipeline::default();

    let report = pipeline.import(input);

    assert_eq!(
        report.imported(),
        vec![ImportedCustomer::new("CUS-1", "Ana", "ana@example.com")]
    );
    assert_eq!(
        report.errors(),
        vec![
            ImportError::new(3, "id requerido"),
            ImportError::new(4, "email inválido")
        ]
    );
}

#[test]
fn pipeline_etl_rejects_input_without_required_header() {
    let input = "\
identifier,full_name,email
CUS-1,Ana,ana@example.com
";
    let pipeline = CsvImportPipeline::default();

    let report = pipeline.import(input);

    assert!(report.imported().is_empty());
    assert_eq!(
        report.errors(),
        vec![ImportError::new(1, "encabezado esperado: id,name,email")]
    );
}
