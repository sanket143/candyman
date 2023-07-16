use std::collections::HashMap;

use markdown::{tokenize, Block, Span};

fn get_text_from_span(span: &Span) -> &str {
    match span {
        Span::Text(text) => text.as_str(),
        Span::Code(text) => text,
        Span::Link(text, ..) => text,
        _ => "",
    }
}

fn get_text_from_spans(spans: Vec<Span>) -> String {
    spans.iter().fold(String::from(""), |mut acc, span| {
        acc.push_str(get_text_from_span(span));
        acc
    })
}

pub fn get_request_data(markdown: &str) -> HashMap<String, String> {
    let tokens = tokenize(markdown);
    let mut section = String::new();
    let mut request_values = HashMap::<String, String>::new();

    for token in tokens {
        match token {
            Block::Header(spans, ..) => {
                section = get_text_from_spans(spans).to_lowercase();
            }
            Block::Paragraph(spans) => {
                let value = get_text_from_spans(spans);
                request_values.insert(section.clone(), value);
            }
            Block::CodeBlock(_type, value) => {
                request_values.insert(section.clone(), value);
            }
            _ => {}
        };
    }

    request_values
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
            request_data["uri"],
            "https://graphql.api.apollographql.com/api/graphql",
        );
        assert_eq!(request_data["method"], "POST");
        assert_eq!(request_data["body"], "Body");
    }
}
