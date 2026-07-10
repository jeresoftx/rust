use design_patterns_rust::patterns::gof::creational::factory_method::file_processors::{
    ProcessorError, process_file,
};

#[test]
fn factory_method_creates_a_csv_processor_from_extension() {
    let result = process_file("orders.csv", "id,total\n1,25.99\n2,10.00").expect("valid csv");

    assert_eq!(result, "csv rows=2 columns=2");
}

#[test]
fn factory_method_creates_a_json_processor_from_extension() {
    let result =
        process_file("orders.json", r#"[{"id":1},{"id":2},{"id":3}]"#).expect("valid json");

    assert_eq!(result, "json objects=3");
}

#[test]
fn factory_method_creates_an_xml_processor_from_extension() {
    let result =
        process_file("orders.xml", "<orders><order/><order/></orders>").expect("valid xml");

    assert_eq!(result, "xml elements=2");
}

#[test]
fn factory_method_rejects_unknown_file_extension() {
    let error = process_file("orders.txt", "plain text").expect_err("txt is unsupported");

    assert_eq!(
        error,
        ProcessorError::UnsupportedExtension("txt".to_string())
    );
}
