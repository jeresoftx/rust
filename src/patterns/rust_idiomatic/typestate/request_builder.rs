use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissingUrl;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadyToSend;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestBuilder<State> {
    method: String,
    headers: Vec<(String, String)>,
    url: Option<String>,
    body: Option<String>,
    state: PhantomData<State>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FakeResponse {
    status: u16,
    method: String,
    url: String,
    header_count: usize,
    has_body: bool,
}

impl RequestBuilder<MissingUrl> {
    pub fn new() -> Self {
        Self {
            method: "GET".to_string(),
            headers: Vec::new(),
            url: None,
            body: None,
            state: PhantomData,
        }
    }

    pub fn url(self, url: impl Into<String>) -> RequestBuilder<ReadyToSend> {
        RequestBuilder {
            method: self.method,
            headers: self.headers,
            url: Some(url.into()),
            body: self.body,
            state: PhantomData,
        }
    }
}

impl Default for RequestBuilder<MissingUrl> {
    fn default() -> Self {
        Self::new()
    }
}

impl<State> RequestBuilder<State> {
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = method.into();
        self
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
}

impl RequestBuilder<ReadyToSend> {
    pub fn send(self) -> FakeResponse {
        FakeResponse {
            status: 202,
            method: self.method,
            url: self.url.expect("ReadyToSend siempre contiene URL"),
            header_count: self.headers.len(),
            has_body: self.body.is_some(),
        }
    }
}

impl FakeResponse {
    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn summary(&self) -> String {
        let body = if self.has_body { "body" } else { "no body" };

        format!(
            "{} {} with {} headers and {}",
            self.method, self.url, self.header_count, body
        )
    }
}
