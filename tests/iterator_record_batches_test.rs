use design_patterns_rust::patterns::gof::behavioral::iterator::record_batches::RecordBatcher;

#[test]
fn iterator_groups_records_into_fixed_size_batches() {
    let batcher = RecordBatcher::new(vec![1, 2, 3, 4, 5], 2);

    let batches: Vec<Vec<u32>> = batcher.collect();

    assert_eq!(batches, vec![vec![1, 2], vec![3, 4], vec![5]]);
}

#[test]
fn iterator_rejects_zero_batch_size() {
    let result = RecordBatcher::try_new(vec![1, 2, 3], 0);

    assert_eq!(
        result.err(),
        Some("batch size must be greater than zero".to_string())
    );
}
