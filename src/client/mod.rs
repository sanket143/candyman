mod script;

use crate::md::parser;
use anyhow::Result;
use std::fs;

pub fn execute(filename: &str) -> Result<()> {
    let content = fs::read_to_string(filename)?;
    let request_data = parser::get_request_data(content.as_str());
    let response = request_data.call()?;
    let prelim_code = r#"
function log(item){
  if(typeof item == "object"){
    Deno.core.ops.op_json_print(JSON.stringify(item));
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

    for test in request_data.tests {
        let js_code = format!(
            r#"const RESPONSE = {};{};{}"#,
            response, test.script, prelim_code
        );

        println!("\x1b[33m{}\x1b[0m", test.name);
        script::execute(js_code)?;
    }

    Ok(())
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

        let response = request_data.call().unwrap();
        println!("{}", response);
    }
}
