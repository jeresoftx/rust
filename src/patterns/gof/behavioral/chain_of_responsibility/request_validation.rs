/// Tipo asociado `RequestValidator` producido por la abstraccion del ejemplo.
type RequestValidator = fn(&Request) -> Result<(), String>;

/// Tipo publico `Request` usado por el ejemplo para expresar el dominio del patron.
pub struct Request {
    token: Option<String>,
    role: String,
    payload: String,
}

impl Request {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(token: Option<&str>, role: &str, payload: &str) -> Self {
        Self {
            token: token.map(str::to_string),
            role: role.to_string(),
            payload: payload.to_string(),
        }
    }
}

/// Tipo publico `RequestValidationChain` usado por el ejemplo para expresar el dominio del patron.
pub struct RequestValidationChain {
    validators: Vec<RequestValidator>,
}

impl RequestValidationChain {
    /// Valida la entrada respetando las reglas del patron.
    pub fn validate(&self, request: &Request) -> Result<String, String> {
        for validator in &self.validators {
            validator(request)?;
        }

        Ok("request accepted".to_string())
    }
}

impl Default for RequestValidationChain {
    /// Operacion `default` definida por la abstraccion del ejemplo.
    fn default() -> Self {
        Self {
            validators: vec![validate_authentication, validate_role, validate_payload],
        }
    }
}

/// Operacion `validate authentication` definida por la abstraccion del ejemplo.
fn validate_authentication(request: &Request) -> Result<(), String> {
    if request.token.is_some() {
        Ok(())
    } else {
        Err("missing authentication token".to_string())
    }
}

/// Operacion `validate role` definida por la abstraccion del ejemplo.
fn validate_role(request: &Request) -> Result<(), String> {
    if request.role == "admin" {
        Ok(())
    } else {
        Err(format!("role {} cannot perform this action", request.role))
    }
}

/// Operacion `validate payload` definida por la abstraccion del ejemplo.
fn validate_payload(request: &Request) -> Result<(), String> {
    if request.payload.trim().is_empty() {
        Err("request payload is empty".to_string())
    } else {
        Ok(())
    }
}
