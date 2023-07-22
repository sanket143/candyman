mod client;
mod lexer;
mod md;
mod types;

use anyhow::Result;

enum Action {
    Continue,
    Quit,
}

fn evaluate(text: String) -> Result<Action> {
    if text.len() == 0 {
        return Ok(Action::Continue);
    }

    let mut lex = lexer::new_lexer(&text);
    let mut token = lex.next();

    loop {
        if token.ttype == lexer::types::TokenType::EOF {
            break;
        }
        token = lex.next();
    }

    let tokens: Vec<&str> = text.split_whitespace().collect();
    let command = tokens[0];

    match command {
        "e" => {
            let response = client::execute(tokens[1])?;
            println!("{}", response);
            Ok(Action::Continue)
        }
        "test" => {
            let response = client::execute(tokens[1])?;
            println!("{}", response);
            Ok(Action::Continue)
        }
        "q" => Ok(Action::Quit),
        _ => Ok(Action::Continue),
    }
}

fn main() {
    let mut rl = rustyline::DefaultEditor::new().unwrap();

    loop {
        let readline = rl.readline(">> ").unwrap_or(String::from(""));
        rl.add_history_entry(&readline).unwrap();

        match evaluate(readline) {
            Ok(Action::Quit) => break,
            Err(e) => println!("{}", e),
            _ => {}
        }
    }
}
