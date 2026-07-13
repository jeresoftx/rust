#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `Environment` dentro del ejemplo.
pub enum Environment {
    /// Variante `Local` del estado o error del ejemplo.
    Local,
    /// Variante `Staging` del estado o error del ejemplo.
    Staging,
    /// Variante `Production` del estado o error del ejemplo.
    Production,
}

trait HttpClient {
    /// Operacion `get` definida por la abstraccion del ejemplo.
    fn get(&self, path: &str) -> String;
}

struct LocalHttpClient;

impl HttpClient for LocalHttpClient {
    /// Operacion `get` definida por la abstraccion del ejemplo.
    fn get(&self, path: &str) -> String {
        format!("GET http://localhost:3000{path} timeout=1000ms retries=0")
    }
}

struct StagingHttpClient;

impl HttpClient for StagingHttpClient {
    /// Operacion `get` definida por la abstraccion del ejemplo.
    fn get(&self, path: &str) -> String {
        format!("GET https://staging.api.example.com{path} timeout=3000ms retries=1")
    }
}

struct ProductionHttpClient;

impl HttpClient for ProductionHttpClient {
    /// Operacion `get` definida por la abstraccion del ejemplo.
    fn get(&self, path: &str) -> String {
        format!("GET https://api.example.com{path} timeout=5000ms retries=3")
    }
}

/// Operacion `client for` definida por la abstraccion del ejemplo.
fn client_for(environment: Environment) -> Box<dyn HttpClient> {
    match environment {
        Environment::Local => Box::new(LocalHttpClient),
        Environment::Staging => Box::new(StagingHttpClient),
        Environment::Production => Box::new(ProductionHttpClient),
    }
}

/// Modela la operacion `request summary` dentro del ejemplo del patron.
pub fn request_summary(environment: Environment, path: &str) -> String {
    let client = client_for(environment);

    client.get(path)
}
