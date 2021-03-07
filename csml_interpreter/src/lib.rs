pub mod data;
pub mod error_format;
pub mod imports;
pub mod interpreter;
pub mod linter;
pub mod parser;

pub use interpreter::components::load_components;
pub use parser::step_checksum::get_step;

use interpreter::interpret_scope;
use parser::parse_flow;

use data::ast::{Expr, Flow, InstructionScope, Interval};
use data::context::get_hashmap_from_mem;
use data::error_info::ErrorInfo;
use data::event::Event;
use data::message_data::MessageData;
use data::msg::MSG;
use data::warnings::Warnings;
use data::CsmlBot;
use data::CsmlResult;
use data::{Context, Data, Position};
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
    sender: &Option<mpsc::Sender<MSG>>,
) -> MessageData {
    let mut msg_data = match flow
        .flow_instructions
        .get(&InstructionScope::StepScope(step.to_owned()))
    {
        Some(Expr::Scope { scope, .. }) => {
            Position::set_step(&step);

            interpret_scope(scope, &mut data, &sender)
        }
        _ => Err(gen_error_info(
            Position::new(Interval::new_as_u32(0, 0, 0, None, None)),
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

pub fn validate_bot(bot: &CsmlBot) -> CsmlResult {
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

                flows.insert(flow.name.to_owned(), ast_flow);
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

fn get_flows(bot: &CsmlBot) -> HashMap<String, Flow> {
    match &bot.bot_ast {
        Some(bot) => {
            let base64decoded = base64::decode(&bot).unwrap();
            bincode::deserialize(&base64decoded[..]).unwrap()
        }
        None => {
            let bot = validate_bot(&bot);
            match bot.flows {
                Some(flows) => flows,
                None => HashMap::new(),
            }
        }
    }
}

pub fn interpret(
    bot: CsmlBot,
    mut context: Context,
    event: Event,
    sender: Option<mpsc::Sender<MSG>>,
) -> MessageData {
    let mut msg_data = MessageData::default();

    let mut flow = context.flow.to_owned();
    let mut step = context.step.to_owned();

    let mut step_vars = match &context.hold {
        Some(hold) => get_hashmap_from_mem(&hold.step_vars),
        None => HashMap::new(),
    };

    let native = match bot.native_components {
        Some(ref obj) => obj.to_owned(),
        None => serde_json::Map::new(),
    };

    let custom = match bot.custom_components {
        Some(serde_json::Value::Object(ref obj)) => obj.to_owned(),
        _ => serde_json::Map::new(),
    };

    let flows = get_flows(&bot);

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
            vec![],
            0,
            step_vars,
            &custom,
            &native,
        );

        msg_data = msg_data + execute_step(&step, &ast, &mut data, &sender);
        flow = data.context.flow.to_string();
        step = data.context.step.to_string();
        // add reset loops index
        step_vars = HashMap::new();
    }

    msg_data
}
