pub mod data;
pub mod error_format;
pub mod interpreter;
pub mod linter;
pub mod parser;

pub use interpreter::components::load_components;

use interpreter::interpret_scope;
use parser::parse_flow;

use data::ast::{Expr, Flow, InstructionScope, Interval};
use data::context::get_hashmap_from_mem;
use data::csml_bot::CsmlBot;
use data::csml_result::CsmlResult;
use data::error_info::ErrorInfo;
use data::event::Event;
use data::message_data::MessageData;
use data::msg::MSG;
use data::position::Position;
use data::warnings::Warnings;
use data::ContextJson;
use data::Data;
use error_format::*;
use linter::data::Linter;
use linter::linter::lint_flow;
use parser::state_context::StateContext;
use parser::ExitCondition;

use std::collections::HashMap;
use std::sync::mpsc;

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn execute_step(
    step: &str,
    flow: &Flow,
    mut data: &mut Data,
    instruction_index: &Option<usize>,
    sender: &Option<mpsc::Sender<MSG>>,
) -> MessageData {
    let mut msg_data = match flow
        .flow_instructions
        .get(&InstructionScope::StepScope(step.to_owned()))
    {
        Some(Expr::Scope { scope, .. }) => {
            Position::set_step(&step);

            interpret_scope(scope, &mut data, &instruction_index, &sender)
        }
        _ => Err(gen_error_info(
            Position::new(Interval::new_as_u32(0, 0)),
            format!("[{}] {}", step, ERROR_STEP_EXIST),
        )),
    };

    if let Ok(msg_data) = &mut msg_data {
        match &mut msg_data.exit_condition {
            Some(condition) if *condition == ExitCondition::Goto => {
                msg_data.exit_condition = None;
            }
            Some(_) => (),
            // if no goto at the end of the scope end conversation
            None => {
                msg_data.exit_condition = Some(ExitCondition::End);
                data.context.step = "end".to_string();
                MSG::send(
                    &sender,
                    MSG::Next {
                        flow: None,
                        step: Some("end".to_owned()),
                    },
                );
            }
        }
    }

    MessageData::error_to_message(msg_data, sender)
}

fn get_ast(
    // bot: &CsmlBot,
    flow_name: &str,
    hashmap: &mut HashMap<String, Flow>,
) -> Result<Flow, Vec<ErrorInfo>> {
    // let content = bot.get_flow(&flow_name)?;

    return match hashmap.get(flow_name) {
        Some(ast) => Ok(ast.to_owned()),
        None => {
            unimplemented!()
            // Position::set_flow(&flow_name);
            // Warnings::clear();

            // match parse_flow(&content) {
            //     Ok(result) => {
            //         hashmap.insert(flow_name.to_owned(), result.to_owned());

            //         Ok(result)
            //     }
            //     Err(error) => Err(vec![error]),
            // }
        }
    };
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn get_steps_from_flow(bot: CsmlBot) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();

    Warnings::clear();
    Linter::clear();

    for flow in bot.flows.iter() {
        if let Ok(parsed_flow) = parse_flow(&flow.content) {
            let mut vec = vec![];

            for instruction_type in parsed_flow.flow_instructions.keys() {
                if let InstructionScope::StepScope(step_name) = instruction_type {
                    vec.push(step_name.to_owned());
                }
            }
            result.insert(flow.name.to_owned(), vec);
        }
    }

    result
}

// ###################################################################

// fn get_fn_names(vec: &[Expr]) -> Vec<(String, Option<String>)> {
//         vec
//         .iter()
//         .map(|value| match value {
//             Expr::IdentExpr(ident) => (ident.ident.to_owned(), None),
//             Expr::ObjectExpr(ObjectType::As(name, expr)) => {
//                 let ident = match &**expr {
//                     Expr::IdentExpr(ident) => ident.to_owned(),
//                     _ => unimplemented!()
//                 };

//                 (ident.ident.to_owned(), Some(name.ident.to_owned()) )
//             },
//             _ => unimplemented!(),
//         })
//         .collect()
// }

// fn import_functions(bot: &mut HashMap<String, Flow>, import: HashMap<ImportScope, Expr>) {
//     for (instruction, expr) in import.iter() {
//         if let (
//             ImportScope {
//                 at_flow,
//                 from_flow,
//                 position: _position,
//             },
//             Expr::VecExpr(vec, ..),
//         ) = (instruction, expr)
//         {
//             let fn_names = get_fn_names(vec);

