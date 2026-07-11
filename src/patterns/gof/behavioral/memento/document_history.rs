#[derive(Debug, Clone, PartialEq, Eq)]
struct DocumentState {
    title: String,
    body: String,
    tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentSnapshot {
    state: DocumentState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentEditor {
    state: DocumentState,
}

impl DocumentEditor {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            state: DocumentState {
                title: title.into(),
                body: String::new(),
                tags: Vec::new(),
            },
        }
    }

    pub fn save(&self) -> DocumentSnapshot {
        DocumentSnapshot {
            state: self.state.clone(),
        }
    }

    pub fn restore(&mut self, snapshot: DocumentSnapshot) {
        self.state = snapshot.state;
    }

    pub fn rename(&mut self, title: impl Into<String>) {
        self.state.title = title.into();
    }

    pub fn replace_body(&mut self, body: impl Into<String>) {
        self.state.body = body.into();
    }

    pub fn add_tag(&mut self, tag: impl Into<String>) {
        self.state.tags.push(tag.into());
    }

    pub fn title(&self) -> &str {
        &self.state.title
    }

    pub fn body(&self) -> &str {
        &self.state.body
    }

    pub fn tags(&self) -> Vec<String> {
        self.state.tags.clone()
    }
}

#[derive(Debug, Default, Clone)]
pub struct DocumentHistory {
    snapshots: Vec<DocumentSnapshot>,
}

impl DocumentHistory {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
        }
    }

    pub fn checkpoint(&mut self, editor: &DocumentEditor) {
        self.snapshots.push(editor.save());
    }

    pub fn undo(&mut self, editor: &mut DocumentEditor) -> bool {
        if let Some(snapshot) = self.snapshots.pop() {
            editor.restore(snapshot);
            true
        } else {
            false
        }
    }
}
