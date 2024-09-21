use anyhow::Result;
use parser::Candyman;

mod parser;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let candyman = Candyman::new(filename)?;
    let resp = candyman.call()?;

    println!("{}", resp);

    Ok(())
}
