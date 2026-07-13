#[derive(Debug, Clone, PartialEq, Eq)]
struct DocumentState {
    title: String,
    body: String,
    tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `DocumentSnapshot` usado por el ejemplo para expresar el dominio del patron.
pub struct DocumentSnapshot {
    state: DocumentState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `DocumentEditor` usado por el ejemplo para expresar el dominio del patron.
pub struct DocumentEditor {
    state: DocumentState,
}

impl DocumentEditor {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            state: DocumentState {
                title: title.into(),
                body: String::new(),
                tags: Vec::new(),
            },
        }
    }

    /// Modela la operacion `save` dentro del ejemplo del patron.
    pub fn save(&self) -> DocumentSnapshot {
        DocumentSnapshot {
            state: self.state.clone(),
        }
    }

    /// Modela la operacion `restore` dentro del ejemplo del patron.
    pub fn restore(&mut self, snapshot: DocumentSnapshot) {
        self.state = snapshot.state;
    }

    /// Modela la operacion `rename` dentro del ejemplo del patron.
    pub fn rename(&mut self, title: impl Into<String>) {
        self.state.title = title.into();
    }

    /// Modela la operacion `replace body` dentro del ejemplo del patron.
    pub fn replace_body(&mut self, body: impl Into<String>) {
        self.state.body = body.into();
    }

    /// Modela la operacion `add tag` dentro del ejemplo del patron.
    pub fn add_tag(&mut self, tag: impl Into<String>) {
        self.state.tags.push(tag.into());
    }

    /// Modela la operacion `title` dentro del ejemplo del patron.
    pub fn title(&self) -> &str {
        &self.state.title
    }

    /// Modela la operacion `body` dentro del ejemplo del patron.
    pub fn body(&self) -> &str {
        &self.state.body
    }

    /// Modela la operacion `tags` dentro del ejemplo del patron.
    pub fn tags(&self) -> Vec<String> {
        self.state.tags.clone()
    }
}

#[derive(Debug, Default, Clone)]
/// Tipo publico `DocumentHistory` usado por el ejemplo para expresar el dominio del patron.
pub struct DocumentHistory {
    snapshots: Vec<DocumentSnapshot>,
}

impl DocumentHistory {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
        }
    }

    /// Modela la operacion `checkpoint` dentro del ejemplo del patron.
    pub fn checkpoint(&mut self, editor: &DocumentEditor) {
        self.snapshots.push(editor.save());
    }

    /// Modela la operacion `undo` dentro del ejemplo del patron.
    pub fn undo(&mut self, editor: &mut DocumentEditor) -> bool {
        if let Some(snapshot) = self.snapshots.pop() {
            editor.restore(snapshot);
            true
        } else {
            false
        }
    }
}
