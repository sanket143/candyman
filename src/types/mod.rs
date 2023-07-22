use anyhow::Result;
use ureq::json;

#[derive(Debug, Default)]
pub struct QuesoTest {
    pub name: String,
    pub script: String,
}

impl QuesoTest {
    pub fn new(name: String, script: String) -> Self {
        Self { name, script }
    }
}

#[derive(Debug, Default)]
pub struct Queso {
    pub method: String,
    pub uri: String,
    pub body: String,
    pub body_type: String,
    pub tests: Vec<QuesoTest>,
}

impl Queso {
    pub fn add_method(&mut self, method: String) {
        self.method = method;
    }

    pub fn add_uri(&mut self, uri: String) {
        self.uri = uri;
    }

    pub fn add_body_type(&mut self, body_type: String) {
        let body_type = match body_type.as_str() {
            "json" => "application/json",
            "text" => "text/plain",
            "html" => "text/html",
            "graphql" => "application/json",
            _ => body_type.as_str(),
        }
        .to_owned();

        self.body_type = body_type.clone();
    }

    pub fn add_body(&mut self, body: String) {
        self.body = body;
    }

    pub fn add_graphql_body(&mut self, query: String, variables: Option<String>) {
        let body = json!({
            "query": query,
            "variables": variables,
        });

        self.body = body.to_string();
    }

    pub fn add_test(&mut self, queso_test: (String, String)) {
        let test = QuesoTest::new(queso_test.0, queso_test.1);
        self.tests.push(test);
    }

    pub fn call(&self) -> Result<String> {
        let response = ureq::request(self.method.as_str(), self.uri.as_str())
            .set("content-type", &self.body_type)
            .send_string(&self.body.as_str())?
            .into_string()?;

        Ok(response)
    }
}
