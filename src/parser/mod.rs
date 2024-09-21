use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use anyhow::Result;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use yaml_rust2::{Yaml, YamlLoader};

#[derive(Default, Serialize, Deserialize, Debug)]
struct GraphQLRequest {
    query: String,
    variables: Value,
}

#[derive(Default, Debug)]
enum BodyType {
    #[default]
    REGULAR,
    GRAPHQL,
}

#[derive(Parser, Debug, Default)]
#[grammar = "parser/grammar.pest"]
pub struct Candyman {
    method: String,
    uri: String,
    body: String,
    body_type: BodyType,
    headers: HashMap<String, String>,
    graphql_body: GraphQLRequest,
}

impl Candyman {
    fn visit_header(self: &mut Self, pair: &Pair<Rule>) -> Result<()> {
        let yaml_loader = YamlLoader::load_from_str(pair.as_str())?;

        for header in yaml_loader.iter() {
            if let Yaml::Hash(header) = header {
                for header in header.iter() {
                    self.headers.insert(
                        header.0.as_str().unwrap().to_owned().to_lowercase(),
                        header.1.as_str().unwrap().to_owned(),
                    );
                }
            }
        }

        Ok(())
    }

    fn visit_graphql_query(self: &mut Self, pair: &Pair<Rule>) -> Result<()> {
        self.body_type = BodyType::GRAPHQL;
        self.graphql_body.query = pair.as_str().to_owned();

        Ok(())
    }

    fn visit_graphql_variables(self: &mut Self, pair: &Pair<Rule>) -> Result<()> {
        self.body_type = BodyType::GRAPHQL;
        self.graphql_body.variables = pair.as_str().into();

        Ok(())
    }

    fn visit_block(self: &mut Self, pair: Pair<Rule>) -> Result<()> {
        let mut event = pair.as_rule();

        for pair in pair.into_inner() {
            if pair.as_rule() == Rule::BLOCK_CONTENT {
                match event {
                    Rule::URI => {
                        self.uri = pair.as_str().trim().to_owned();
                    }
                    Rule::METHOD => {
                        self.method = pair.as_str().trim().to_owned();
                    }
                    Rule::HEADERS => {
                        self.visit_header(&pair)?;
                    }
                    Rule::BODY_GRAPHQL_QUERY => {
                        self.visit_graphql_query(&pair)?;
                    }
                    Rule::BODY_GRAPHQL_VARIABLES => {
                        self.visit_graphql_variables(&pair)?;
                    }
                    _ => {}
                }
            }

            event = pair.as_rule();
        }

        // in case the request is GraphQL request, this will update the body
        self.handle_graphql_body()?;

        Ok(())
    }

    fn handle_graphql_body(self: &mut Self) -> Result<()> {
        match self.body_type {
            BodyType::GRAPHQL => {
                self.headers.insert(
                    String::from("content-type"),
                    String::from("application/json"),
                );
                self.body = serde_json::to_string(&self.graphql_body)?;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn call(&self) -> Result<String> {
        let mut req = ureq::request(self.method.as_str(), self.uri.as_str());

        for (key, value) in &self.headers {
            req = req.set(key, value);
        }

        println!("{:#?}", self);
        let response = req.send_string(&self.body.as_str())?.into_string()?;

        Ok(response)
    }

    pub fn new(filename: &String) -> Result<Self> {
        let content = std::fs::read_to_string(filename)?;
        let mut candyman = Candyman {
            method: String::from("GET"),
            ..Candyman::default()
        };

        let pairs: Pair<Rule> = Candyman::parse(Rule::PROGRAM, &content)?.next().unwrap();

        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::BLOCK => {
                    candyman.visit_block(pair)?;
                }
                _ => {}
            }
        }

        Ok(candyman)
    }
}
