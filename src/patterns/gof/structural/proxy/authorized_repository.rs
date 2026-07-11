use std::cell::Cell;
use std::collections::HashMap;

pub struct UserSession {
    username: String,
    role: String,
}

impl UserSession {
    pub fn new(username: impl Into<String>, role: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            role: role.into(),
        }
    }

    fn can_read_sensitive_documents(&self) -> bool {
        self.role == "admin"
    }
}

pub struct DocumentRepository {
    documents: HashMap<String, String>,
    reads: Cell<usize>,
}

impl DocumentRepository {
    pub fn new() -> Self {
        Self {
            documents: HashMap::from([("doc-1".to_string(), "Quarterly payroll".to_string())]),
            reads: Cell::new(0),
        }
    }

    fn find(&self, document_id: &str) -> Option<String> {
        self.reads.set(self.reads.get() + 1);
        self.documents.get(document_id).cloned()
    }

    fn reads(&self) -> usize {
        self.reads.get()
    }
}

impl Default for DocumentRepository {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AuthorizedDocumentRepository {
    repository: DocumentRepository,
}

impl AuthorizedDocumentRepository {
    pub fn new(repository: DocumentRepository) -> Self {
        Self { repository }
    }

    pub fn find_sensitive_document(
        &self,
        session: &UserSession,
        document_id: &str,
    ) -> Result<String, String> {
        if !session.can_read_sensitive_documents() {
            return Err(format!(
                "{} cannot read sensitive documents",
                session.username
            ));
        }

        self.repository
            .find(document_id)
            .ok_or_else(|| format!("{document_id} was not found"))
    }

    pub fn real_repository_reads(&self) -> usize {
        self.repository.reads()
    }
}
