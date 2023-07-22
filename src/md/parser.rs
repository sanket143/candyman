use crate::types::Queso;
use markdown::{tokenize, Block, Span};

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

pub fn get_request_data(markdown: &str) -> Queso {
    let tokens = tokenize(markdown);
    let mut section: Option<String> = None;
    let mut queso = Queso::default();
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
                            queso.add_method(value);
                        }
                        "uri" => {
                            queso.add_uri(value);
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

                            queso.add_body_type(typename.clone());

                            if typename == "graphql" {
                                let variables = iter.peek();
                                if let Some(Block::CodeBlock(_, variables)) = variables {
                                    queso.add_graphql_body(
                                        value.clone(),
                                        Some(variables.to_owned()),
                                    );
                                } else {
                                    queso.add_graphql_body(value.clone(), None);
                                }
                            } else {
                                queso.add_body(value.clone());
                            }
                        }
                        _ => {}
                    }
                    if section_name.to_lowercase().starts_with("[test]") {
                        queso.add_test((section_name.clone(), value.clone()));
                    }
                    section = None;
                }
            }
            _ => {}
        };
    }

    queso
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
