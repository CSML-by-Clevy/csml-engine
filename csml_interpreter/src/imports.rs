use crate::data::error_info::ErrorInfo;
use crate::data::ast::{Expr, Flow, ImportScope, InstructionScope};

use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn get_function<'a>(
    flow: &'a Flow,
    fn_name: &str,
    original_name: &Option<String>,
) -> Option<(Vec<String>, Expr, &'a Flow)> {
    let name = match original_name {
        Some(original_name) => original_name.to_owned(),
        None => fn_name.to_owned(),
    };

    if let (InstructionScope::FunctionScope { name: _, args }, expr) = flow
        .flow_instructions
        .get_key_value(&InstructionScope::FunctionScope {
            name,
            args: Vec::new(),
        })?
    {
        return Some((args.to_owned(), expr.to_owned(), flow));
    }
    None
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn validate_imports(
    bot: &HashMap<String, Flow>,
    imports: Vec<ImportScope>,
    errors: &mut Vec<ErrorInfo>,
) {
    for import in imports.iter() {
        match search_function(bot, import) {
            Ok(_) => {}
            Err(err) => errors.push(err),
        }
    }
}

pub fn search_function<'a>(
    bot: &'a HashMap<String, Flow>,
    import: &ImportScope,
) -> Result<(Vec<String>, Expr, &'a Flow), ErrorInfo> {
    match &import.from_flow {
        Some(flow_name) => match bot.get(flow_name) {
            Some(flow) => {
                get_function(flow, &import.name, &import.original_name).ok_or(ErrorInfo {
                    position: import.position.clone(),
                    message: format!("function '{}' not found in '{}' flow", import.name, flow_name),
                })
            }
            None => Err(ErrorInfo {
                position: import.position.clone(),
                message: format!("function '{}' not found in '{}' flow", import.name, flow_name),
            }),
        },
        None => {
            for (_name, flow) in bot.iter() {
                if let Some(values) = get_function(flow, &import.name, &import.original_name) {
                    return Ok(values);
                }
            }

            Err(ErrorInfo {
                position: import.position.clone(),
                message: format!("function '{}' not found in bot", import.name),
            })
        }
    }
}
