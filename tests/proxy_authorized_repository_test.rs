use design_patterns_rust::patterns::gof::structural::proxy::authorized_repository::{
    AuthorizedDocumentRepository, DocumentRepository, UserSession,
};

#[test]
fn proxy_allows_authorized_users_to_read_sensitive_documents() {
    let repository = DocumentRepository::new();
    let proxy = AuthorizedDocumentRepository::new(repository);
    let admin = UserSession::new("ana", "admin");

    let document = proxy.find_sensitive_document(&admin, "doc-1");

    assert_eq!(document, Ok("Quarterly payroll".to_string()));
    assert_eq!(proxy.real_repository_reads(), 1);
}

#[test]
fn proxy_rejects_unauthorized_users_before_reading_repository() {
    let repository = DocumentRepository::new();
    let proxy = AuthorizedDocumentRepository::new(repository);
    let viewer = UserSession::new("leo", "viewer");

    let document = proxy.find_sensitive_document(&viewer, "doc-1");

    assert_eq!(
        document,
        Err("leo cannot read sensitive documents".to_string())
    );
    assert_eq!(proxy.real_repository_reads(), 0);
}
