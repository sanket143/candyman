use anyhow::Result;
use std::process::Command;

pub fn execute(js_code: String) -> Result<String> {
    Command::new("node")
        .arg("-p")
        .arg(js_code)
        .spawn()
        .expect("node command failed to start");

    Ok(String::new())
}
