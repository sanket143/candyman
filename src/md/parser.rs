use crate::types::Candyman;
use markdown::{tokenize, Block, Span};
use toml::Table;

fn get_text_from_span(span: &Span) -> &str {
    match span {
        Span::Text(text) => text.as_str(),
        Span::Code(text) => text,
        Span::Link(text, ..) => text,
        _ => "",
    }
}

fn get_text_from_spans(spans: &Vec<Span>) -> String {
    spans.iter().fold(String::from(""), |mut acc, span| {
        acc.push_str(get_text_from_span(span));
        acc
    })
}

pub fn get_request_data(markdown: &str) -> Candyman {
    let tokens = tokenize(markdown);
    let mut section: Option<String> = None;
    let mut candyman = Candyman::default();
    let mut iter = tokens.iter().peekable();

    while let Some(token) = iter.next() {
        match token {
            Block::Header(spans, ..) => {
                section = Some(get_text_from_spans(&spans));
            }
            Block::Paragraph(spans) => {
                if let Some(section_name) = section.clone() {
                    let value = get_text_from_spans(&spans);
                    match section_name.to_lowercase().as_str() {
                        "method" => {
                            candyman.add_method(value);
                        }
                        "uri" => {
                            candyman.add_uri(value);
                        }
                        _ => {}
                    }
                    section = None;
                }
            }
            Block::CodeBlock(typename, value) => {
                if let Some(section_name) = section.clone() {
                    match section_name.to_lowercase().as_str() {
                        "body" => {
                            let typename = typename.clone().unwrap_or(String::from("text/plain"));

                            candyman.add_content_type(typename.clone());

                            if typename == "graphql" {
                                let variables = iter.peek();

                                while let Some(block) = iter.peek() {
                                    match block {
                                        Block::Header(..) => {
                                            candyman.add_graphql_body(value.clone(), None);
                                            break;
                                        }
                                        Block::CodeBlock(_, variables) => {
                                            candyman.add_graphql_body(
                                                value.clone(),
                                                Some(variables.to_owned()),
                                            );
                                            break;
                                        }
                                        _ => {
                                            iter.next();
                                        }
                                    }
                                }
                            } else {
                                candyman.add_body(value.clone());
                            }
                        }
                        "request" => {
                            if let Some(typename) = typename {
                                if typename == "toml" {
                                    let value = value.parse::<Table>().unwrap();
                                    if let Some(uri) = value.get("uri") {
                                        candyman.add_uri(uri.as_str().unwrap().to_owned());
                                    }

                                    if let Some(method) = value.get("method") {
                                        candyman.add_method(method.as_str().unwrap().to_owned());
                                    }
                                }
                            }
                        }
                        "headers" => {
                            if let Some(typename) = typename {
                                if typename == "json" {
                                    candyman
                                        .add_headers(ureq::serde_json::from_str(value).unwrap());
                                } else {
                                    println!("Headers code block typename should be json");
                                }
                            }
                        }
                        _ => {}
                    }
                    if section_name.to_lowercase().starts_with("[test]") {
                        candyman.add_test((section_name.clone(), value.clone()));
                    }
                    section = None;
                }
            }
            _ => {}
        };
    }

    candyman
}

#[cfg(test)]
mod test {
    #[test]
    fn test_get_request() {
        let request_data = super::get_request_data(
            r#"
# URI
https://graphql.api.apollographql.com/api/graphql

# Method
POST

# Body
```
Body
````

# Script
```
print("Hello"); // should be executed after the response is received
```
"#,
        );
        println!("{:#?}", request_data);

        assert_eq!(
            request_data.uri,
            "https://graphql.api.apollographql.com/api/graphql",
        );
        assert_eq!(request_data.method, "POST");
        assert_eq!(request_data.body, "Body");
    }
}
