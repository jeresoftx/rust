#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationClass {
    Critical,
    NonCritical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulkheadError {
    ClassFull(OperationClass),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Permit {
    class: OperationClass,
}

#[derive(Debug)]
pub struct WorkflowBulkhead {
    critical_limit: usize,
    non_critical_limit: usize,
    active_critical: usize,
    active_non_critical: usize,
    critical_failures: usize,
    non_critical_failures: usize,
}

impl WorkflowBulkhead {
    pub fn new(critical_limit: usize, non_critical_limit: usize) -> Self {
        Self {
            critical_limit,
            non_critical_limit,
            active_critical: 0,
            active_non_critical: 0,
            critical_failures: 0,
            non_critical_failures: 0,
        }
    }

    pub fn acquire(&mut self, class: OperationClass) -> Result<Permit, BulkheadError> {
        match class {
            OperationClass::Critical if self.active_critical < self.critical_limit => {
                self.active_critical += 1;
                Ok(Permit { class })
            }
            OperationClass::NonCritical if self.active_non_critical < self.non_critical_limit => {
                self.active_non_critical += 1;
                Ok(Permit { class })
            }
            _ => Err(BulkheadError::ClassFull(class)),
        }
    }

    pub fn release(&mut self, permit: Permit) {
        match permit.class {
            OperationClass::Critical => {
                self.active_critical = self.active_critical.saturating_sub(1);
            }
            OperationClass::NonCritical => {
                self.active_non_critical = self.active_non_critical.saturating_sub(1);
            }
        }
    }

    pub fn record_failure(&mut self, class: OperationClass) {
        match class {
            OperationClass::Critical => self.critical_failures += 1,
            OperationClass::NonCritical => self.non_critical_failures += 1,
        }
    }

    pub fn active(&self, class: OperationClass) -> usize {
        match class {
            OperationClass::Critical => self.active_critical,
            OperationClass::NonCritical => self.active_non_critical,
        }
    }

    pub fn failures(&self, class: OperationClass) -> usize {
        match class {
            OperationClass::Critical => self.critical_failures,
            OperationClass::NonCritical => self.non_critical_failures,
        }
    }
}
