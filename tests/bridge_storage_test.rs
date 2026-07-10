use design_patterns_rust::patterns::gof::structural::bridge::storage::{
    CloudStorageProvider, DocumentStore, LocalStorageProvider,
};

#[test]
fn bridge_saves_the_same_document_to_different_storage_providers() {
    let local_store = DocumentStore::new(LocalStorageProvider::new("/var/app/docs"));
    let cloud_store = DocumentStore::new(CloudStorageProvider::new("s3://company-docs"));

    assert_eq!(
        local_store.save("invoice-1.txt", "paid"),
        "local path=/var/app/docs/invoice-1.txt bytes=4"
    );
    assert_eq!(
        cloud_store.save("invoice-1.txt", "paid"),
        "cloud bucket=s3://company-docs key=invoice-1.txt bytes=4"
    );
}

#[test]
fn bridge_keeps_document_store_independent_from_provider() {
    let store = DocumentStore::new(LocalStorageProvider::new("/tmp/archive"));

    assert_eq!(
        store.save("audit.log", "ok"),
        "local path=/tmp/archive/audit.log bytes=2"
    );
}
