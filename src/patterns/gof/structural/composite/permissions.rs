/// Conjunto de estados o errores publicos de `Permission` dentro del ejemplo.
pub enum Permission {
    /// Variante `Action` del estado o error del ejemplo.
    Action {
        /// Valor publico `module` asociado a la variante `Action`.
        module: String,
        /// Valor publico `action` asociado a la variante `Action`.
        action: String,
    },
    /// Variante `Group` del estado o error del ejemplo.
    Group(PermissionGroup),
}

impl Permission {
    /// Modela la operacion `action` dentro del ejemplo del patron.
    pub fn action(module: impl Into<String>, action: impl Into<String>) -> Self {
        Self::Action {
            module: module.into(),
            action: action.into(),
        }
    }

    /// Operacion `allows` definida por la abstraccion del ejemplo.
    fn allows(&self, module: &str, action: &str) -> bool {
        match self {
            Self::Action {
                module: allowed_module,
                action: allowed_action,
            } => allowed_module == module && allowed_action == action,
            Self::Group(group) => group.allows(module, action),
        }
    }

    /// Operacion `collect paths` definida por la abstraccion del ejemplo.
    fn collect_paths(&self, prefix: &str, paths: &mut Vec<String>) {
        match self {
            Self::Action { module, action } => paths.push(format!("{prefix}/{module}:{action}")),
            Self::Group(group) => group.collect_paths(prefix, paths),
        }
    }
}

impl From<PermissionGroup> for Permission {
    /// Operacion `from` definida por la abstraccion del ejemplo.
    fn from(group: PermissionGroup) -> Self {
        Self::Group(group)
    }
}

/// Tipo publico `PermissionGroup` usado por el ejemplo para expresar el dominio del patron.
pub struct PermissionGroup {
    name: String,
    children: Vec<Permission>,
}

impl PermissionGroup {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            children: Vec::new(),
        }
    }

    /// Modela la operacion `with` dentro del ejemplo del patron.
    pub fn with(mut self, permission: impl Into<Permission>) -> Self {
        self.children.push(permission.into());
        self
    }

    /// Modela la operacion `allows` dentro del ejemplo del patron.
    pub fn allows(&self, module: &str, action: &str) -> bool {
        self.children
            .iter()
            .any(|child| child.allows(module, action))
    }

    /// Modela la operacion `paths` dentro del ejemplo del patron.
    pub fn paths(&self) -> Vec<String> {
        let mut paths = Vec::new();
        self.collect_paths("", &mut paths);
        paths
    }

    /// Operacion `collect paths` definida por la abstraccion del ejemplo.
    fn collect_paths(&self, prefix: &str, paths: &mut Vec<String>) {
        let group_path = if prefix.is_empty() {
            self.name.clone()
        } else {
            format!("{prefix}/{}", self.name)
        };

        for child in &self.children {
            child.collect_paths(&group_path, paths);
        }
    }
}
