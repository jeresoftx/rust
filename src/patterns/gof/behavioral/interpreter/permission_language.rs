use std::collections::HashSet;

pub struct PermissionSet {
    role: String,
    scopes: HashSet<String>,
}

impl PermissionSet {
    pub fn new<const N: usize>(role: impl Into<String>, scopes: [&str; N]) -> Self {
        Self {
            role: role.into(),
            scopes: scopes.into_iter().map(str::to_string).collect(),
        }
    }
}

pub enum PermissionExpression {
    Role(String),
    Scope(String),
    And(Box<PermissionExpression>, Box<PermissionExpression>),
    Or(Box<PermissionExpression>, Box<PermissionExpression>),
}

impl PermissionExpression {
    pub fn parse(input: &str) -> Result<Self, String> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.as_slice() {
            [single] => Self::parse_term(single),
            [left, "AND", right] => Ok(Self::And(
                Box::new(Self::parse_term(left)?),
                Box::new(Self::parse_term(right)?),
            )),
            [left, "OR", right] => Ok(Self::Or(
                Box::new(Self::parse_term(left)?),
                Box::new(Self::parse_term(right)?),
            )),
            _ => Err("permission expression must be a term or binary expression".to_string()),
        }
    }

    pub fn allows(&self, permissions: &PermissionSet) -> bool {
        match self {
            Self::Role(role) => permissions.role == *role,
            Self::Scope(scope) => permissions.scopes.contains(scope),
            Self::And(left, right) => left.allows(permissions) && right.allows(permissions),
            Self::Or(left, right) => left.allows(permissions) || right.allows(permissions),
        }
    }

    fn parse_term(input: &str) -> Result<Self, String> {
        if let Some(role) = input.strip_prefix("role:") {
            Ok(Self::Role(role.to_string()))
        } else if let Some(scope) = input.strip_prefix("scope:") {
            Ok(Self::Scope(scope.to_string()))
        } else {
            Err(format!("unsupported permission term {input}"))
        }
    }
}
