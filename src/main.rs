mod lexer;
mod parser;

use lexer::{Lexer, token::Tokens};
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
    let text = read_file("testv1.CSML".to_owned()).unwrap();
    // let split: Vec<&str> = dbg!(text.split('\n').collect());
    let lex_tokens = Lexer::lex_tokens(text.as_bytes());

    match lex_tokens {
        Ok((_complete, t) ) => {
            let tokens = Tokens::new(&t);
            let ret = Parser::parse_tokens(tokens);
            println!("label  {:?}", ret) 
        },
        Err(e) => { println!("{:?}", e) }
    };
}
