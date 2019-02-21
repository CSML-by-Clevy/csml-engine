mod lexer;

use lexer::Lexer;
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
    let split: Vec<&str> = dbg!(text.split('\n').collect());
    let lex_tokens = Lexer::lex_tokens(split);

    dbg!(lex_tokens);
}
