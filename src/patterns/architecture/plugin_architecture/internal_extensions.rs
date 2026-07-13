#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `RequestContext` usado por el ejemplo para expresar el dominio del patron.
pub struct RequestContext {
    path: String,
    headers: Vec<(String, String)>,
    audit_log: Vec<String>,
}

impl RequestContext {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            headers: Vec::new(),
            audit_log: Vec::new(),
        }
    }

    /// Modela la operacion `path` dentro del ejemplo del patron.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Modela la operacion `headers` dentro del ejemplo del patron.
    pub fn headers(&self) -> Vec<(String, String)> {
        self.headers.clone()
    }

    /// Modela la operacion `audit log` dentro del ejemplo del patron.
    pub fn audit_log(&self) -> Vec<String> {
        self.audit_log.clone()
    }

    /// Operacion `add header` definida por la abstraccion del ejemplo.
    fn add_header(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.headers.push((key.into(), value.into()));
    }

    /// Operacion `add audit entry` definida por la abstraccion del ejemplo.
    fn add_audit_entry(&mut self, entry: impl Into<String>) {
        self.audit_log.push(entry.into());
    }
}

/// Contrato publico `RequestExtension` que desacopla las piezas del ejemplo.
pub trait RequestExtension {
    /// Operacion `apply` definida por la abstraccion del ejemplo.
    fn apply(&self, context: &mut RequestContext);
}

#[derive(Default)]
/// Tipo publico `RequestProcessor` usado por el ejemplo para expresar el dominio del patron.
pub struct RequestProcessor {
    extensions: Vec<Box<dyn RequestExtension>>,
}

impl RequestProcessor {
    /// Modela la operacion `add extension` dentro del ejemplo del patron.
    pub fn add_extension(&mut self, extension: Box<dyn RequestExtension>) {
        self.extensions.push(extension);
    }

    /// Modela la operacion `process` dentro del ejemplo del patron.
    pub fn process(&self, mut context: RequestContext) -> RequestContext {
        for extension in &self.extensions {
            extension.apply(&mut context);
        }

        context
    }
}

/// Tipo publico `TraceIdExtension` usado por el ejemplo para expresar el dominio del patron.
pub struct TraceIdExtension {
    trace_id: String,
}

impl TraceIdExtension {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(trace_id: impl Into<String>) -> Self {
        Self {
            trace_id: trace_id.into(),
        }
    }
}

impl RequestExtension for TraceIdExtension {
    /// Operacion `apply` definida por la abstraccion del ejemplo.
    fn apply(&self, context: &mut RequestContext) {
        context.add_header("x-trace-id", &self.trace_id);
    }
}

/// Tipo publico `TenantHeaderExtension` usado por el ejemplo para expresar el dominio del patron.
pub struct TenantHeaderExtension {
    tenant_id: String,
}

impl TenantHeaderExtension {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(tenant_id: impl Into<String>) -> Self {
        Self {
            tenant_id: tenant_id.into(),
        }
    }
}

impl RequestExtension for TenantHeaderExtension {
    /// Operacion `apply` definida por la abstraccion del ejemplo.
    fn apply(&self, context: &mut RequestContext) {
        context.add_header("x-tenant", &self.tenant_id);
    }
}

/// Tipo publico `AuditExtension` usado por el ejemplo para expresar el dominio del patron.
pub struct AuditExtension {
    status: String,
}

impl AuditExtension {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(status: impl Into<String>) -> Self {
        Self {
            status: status.into(),
        }
    }
}

impl RequestExtension for AuditExtension {
    /// Operacion `apply` definida por la abstraccion del ejemplo.
    fn apply(&self, context: &mut RequestContext) {
        context.add_audit_entry(format!("{}:{}", self.status, context.path()));
    }
}
