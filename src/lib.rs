mod interpreter;
mod lexer;
mod parser;

use interpreter::*;
use lexer::{token::Tokens, Lexer};
use parser::Parser;
use libc::*;

use std::io::{Result, prelude::*};
use std::fs::File;

// use serde::{Deserialize, Serialize};

// use std::path::Path;
// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct test {
//     name: String,
//     body: Vec<serde_json::Value>
// }

    // fn read_code_from_file<P: AsRef<Path>>(path: P) -> Result<test> {
//     let file = File::open(path)?;

//     let first_pass = serde_json::from_reader(file)?;

//     Ok(first_pass)
// }
// let nodes = read_code_from_file("test.json");
// println!("------------> {:?}", nodes);

// #[macro_use]
// extern crate neon;

use neon::{register_module, prelude::*};

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});


// #[no_mangle]
// pub extern "C" fn rust_function() {
//     println!("Hello Lib");
// }

// #[no_mangle]
// pub extern "C" fn get_flow(filename: *const c_char) {
    
// }

// fn read_file(file_path: String) -> Result<String> {
//     let mut file = File::open(file_path)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }

// fn main() {
//     let text = read_file("testv3.CSML".to_owned()).unwrap();
//     let lex_tokens = Lexer::lex_tokens(text.as_bytes());

//     match lex_tokens {
//         Ok((_complete, t)) => {
//             let tokens = Tokens::new(&t);
//             match Parser::parse_tokens(tokens) {
//                 Ok((_, flow)) => {
//                     let mut inter = Interpreter::new(flow);
//                     inter.interpret();
//                 },
//                 Err(e) => {
//                     println!("Error in Paring AST {:?}", e);
//                 }
//             }
//         }
//         Err(e) => println!("Problem in Lexing Tokens -> {:?}", e),
//     };
// }