//             for (fn_name, as_name) in fn_names.iter() {
//                 match search_function(bot, fn_name, as_name,from_flow) {
//                     Some((scope, expr)) => {
//                         let flow = bot.get_mut(at_flow).unwrap();
//                         flow.flow_instructions.insert(scope, expr);
//                     }
//                     None => unimplemented!(),
//                 }
//             }
//         }
//     }
// }

fn get_function<'a>(flow: &'a Flow, fn_name: &str, original_name: &Option<String>) -> Option<(Vec<String>, Expr, &'a Flow)> {

    let name = match original_name {
        Some(original_name) => original_name.to_owned(),
        None => fn_name.to_owned()
    };

    if let ( InstructionScope::FunctionScope{name: _, args} , expr) = flow 
        .flow_instructions
        .get_key_value(&InstructionScope::FunctionScope {
            name,
            args: Vec::new(),
        })? 
    {
       return Some((args.to_owned() , expr.to_owned(), flow))
    }
    None
}

pub fn search_function<'a>(
    bot: &'a HashMap<String, Flow>,
    fn_name: &str,
    original_name: &Option<String>,
    from_flow: &Option<String>,
) -> Option<(Vec<String>, Expr, &'a Flow)> {
    match from_flow {
        Some(flow_name) => match bot.get(flow_name) {
            Some(flow) => get_function(flow, fn_name, original_name),
            None => None,
        },
        None => {
            for (_name, flow) in bot.iter() {
                if let Some(values) = get_function(flow, fn_name, original_name) {
                    return Some(values);
                }
            }
            None
        }
    }
}

// ###################################################################

pub fn validate_bot(bot: CsmlBot) -> CsmlResult {
    let mut flows = HashMap::default();
    let mut errors = Vec::new();

    Warnings::clear();
    Linter::clear();

    for flow in bot.flows.iter() {
        Position::set_flow(&flow.name);
        Linter::add_flow(&flow.name);

        match parse_flow(&flow.content) {
            Ok(ast_flow) => {
                flows.insert(flow.name.to_owned(), ast_flow);
            }
            Err(error) => {
                errors.push(error);
            }
        }
    }

    // validate import
    // import_functions(&mut flows, imports);
    lint_flow(&bot, &mut errors);

    CsmlResult::new(flows, Warnings::get(), errors)
}

//TODO: received ast instead of bot
pub fn interpret(
    bot: CsmlBot,
    context: ContextJson,
    event: Event,
    sender: Option<mpsc::Sender<MSG>>,
) -> MessageData {
    let mut msg_data = MessageData::default();
    let mut context = context.to_literal();

    let mut flow = context.flow.to_owned();
    let mut step = context.step.to_owned();
    // let mut hashmap: HashMap<String, Flow> = HashMap::default();

    let mut step_vars = match &context.hold {
        Some(hold) => get_hashmap_from_mem(&hold.step_vars),
        None => HashMap::new(),
    };

    let mut instruction_index = match context.hold {
        Some(result) => {
            context.hold = None;
            Some(result.index)
        }
        None => None,
    };

    let native = match bot.native_components {
        Some(ref obj) => obj.to_owned(),
        None => serde_json::Map::new(),
    };

    let custom = match bot.custom_components {
        Some(serde_json::Value::Object(ref obj)) => obj.to_owned(),
        _ => serde_json::Map::new(),
    };

    // TMP #####################
        let bot = validate_bot(bot);
        let mut flows = match bot.flows {
            Some(flows) => flows,
            None => unimplemented!()
        };
    // ########################


    Warnings::clear();
    Linter::clear();
    while msg_data.exit_condition.is_none() {
        Position::set_flow(&flow);

        let ast = match get_ast(&flow, &mut flows) {
            Ok(result) => result,
            Err(error) => {
                StateContext::clear_state();
                StateContext::clear_rip();

                let mut msg_data = MessageData::default();

                for err in error {
                    msg_data = msg_data + MessageData::error_to_message(Err(err), &None);
                }

                return msg_data;
            }
        };

        let mut data = Data::new(&flows, &ast, &mut context, &event, step_vars, &custom, &native);

        msg_data = msg_data + execute_step(&step, &ast, &mut data, &instruction_index, &sender);

        flow = data.context.flow.to_string();
        step = data.context.step.to_string();
        step_vars = HashMap::new();
        instruction_index = None;
    }

    msg_data
}
