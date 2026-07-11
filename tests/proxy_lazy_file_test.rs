use design_patterns_rust::patterns::gof::structural::proxy::lazy_file::{LargeFile, LazyFileProxy};

#[test]
fn proxy_delays_loading_large_file_until_content_is_requested() {
    let file = LargeFile::new("audit.log", "entry-1\nentry-2");
    let mut proxy = LazyFileProxy::new(file);

    assert_eq!(proxy.load_count(), 0);

    assert_eq!(proxy.content(), "entry-1\nentry-2");
    assert_eq!(proxy.load_count(), 1);
}

#[test]
fn proxy_reuses_loaded_file_content_on_later_reads() {
    let file = LargeFile::new("report.csv", "id,total\n1,99");
    let mut proxy = LazyFileProxy::new(file);

    assert_eq!(proxy.content(), "id,total\n1,99");
    assert_eq!(proxy.content(), "id,total\n1,99");

    assert_eq!(proxy.file_name(), "report.csv");
    assert_eq!(proxy.load_count(), 1);
}
