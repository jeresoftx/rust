#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistrationForm {
    name: String,
    email: String,
}

impl RegistrationForm {
    pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmissionReceipt {
    pub record_id: u32,
    pub message: String,
}

#[derive(Debug, Default, Clone)]
pub struct FormMediator {
    next_record_id: u32,
    saved_records: u32,
    events: Vec<String>,
}

impl FormMediator {
    pub fn new() -> Self {
        Self {
            next_record_id: 1,
            saved_records: 0,
            events: Vec::new(),
        }
    }

    pub fn submit(&mut self, form: RegistrationForm) -> Result<SubmissionReceipt, String> {
        if !self.is_valid(&form) {
            self.events.push("validation:failed".to_string());
            return Err("name and valid email are required".to_string());
        }

        self.events.push("validation:passed".to_string());
        let record_id = self.save();
        let message = "registration saved".to_string();
        self.events.push(format!("notification:{message}"));

        Ok(SubmissionReceipt { record_id, message })
    }

    pub fn events(&self) -> Vec<String> {
        self.events.clone()
    }

    pub fn saved_records(&self) -> u32 {
        self.saved_records
    }

    fn is_valid(&self, form: &RegistrationForm) -> bool {
        !form.name.trim().is_empty() && form.email.contains('@')
    }

    fn save(&mut self) -> u32 {
        let record_id = self.next_record_id;
        self.next_record_id += 1;
        self.saved_records += 1;
        self.events.push(format!("repository:saved:{record_id}"));
        record_id
    }
}
