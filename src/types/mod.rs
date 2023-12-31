use std::collections::HashMap;

use anyhow::Result;
use ureq::json;

#[derive(Debug, Default)]
pub struct Test {
    pub name: String,
    pub script: String,
}

impl Test {
    pub fn new(name: String, script: String) -> Self {
        Self { name, script }
    }
}

#[derive(Debug, Default)]
pub struct Candyman {
    pub method: String,
    pub uri: String,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub content_type: String,
    pub tests: Vec<Test>,
}

impl Candyman {
    pub fn add_method(&mut self, method: String) {
        self.method = method;
    }

    pub fn add_uri(&mut self, uri: String) {
        self.uri = uri;
    }

    pub fn add_content_type(&mut self, content_type: String) {
        let content_type = match content_type.as_str() {
            "json" => "application/json",
            "text" => "text/plain",
            "html" => "text/html",
            "graphql" => "application/json",
            _ => content_type.as_str(),
        }
        .to_owned();

        self.content_type = content_type.clone();
    }

    pub fn add_headers(&mut self, headers: HashMap<String, String>) {
        self.headers = headers;
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

    pub fn add_test(&mut self, test: (String, String)) {
        let test = Test::new(test.0, test.1);
        self.tests.push(test);
    }

    pub fn call(&self) -> Result<String> {
        let mut req = ureq::request(self.method.as_str(), self.uri.as_str())
            .set("content-type", &self.content_type);

        for (key, value) in &self.headers {
            req = req.set(key, value);
        }

        let response = req.send_string(&self.body.as_str())?.into_string()?;

        Ok(response)
    }
}
