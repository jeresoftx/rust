pub struct Request {
    token: Option<String>,
    role: String,
    payload: String,
}

impl Request {
    pub fn new(token: Option<&str>, role: &str, payload: &str) -> Self {
        Self {
            token: token.map(str::to_string),
            role: role.to_string(),
            payload: payload.to_string(),
        }
    }
}

pub struct RequestValidationChain {
    validators: Vec<fn(&Request) -> Result<(), String>>,
}

impl RequestValidationChain {
    pub fn validate(&self, request: &Request) -> Result<String, String> {
        for validator in &self.validators {
            validator(request)?;
        }

        Ok("request accepted".to_string())
    }
}

impl Default for RequestValidationChain {
    fn default() -> Self {
        Self {
            validators: vec![validate_authentication, validate_role, validate_payload],
        }
    }
}

fn validate_authentication(request: &Request) -> Result<(), String> {
    if request.token.is_some() {
        Ok(())
    } else {
        Err("missing authentication token".to_string())
    }
}

fn validate_role(request: &Request) -> Result<(), String> {
    if request.role == "admin" {
        Ok(())
    } else {
        Err(format!("role {} cannot perform this action", request.role))
    }
}

fn validate_payload(request: &Request) -> Result<(), String> {
    if request.payload.trim().is_empty() {
        Err("request payload is empty".to_string())
    } else {
        Ok(())
    }
}
