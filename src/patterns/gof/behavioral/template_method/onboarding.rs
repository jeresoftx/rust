#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `OnboardingPlan` usado por el ejemplo para expresar el dominio del patron.
pub struct OnboardingPlan {
    steps: Vec<String>,
}

impl OnboardingPlan {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(steps: Vec<String>) -> Self {
        Self { steps }
    }

    /// Modela la operacion `steps` dentro del ejemplo del patron.
    pub fn steps(&self) -> Vec<&str> {
        self.steps.iter().map(String::as_str).collect()
    }
}

/// Contrato publico `OnboardingTemplate` que desacopla las piezas del ejemplo.
pub trait OnboardingTemplate {
    /// Operacion `email` definida por la abstraccion del ejemplo.
    fn email(&self) -> &str;
    /// Operacion `account type` definida por la abstraccion del ejemplo.
    fn account_type(&self) -> &'static str;

    /// Operacion `is valid` definida por la abstraccion del ejemplo.
    fn is_valid(&self) -> bool {
        self.email().contains('@')
    }

    /// Operacion `base provisioning` definida por la abstraccion del ejemplo.
    fn base_provisioning(&self) -> Vec<&'static str> {
        vec!["workspace", "billing_profile"]
    }

    /// Operacion `hooks` definida por la abstraccion del ejemplo.
    fn hooks(&self) -> Vec<&'static str> {
        vec!["send_welcome_email"]
    }

    /// Operacion `onboard` definida por la abstraccion del ejemplo.
    fn onboard(&self) -> OnboardingPlan {
        if !self.is_valid() {
            return OnboardingPlan::new(vec!["validate:failed".to_string()]);
        }

        let mut steps = vec![
            format!("validate:{}", self.email()),
            format!("create_account:{}", self.account_type()),
        ];

        steps.extend(
            self.base_provisioning()
                .into_iter()
                .map(|step| format!("provision:{step}")),
        );
        steps.extend(self.hooks().into_iter().map(|hook| format!("hook:{hook}")));
        steps.push(format!("complete:{}", self.email()));

        OnboardingPlan::new(steps)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `StarterOnboarding` usado por el ejemplo para expresar el dominio del patron.
pub struct StarterOnboarding {
    email: String,
}

impl StarterOnboarding {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
        }
    }
}

impl OnboardingTemplate for StarterOnboarding {
    /// Operacion `email` definida por la abstraccion del ejemplo.
    fn email(&self) -> &str {
        &self.email
    }

    /// Operacion `account type` definida por la abstraccion del ejemplo.
    fn account_type(&self) -> &'static str {
        "starter"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `EnterpriseOnboarding` usado por el ejemplo para expresar el dominio del patron.
pub struct EnterpriseOnboarding {
    email: String,
}

impl EnterpriseOnboarding {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
        }
    }
}

impl OnboardingTemplate for EnterpriseOnboarding {
    /// Operacion `email` definida por la abstraccion del ejemplo.
    fn email(&self) -> &str {
        &self.email
    }

    /// Operacion `account type` definida por la abstraccion del ejemplo.
    fn account_type(&self) -> &'static str {
        "enterprise"
    }

    /// Operacion `hooks` definida por la abstraccion del ejemplo.
    fn hooks(&self) -> Vec<&'static str> {
        vec!["assign_success_manager", "enable_sso"]
    }
}
