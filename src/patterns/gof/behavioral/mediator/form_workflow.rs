#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `RegistrationForm` usado por el ejemplo para expresar el dominio del patron.
pub struct RegistrationForm {
    name: String,
    email: String,
}

impl RegistrationForm {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `SubmissionReceipt` usado por el ejemplo para expresar el dominio del patron.
pub struct SubmissionReceipt {
    /// Campo publico `record_id` expuesto por el tipo del ejemplo.
    pub record_id: u32,
    /// Campo publico `message` expuesto por el tipo del ejemplo.
    pub message: String,
}

#[derive(Debug, Default, Clone)]
/// Tipo publico `FormMediator` usado por el ejemplo para expresar el dominio del patron.
pub struct FormMediator {
    next_record_id: u32,
    saved_records: u32,
    events: Vec<String>,
}

impl FormMediator {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self {
            next_record_id: 1,
            saved_records: 0,
            events: Vec::new(),
        }
    }

    /// Modela la operacion `submit` dentro del ejemplo del patron.
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

    /// Modela la operacion `events` dentro del ejemplo del patron.
    pub fn events(&self) -> Vec<String> {
        self.events.clone()
    }

    /// Modela la operacion `saved records` dentro del ejemplo del patron.
    pub fn saved_records(&self) -> u32 {
        self.saved_records
    }

    /// Operacion `is valid` definida por la abstraccion del ejemplo.
    fn is_valid(&self, form: &RegistrationForm) -> bool {
        !form.name.trim().is_empty() && form.email.contains('@')
    }

    /// Operacion `save` definida por la abstraccion del ejemplo.
    fn save(&mut self) -> u32 {
        let record_id = self.next_record_id;
        self.next_record_id += 1;
        self.saved_records += 1;
        self.events.push(format!("repository:saved:{record_id}"));
        record_id
    }
}
