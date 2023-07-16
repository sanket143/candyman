mod script;

use crate::md::parser;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;

fn get_response_from_request_data(request_data: HashMap<String, String>) -> Result<String> {
    let response = ureq::request(
        request_data["method"].as_str(),
        request_data["uri"].as_str(),
    )
    .set("content-type", "application/json")
    .send_string(request_data["body"].as_str())?
    .into_string()?;

    Ok(response)
}

pub fn execute(filename: &str) -> Result<String> {
    let content = fs::read_to_string(filename)?;
    let request_data = parser::get_request_data(content.as_str());
    let response = get_response_from_request_data(request_data.clone())?;
    let prelim_code = r#"
function log(item){
  if(typeof item == "object"){
    Deno.core.print(JSON.stringify(item));
  } else {
    Deno.core.print(String(item));
  }
}

function print(first, ...args){
  log(first);

  args.map((arg) => {
    log(" ");
    log(arg);
  });  

  log("\n");
};

function fail(text){
  Deno.core.ops.op_fail(text);
}

function pass(text){
  Deno.core.ops.op_pass(text);
}
    "#;
    let js_code = format!(
        r#"const RESPONSE = {};{};{}"#,
        response, request_data["script"], prelim_code
    );
    script::execute(js_code)?;

    Ok(String::new())
}

#[cfg(test)]
mod test {
    use crate::md::parser::get_request_data;
    #[test]
    fn test_response_from_request_data() {
        let request_data = get_request_data(
            r#"
# URI
https://api.mocki.io/v2/c4d7a195/graphql

# Method
POST

# Body
```json
{"operationName":null,"variables":{},"query":"{\n  users {\n    id\n    name\n  }\n}\n"}
````
"#,
        );

        let response = super::get_response_from_request_data(request_data).unwrap();
        println!("{}", response);
    }
}
