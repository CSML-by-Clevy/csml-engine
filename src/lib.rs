mod interpreter;
mod lexer;
mod parser;

use interpreter::*;
use lexer::{token::Tokens, Lexer};
use parser::Parser;
use neon::{register_module, prelude::*};
use neon_serde;

use serde::{Deserialize, Serialize};

use std::io::{Result, prelude::*};
use std::fs::File;

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct My_test {
//     pub interaction_id: Option<String>,
//     pub messages: Vec<serde_json::Value>,
//     pub next_step: Option<String>,
// }

// pub struct My_event {
//     pub channel_id: String,
//     pub channel_type: String,
//     pub user_id: String,
//     pub timestamp: String,
// }
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Error_msg {
    pub error: String,
}

// fn read_file(file_path: String) -> Result<String> {
//     let mut file = File::open(file_path)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }

fn hello(mut cx: FunctionContext) -> JsResult<JsNull> {
    println!("Hello from lib");
    Ok(cx.null())
}

// let arg1 = cx.argument::<JsString>(0)?.value();
// let tmp: My_test = serde_json::from_str(&arg1).unwrap();
fn parse_file(mut cx: FunctionContext) -> JsResult<JsValue>{
    let file = cx.argument::<JsString>(0)?.value();
    let lex_tokens = Lexer::lex_tokens(file.as_bytes());

    match lex_tokens {
        Ok((_complete, t)) => {
            let tokens = Tokens::new(&t);
            match Parser::parse_tokens(tokens) {
                Ok((_, flow)) => {
                    let js_value = neon_serde::to_value(&mut cx, &flow)?;
                    return Ok(js_value)
                    // let mut inter = Interpreter::new(flow);
                    // inter.interpret();
                },
                Err(e) => {
                    println!("Error in Paring AST {:?}", e);
                }
            }
        }
        Err(e) => println!("Problem in Lexing Tokens -> {:?}", e),
    };
    let e = Error_msg{error: "error".to_owned() };
    let js_value = neon_serde::to_value(&mut cx, &e)?;
    return Ok(js_value)
}



register_module!(mut cx, {
    cx.export_function("hello", hello)?;
    cx.export_function("parse_file", parse_file)?;
    Ok(())
});
