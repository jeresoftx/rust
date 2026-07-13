use std::cell::{Cell, RefCell};

#[derive(Debug)]
/// Tipo publico `CriticalSection` usado por el ejemplo para expresar el dominio del patron.
pub struct CriticalSection {
    name: String,
    locked: Cell<bool>,
    audit_log: RefCell<Vec<String>>,
}

#[derive(Debug)]
/// Tipo publico `SectionGuard` usado por el ejemplo para expresar el dominio del patron.
pub struct SectionGuard<'a> {
    section: &'a CriticalSection,
}

impl CriticalSection {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            locked: Cell::new(false),
            audit_log: RefCell::new(Vec::new()),
        }
    }

    /// Modela la operacion `enter` dentro del ejemplo del patron.
    pub fn enter(&self) -> Option<SectionGuard<'_>> {
        if self.locked.replace(true) {
            self.locked.set(true);
            return None;
        }

        Some(SectionGuard { section: self })
    }

    /// Modela la operacion `is locked` dentro del ejemplo del patron.
    pub fn is_locked(&self) -> bool {
        self.locked.get()
    }

    /// Modela la operacion `audit log` dentro del ejemplo del patron.
    pub fn audit_log(&self) -> Vec<String> {
        self.audit_log.borrow().clone()
    }
}

impl SectionGuard<'_> {
    /// Modela la operacion `record` dentro del ejemplo del patron.
    pub fn record(&mut self, event: impl Into<String>) {
        self.section
            .audit_log
            .borrow_mut()
            .push(format!("{}:{}", self.section.name, event.into()));
    }
}

impl Drop for SectionGuard<'_> {
    /// Operacion `drop` definida por la abstraccion del ejemplo.
    fn drop(&mut self) {
        self.section.locked.set(false);
        self.section
            .audit_log
            .borrow_mut()
            .push(format!("{}:released", self.section.name));
    }
}
