mod interpreter;
mod lexer;
mod parser;

use interpreter::{json_to_rust::*, Interpreter};
use lexer::{token::Tokens, Lexer};
use parser::{Parser, ast::*};
use neon::{register_module, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorMsg {
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
fn parse_file(mut cx: FunctionContext) -> JsResult<JsString>{
    let file = cx.argument::<JsString>(0)?.value();
    println!("-------------------");
    let lex_tokens = dbg!(Lexer::lex_tokens(file.as_bytes()));

    println!("-------------------");
    match lex_tokens {
        Ok((_complete, t)) => {
            let tokens = Tokens::new(&t);
            // println!("{:#?}", tokens);
            match Parser::parse_tokens(tokens) {
                Ok(flow) => {
                    if let Ok(json) = serde_json::to_string(&flow) {
                        return Ok(cx.string(json));
                    }
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
    Ok(cx.string(""))
}

fn flow_istrigger(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let arg0 = cx.argument::<JsString>(0)?.value();
    let arg1 = cx.argument::<JsString>(1)?.value();

    let flow: Flow = serde_json::from_str(&arg0).unwrap();
    Ok(cx.boolean(Interpreter::is_trigger(&flow, &arg1)))
}

fn interpret_flow(mut cx: FunctionContext) -> JsResult<JsString> {
    let arg0 = cx.argument::<JsString>(0)?.value();
    let step_name = cx.argument::<JsString>(1)?.value();
    let context = cx.argument::<JsString>(2)?.value();
    let event = match cx.argument_opt(3) {
        Some(arg)     => {
            let tmp = arg.downcast::<JsString>().or_throw(&mut cx)?.value();
            let event: Event = serde_json::from_str(&tmp).unwrap();
            Some(event)
        },
        None      => None
    };

    let flow: Flow = serde_json::from_str(&arg0).unwrap();
    let memory: JsContext = serde_json::from_str(&context).unwrap();

    println!("memory -> {:?} \n", memory);
    println!("event -> {:?} \n", event);

    let ret = Interpreter::interpret(&flow, &step_name, &memory, &event);
    Ok(cx.string(ret))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)?;
    cx.export_function("parse_file", parse_file)?;
    cx.export_function("interpret_flow", interpret_flow)?;
    cx.export_function("flowIsTrigger", flow_istrigger)?;
    Ok(())
});
