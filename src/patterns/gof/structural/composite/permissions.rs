pub enum Permission {
    Action { module: String, action: String },
    Group(PermissionGroup),
}

impl Permission {
    pub fn action(module: impl Into<String>, action: impl Into<String>) -> Self {
        Self::Action {
            module: module.into(),
            action: action.into(),
        }
    }

    fn allows(&self, module: &str, action: &str) -> bool {
        match self {
            Self::Action {
                module: allowed_module,
                action: allowed_action,
            } => allowed_module == module && allowed_action == action,
            Self::Group(group) => group.allows(module, action),
        }
    }

    fn collect_paths(&self, prefix: &str, paths: &mut Vec<String>) {
        match self {
            Self::Action { module, action } => paths.push(format!("{prefix}/{module}:{action}")),
            Self::Group(group) => group.collect_paths(prefix, paths),
        }
    }
}

impl From<PermissionGroup> for Permission {
    fn from(group: PermissionGroup) -> Self {
        Self::Group(group)
    }
}

pub struct PermissionGroup {
    name: String,
    children: Vec<Permission>,
}

impl PermissionGroup {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            children: Vec::new(),
        }
    }

    pub fn with(mut self, permission: impl Into<Permission>) -> Self {
        self.children.push(permission.into());
        self
    }

    pub fn allows(&self, module: &str, action: &str) -> bool {
        self.children
            .iter()
            .any(|child| child.allows(module, action))
    }

    pub fn paths(&self) -> Vec<String> {
        let mut paths = Vec::new();
        self.collect_paths("", &mut paths);
        paths
    }

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
