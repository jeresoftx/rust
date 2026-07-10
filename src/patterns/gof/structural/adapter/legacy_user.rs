pub struct LegacyUserRecord {
    id: String,
    full_name: String,
    email_address: String,
    status_code: String,
}

impl LegacyUserRecord {
    pub fn new(
        id: impl Into<String>,
        full_name: impl Into<String>,
        email_address: impl Into<String>,
        status_code: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            full_name: full_name.into(),
            email_address: email_address.into(),
            status_code: status_code.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UserProfile {
    id: String,
    display_name: String,
    email: String,
    active: bool,
}

impl UserProfile {
    pub fn summary(&self) -> String {
        format!(
            "id={} name={} email={} active={}",
            self.id, self.display_name, self.email, self.active
        )
    }
}

pub fn adapt_legacy_user(legacy: LegacyUserRecord) -> Result<UserProfile, String> {
    if !legacy.email_address.contains('@') {
        return Err("legacy user email must contain @".to_string());
    }

    Ok(UserProfile {
        id: legacy.id,
        display_name: legacy.full_name,
        email: legacy.email_address,
        active: legacy.status_code == "active",
    })
}
