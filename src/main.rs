mod interact;
mod lexer;
mod parser;
mod builtins;

use interact::*;
use lexer::{token::Tokens, Lexer};
use parser::Parser;
use std::fs::File;
use std::io::prelude::*;

fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let text = read_file("testv3.CSML".to_owned()).unwrap();
    let lex_tokens = Lexer::lex_tokens(text.as_bytes());

    // interact::test_json();

    match lex_tokens {
        Ok((_complete, t)) => {
            let tokens = Tokens::new(&t);
            let (_, flow) = Parser::parse_tokens(tokens).unwrap();
            let mut inter = Interpreter::new(flow);
            inter.interpret();
        }
        Err(e) => println!("{:?}", e),
    };
}
