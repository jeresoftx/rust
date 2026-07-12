#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestContext {
    path: String,
    headers: Vec<(String, String)>,
    audit_log: Vec<String>,
}

impl RequestContext {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            headers: Vec::new(),
            audit_log: Vec::new(),
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn headers(&self) -> Vec<(String, String)> {
        self.headers.clone()
    }

    pub fn audit_log(&self) -> Vec<String> {
        self.audit_log.clone()
    }

    fn add_header(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.headers.push((key.into(), value.into()));
    }

    fn add_audit_entry(&mut self, entry: impl Into<String>) {
        self.audit_log.push(entry.into());
    }
}

pub trait RequestExtension {
    fn apply(&self, context: &mut RequestContext);
}

#[derive(Default)]
pub struct RequestProcessor {
    extensions: Vec<Box<dyn RequestExtension>>,
}

impl RequestProcessor {
    pub fn add_extension(&mut self, extension: Box<dyn RequestExtension>) {
        self.extensions.push(extension);
    }

    pub fn process(&self, mut context: RequestContext) -> RequestContext {
        for extension in &self.extensions {
            extension.apply(&mut context);
        }

        context
    }
}

pub struct TraceIdExtension {
    trace_id: String,
}

impl TraceIdExtension {
    pub fn new(trace_id: impl Into<String>) -> Self {
        Self {
            trace_id: trace_id.into(),
        }
    }
}

impl RequestExtension for TraceIdExtension {
    fn apply(&self, context: &mut RequestContext) {
        context.add_header("x-trace-id", &self.trace_id);
    }
}

pub struct TenantHeaderExtension {
    tenant_id: String,
}

impl TenantHeaderExtension {
    pub fn new(tenant_id: impl Into<String>) -> Self {
        Self {
            tenant_id: tenant_id.into(),
        }
    }
}

impl RequestExtension for TenantHeaderExtension {
    fn apply(&self, context: &mut RequestContext) {
        context.add_header("x-tenant", &self.tenant_id);
    }
}

pub struct AuditExtension {
    status: String,
}

impl AuditExtension {
    pub fn new(status: impl Into<String>) -> Self {
        Self {
            status: status.into(),
        }
    }
}

impl RequestExtension for AuditExtension {
    fn apply(&self, context: &mut RequestContext) {
        context.add_audit_entry(format!("{}:{}", self.status, context.path()));
    }
}
