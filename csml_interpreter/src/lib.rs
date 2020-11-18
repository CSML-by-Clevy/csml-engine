pub mod data;
pub mod error_format;
pub mod imports;
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
use data::warnings::Warnings;
use data::{ContextJson, Data, Position};
use error_format::*;
use imports::validate_imports;
use linter::data::Linter;
use linter::linter::lint_flow;
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

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn get_steps_from_flow(bot: CsmlBot) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();

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
    Warnings::clear();
    Linter::clear();

    result
}

pub fn validate_bot(bot: CsmlBot) -> CsmlResult {
    let mut flows = HashMap::default();
    let mut errors = Vec::new();
    let mut imports = Vec::new();

    for flow in bot.flows.iter() {
        Position::set_flow(&flow.name);
        Linter::add_flow(&flow.name);

        match parse_flow(&flow.content) {
            Ok(ast_flow) => {
                for (scope, ..) in ast_flow.flow_instructions.iter() {
                    if let InstructionScope::ImportScope(import_scope) = scope {
                        imports.push(import_scope.clone());
                    }
                }
                let encoded: Vec<u8> = bincode::serialize(&ast_flow).unwrap();
                println!("flow {} encode => {:?}", flow.name, encoded.len());
                let base64encode = base64::encode(&encoded);
                println!("base64encode => {:?}", base64encode);
                let base64decoded = base64::decode(&base64encode).unwrap();

                // let decoded: Flow = bincode::deserialize(&encoded[..]).unwrap();
                let decoded: Flow = bincode::deserialize(&base64decoded[..]).unwrap();

                flows.insert(flow.name.to_owned(), decoded);
            }
            Err(error) => {
                errors.push(error);
            }
        }
    }

    let warnings = Warnings::get();
    // only use the linter if there is no error in the paring otherwise the linter will catch false errors
    if errors.is_empty() {
        lint_flow(&bot, &mut errors);
        validate_imports(&flows, imports, &mut errors);
    }

    Warnings::clear();
    Linter::clear();
    CsmlResult::new(flows, warnings, errors)
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

    // ######################## TODO: this is temporary as long as we do not receive the ast
    let bot = validate_bot(bot);
    let flows = match bot.flows {
        Some(flows) => flows,
        None => HashMap::new(),
    };
    // ########################

    while msg_data.exit_condition.is_none() {
        Position::set_flow(&flow);

        let ast = match flows.get(&flow) {
            Some(result) => result.to_owned(),
            None => {
                return MessageData::error_to_message(
                    Err(ErrorInfo {
                        position: Position {
                            flow: flow.clone(),
                            step: "start".to_owned(),
                            interval: Interval::default(),
                        },
                        message: format!("flow '{}' dose not exist in this bot", flow),
                    }),
                    &sender,
                );
            }
        };

        let mut data = Data::new(
            &flows,
            &ast,
            &mut context,
            &event,
            step_vars,
            &custom,
            &native,
        );

        msg_data = msg_data + execute_step(&step, &ast, &mut data, &instruction_index, &sender);

        flow = data.context.flow.to_string();
        step = data.context.step.to_string();
        step_vars = HashMap::new();
        instruction_index = None;
    }

    msg_data
}
