#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Local,
    Staging,
    Production,
}

trait HttpClient {
    fn get(&self, path: &str) -> String;
}

struct LocalHttpClient;

impl HttpClient for LocalHttpClient {
    fn get(&self, path: &str) -> String {
        format!("GET http://localhost:3000{path} timeout=1000ms retries=0")
    }
}

struct StagingHttpClient;

impl HttpClient for StagingHttpClient {
    fn get(&self, path: &str) -> String {
        format!("GET https://staging.api.example.com{path} timeout=3000ms retries=1")
    }
}

struct ProductionHttpClient;

impl HttpClient for ProductionHttpClient {
    fn get(&self, path: &str) -> String {
        format!("GET https://api.example.com{path} timeout=5000ms retries=3")
    }
}

fn client_for(environment: Environment) -> Box<dyn HttpClient> {
    match environment {
        Environment::Local => Box::new(LocalHttpClient),
        Environment::Staging => Box::new(StagingHttpClient),
        Environment::Production => Box::new(ProductionHttpClient),
    }
}

pub fn request_summary(environment: Environment, path: &str) -> String {
    let client = client_for(environment);

    client.get(path)
}
