use std::cell::Cell;
use std::collections::HashMap;

/// Tipo publico `UserSession` usado por el ejemplo para expresar el dominio del patron.
pub struct UserSession {
    username: String,
    role: String,
}

impl UserSession {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(username: impl Into<String>, role: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            role: role.into(),
        }
    }

    /// Operacion `can read sensitive documents` definida por la abstraccion del ejemplo.
    fn can_read_sensitive_documents(&self) -> bool {
        self.role == "admin"
    }
}

/// Tipo publico `DocumentRepository` usado por el ejemplo para expresar el dominio del patron.
pub struct DocumentRepository {
    documents: HashMap<String, String>,
    reads: Cell<usize>,
}

impl DocumentRepository {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self {
            documents: HashMap::from([("doc-1".to_string(), "Quarterly payroll".to_string())]),
            reads: Cell::new(0),
        }
    }

    /// Operacion `find` definida por la abstraccion del ejemplo.
    fn find(&self, document_id: &str) -> Option<String> {
        self.reads.set(self.reads.get() + 1);
        self.documents.get(document_id).cloned()
    }

    /// Operacion `reads` definida por la abstraccion del ejemplo.
    fn reads(&self) -> usize {
        self.reads.get()
    }
}

impl Default for DocumentRepository {
    /// Operacion `default` definida por la abstraccion del ejemplo.
    fn default() -> Self {
        Self::new()
    }
}

/// Tipo publico `AuthorizedDocumentRepository` usado por el ejemplo para expresar el dominio del patron.
pub struct AuthorizedDocumentRepository {
    repository: DocumentRepository,
}

impl AuthorizedDocumentRepository {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(repository: DocumentRepository) -> Self {
        Self { repository }
    }

    /// Modela la operacion `find sensitive document` dentro del ejemplo del patron.
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

    /// Modela la operacion `real repository reads` dentro del ejemplo del patron.
    pub fn real_repository_reads(&self) -> usize {
        self.repository.reads()
    }
}
