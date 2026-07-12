#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnboardingPlan {
    steps: Vec<String>,
}

impl OnboardingPlan {
    pub fn new(steps: Vec<String>) -> Self {
        Self { steps }
    }

    pub fn steps(&self) -> Vec<&str> {
        self.steps.iter().map(String::as_str).collect()
    }
}

pub trait OnboardingTemplate {
    fn email(&self) -> &str;
    fn account_type(&self) -> &'static str;

    fn is_valid(&self) -> bool {
        self.email().contains('@')
    }

    fn base_provisioning(&self) -> Vec<&'static str> {
        vec!["workspace", "billing_profile"]
    }

    fn hooks(&self) -> Vec<&'static str> {
        vec!["send_welcome_email"]
    }

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
pub struct StarterOnboarding {
    email: String,
}

impl StarterOnboarding {
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
        }
    }
}

impl OnboardingTemplate for StarterOnboarding {
    fn email(&self) -> &str {
        &self.email
    }

    fn account_type(&self) -> &'static str {
        "starter"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnterpriseOnboarding {
    email: String,
}

impl EnterpriseOnboarding {
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
        }
    }
}

impl OnboardingTemplate for EnterpriseOnboarding {
    fn email(&self) -> &str {
        &self.email
    }

    fn account_type(&self) -> &'static str {
        "enterprise"
    }

    fn hooks(&self) -> Vec<&'static str> {
        vec!["assign_success_manager", "enable_sso"]
    }
}
