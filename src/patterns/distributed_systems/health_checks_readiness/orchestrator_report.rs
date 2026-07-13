#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ComponentStatus` usado por el ejemplo para expresar el dominio del patron.
pub struct ComponentStatus {
    name: String,
    required: bool,
    ready: bool,
}

impl ComponentStatus {
    /// Modela la operacion `required` dentro del ejemplo del patron.
    pub fn required(name: impl Into<String>, ready: bool) -> Self {
        Self {
            name: name.into(),
            required: true,
            ready,
        }
    }

    /// Modela la operacion `optional` dentro del ejemplo del patron.
    pub fn optional(name: impl Into<String>, ready: bool) -> Self {
        Self {
            name: name.into(),
            required: false,
            ready,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `OverallStatus` dentro del ejemplo.
pub enum OverallStatus {
    /// Variante `Ready` del estado o error del ejemplo.
    Ready,
    /// Variante `NotReady` del estado o error del ejemplo.
    NotReady,
}

#[derive(Debug)]
/// Tipo publico `OrchestratorReport` usado por el ejemplo para expresar el dominio del patron.
pub struct OrchestratorReport {
    components: Vec<ComponentStatus>,
}

impl OrchestratorReport {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(components: Vec<ComponentStatus>) -> Self {
        Self { components }
    }

    /// Modela la operacion `overall status` dentro del ejemplo del patron.
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

    /// Modela la operacion `failed required components` dentro del ejemplo del patron.
    pub fn failed_required_components(&self) -> Vec<String> {
        self.components
            .iter()
            .filter(|component| component.required && !component.ready)
            .map(|component| component.name.clone())
            .collect()
    }

    /// Modela la operacion `component count` dentro del ejemplo del patron.
    pub fn component_count(&self) -> usize {
        self.components.len()
    }
}
