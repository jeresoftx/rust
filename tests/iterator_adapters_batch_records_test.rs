use design_patterns_rust::patterns::rust_idiomatic::iterator_adapters::batch_records::{
    RawRecord, process_record_batches,
};

#[test]
fn iterator_adapters_process_records_in_batches() {
    let records = vec![
        RawRecord::new("  user-1, created  "),
        RawRecord::new("   "),
        RawRecord::new("user-2,updated"),
        RawRecord::new("user-3,deleted"),
    ];

    let batches = process_record_batches(&records, 2);

    assert_eq!(batches.len(), 2);
    assert_eq!(batches[0].summary(), "batch 1: 1 valid records");
    assert_eq!(batches[0].events(), ["user-1:created"]);
    assert_eq!(batches[1].summary(), "batch 2: 2 valid records");
    assert_eq!(batches[1].events(), ["user-2:updated", "user-3:deleted"]);
}

#[test]
fn iterator_adapters_skips_batches_without_valid_records() {
    let records = vec![
        RawRecord::new("missing comma"),
        RawRecord::new(" , "),
        RawRecord::new("user-9,created"),
    ];

    let batches = process_record_batches(&records, 2);

    assert_eq!(batches.len(), 1);
    assert_eq!(batches[0].batch_number(), 2);
    assert_eq!(batches[0].events(), ["user-9:created"]);
}

#[test]
fn iterator_adapters_returns_no_batches_when_batch_size_is_zero() {
    let records = vec![RawRecord::new("user-1,created")];

    assert!(process_record_batches(&records, 0).is_empty());
}
