use anyhow::Result;
use colored_json::ToColoredJson;
use deno_core::*;
use deno_core::{op, Extension, JsRuntime, RuntimeOptions};

#[op]
fn op_fail(text: &str) {
    println!("\x1b[31m{}\x1b[0m", text);
}

#[op]
fn op_pass(text: &str) {
    println!("\x1b[32m{}\x1b[0m", text);
}

#[op]
fn op_json_print(text: &str) {
    println!("{}", text.to_colored_json_auto().unwrap());
}

pub fn execute(js_code: String) -> Result<String> {
    let ext = Extension::builder("my_ext")
        .ops(vec![op_fail::DECL, op_pass::DECL, op_json_print::DECL])
        .build();
    let mut js_runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![ext],
        ..RuntimeOptions::default()
    });
    js_runtime.execute_script("<anon>", js_code.into())?;

    Ok(String::new())
}
