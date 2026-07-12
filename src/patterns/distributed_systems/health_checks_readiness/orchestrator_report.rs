#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentStatus {
    name: String,
    required: bool,
    ready: bool,
}

impl ComponentStatus {
    pub fn required(name: impl Into<String>, ready: bool) -> Self {
        Self {
            name: name.into(),
            required: true,
            ready,
        }
    }

    pub fn optional(name: impl Into<String>, ready: bool) -> Self {
        Self {
            name: name.into(),
            required: false,
            ready,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverallStatus {
    Ready,
    NotReady,
}

#[derive(Debug)]
pub struct OrchestratorReport {
    components: Vec<ComponentStatus>,
}

impl OrchestratorReport {
    pub fn new(components: Vec<ComponentStatus>) -> Self {
        Self { components }
    }

    pub fn overall_status(&self) -> OverallStatus {
        if self
            .components
            .iter()
            .any(|component| component.required && !component.ready)
        {
            OverallStatus::NotReady
        } else {
            OverallStatus::Ready
        }
    }

    pub fn failed_required_components(&self) -> Vec<String> {
        self.components
            .iter()
            .filter(|component| component.required && !component.ready)
            .map(|component| component.name.clone())
            .collect()
    }

    pub fn component_count(&self) -> usize {
        self.components.len()
    }
}
