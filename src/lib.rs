mod interpreter;
mod lexer;
mod parser;

use interpreter::{json_to_rust::{Event, JsContext}, Interpreter, csml_rules::*};
use lexer::{token::Tokens, Lexer};
use parser::{Parser, ast::*};
use neon::{register_module, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorMsg {
    pub error: String,
}

fn parse_file(mut cx: FunctionContext) -> JsResult<JsString> {
    let file = cx.argument::<JsString>(0)?.value();
    let lex_tokens = Lexer::lex_tokens(file.as_bytes());

    match lex_tokens {
        Ok((_complete, t)) => {
            let tokens = Tokens::new(&t);

            match Parser::parse_tokens(tokens) {
                Ok(flow) => {
                    if let Ok(json) = serde_json::to_string(&dbg!(flow) ) {
                        return Ok(cx.string(json));
                    }
                },
                Err(e) => {
                    return cx.throw_error(format!("Error in Paring AST {:?}", e))
                }
            }
        }
        Err(e) => return cx.throw_error(format!("Problem in Lexing Tokens -> {:?}", e)),
    };
    Ok(cx.string(""))
}

fn flow_istrigger(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let arg0 = cx.argument::<JsString>(0)?.value();
    let arg1 = cx.argument::<JsString>(1)?.value();

    let flow: Flow  = match serde_json::from_str(&arg0) {
        Ok(flow)    => flow,
        Err(e)      => return cx.throw_error(format!("Error in parsing Flow : {:?}", e))
    };

    Ok(cx.boolean(is_trigger(&flow, &arg1)))
}

fn interpret_flow(mut cx: FunctionContext) -> JsResult<JsString> {
    let arg0 = cx.argument::<JsString>(0)?.value();
    let step_name = cx.argument::<JsString>(1)?.value();
    let context = cx.argument::<JsString>(2)?.value();

    let event = match cx.argument_opt(3) {
        Some(arg)     => {
            let tmp = arg.downcast::<JsString>().or_throw(&mut cx)?.value();
            let event: Event = match serde_json::from_str(&tmp) {
                Ok(event)   => event,
                Err(e)      => return cx.throw_error( format!("Error in parsing Event : {:?}", e))
            };
            Some(event)
        },
        None      => None
    };

    let flow: Flow = match serde_json::from_str(&arg0) {
        Ok(flow)    => flow,
        Err(e)      => return cx.throw_error(format!("Error in parsing Flow : {:?}", e))
    };  
    let memory: JsContext = match serde_json::from_str(&context) {
        Ok(mem)    => mem,
        Err(e)     => return cx.throw_error(format!("Error in parsing Memory : {:?}", e))
    };
    let message = match Interpreter::interpret(&flow, &step_name, &memory, &event) {
        Ok(msg)    => msg,
        Err(e)     => return cx.throw_error(format!("Error: {:?}", e))
    };

    Ok(cx.string(message))
}

register_module!(mut cx, {
    cx.export_function("parse_file", parse_file)?;
    cx.export_function("interpret_flow", interpret_flow)?;
    cx.export_function("flowIsTrigger", flow_istrigger)?;
    Ok(())
});
