use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct CriticalSection {
    name: String,
    locked: Cell<bool>,
    audit_log: RefCell<Vec<String>>,
}

#[derive(Debug)]
pub struct SectionGuard<'a> {
    section: &'a CriticalSection,
}

impl CriticalSection {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            locked: Cell::new(false),
            audit_log: RefCell::new(Vec::new()),
        }
    }

    pub fn enter(&self) -> Option<SectionGuard<'_>> {
        if self.locked.replace(true) {
            self.locked.set(true);
            return None;
        }

        Some(SectionGuard { section: self })
    }

    pub fn is_locked(&self) -> bool {
        self.locked.get()
    }

    pub fn audit_log(&self) -> Vec<String> {
        self.audit_log.borrow().clone()
    }
}

impl SectionGuard<'_> {
    pub fn record(&mut self, event: impl Into<String>) {
        self.section
            .audit_log
            .borrow_mut()
            .push(format!("{}:{}", self.section.name, event.into()));
    }
}

impl Drop for SectionGuard<'_> {
    fn drop(&mut self) {
        self.section.locked.set(false);
        self.section
            .audit_log
            .borrow_mut()
            .push(format!("{}:released", self.section.name));
    }
}
