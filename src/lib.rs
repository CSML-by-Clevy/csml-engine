pub mod error_format;
pub mod interpreter;
pub mod parser;

use error_format::data::ErrorInfo;
use interpreter::{ast_interpreter::interpret_block, csml_rules::*, data::Data, json_to_rust::*};
use parser::{ast::*, Parser};
use std::collections::HashMap;

pub fn parse_file(file: String) -> Result<Flow, ErrorInfo> {
    // add flow validations
    match Parser::parse_flow(file.as_bytes()) {
        Ok(flow) => Ok(flow),
        Err(e) => Err(e),
    }
}

pub fn is_trigger(flow: &Flow, string: &str) -> bool {
    let info = flow.flow_instructions.get(&InstructionType::StartFlow);

    if let Some(Expr::VecExpr(vec, ..)) = info {
        for elem in vec.iter() {
            match elem {
                Expr::LitExpr(SmartLiteral {
                    literal: Literal::StringLiteral{value, ..},
                    ..
                }) if value.to_lowercase() == string.to_lowercase() => return true,
                _ => continue,
            }
        }
    }
    false
}

pub fn search_for<'a>(flow: &'a Flow, name: &str) -> Option<&'a Expr> {
    flow.flow_instructions
        .get(&InstructionType::NormalStep(name.to_owned()))
}

pub fn execute_step(flow: &Flow, name: &str, mut data: Data) -> Result<String, ErrorInfo> {
    match search_for(flow, name) {
        Some(Expr::Block { arg: actions, .. }) => {
            let result = interpret_block(actions, &mut data)?;

            dbg!(&result);

            match serde_json::to_string(&result) {
                Ok(ser) => Ok(ser),
                Err(_) => unreachable!(),
            }
        }
        _ => Err(ErrorInfo {
            interval: Interval { line: 0, column: 0 },
            message: "ERROR: Empty Flow".to_string(),
        }),
    }
}

pub fn interpret(
    ast: &Flow,
    step_name: &str,
    memory: &Memory,
    event: &Option<Event>,
) -> Result<String, ErrorInfo> {
    if !check_valid_flow(ast) {
        return Err(ErrorInfo {
            interval: Interval { line: 0, column: 0 },
            message: "ERROR: invalid Flow".to_string(),
        });
    }

    // dbg!(&ast);
    // let memory = context_to_memory(context);

    let data = Data {
        ast,
        memory: memory,
        event,
        step_vars: HashMap::new(),
    };
    Ok(execute_step(ast, step_name, data)?)
}
