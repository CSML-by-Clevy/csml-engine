pub mod data;
pub mod error_format;
pub mod interpreter;
pub mod linter;
pub mod fold_bot;
pub mod parser;

pub use interpreter::components::load_components;
pub use parser::step_checksum::get_step;

use interpreter::{interpret_scope, json_to_literal};
use parser::parse_flow;

use data::ast::{Expr, Flow, InstructionScope, Interval};
use data::context::get_hashmap_from_mem;
use data::error_info::ErrorInfo;
use data::event::Event;
use data::message_data::MessageData;
use data::msg::MSG;
use data::CsmlBot;
use data::CsmlResult;
use data::{Context, Data, Position, STEP_LIMIT};
use error_format::*;
use linter::{linter::lint_bot, FlowToValidate};
use fold_bot::fold_bot as fold;
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
    // stop execution if step_count >= STEP_LIMIT in order to avoid infinite loops
    if *data.step_count >= STEP_LIMIT {
        let msg_data = Err(gen_error_info(
            Position::new(
                Interval::new_as_u32(0, 0, 0, None, None),
                &data.context.flow,
            ),
            format!("{}, stop at step {}", ERROR_STEP_LIMIT, step),
        ));

        return MessageData::error_to_message(msg_data, sender);
    }

    let mut msg_data = match flow
        .flow_instructions
        .get(&InstructionScope::StepScope(step.to_owned()))
    {
        Some(Expr::Scope { scope, .. }) => {
            *data.step_count += 1;
            interpret_scope(scope, &mut data, &sender)
        }
        _ => Err(gen_error_info(
            Position::new(
                Interval::new_as_u32(0, 0, 0, None, None),
                &data.context.flow,
            ),
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
    let _ = env_logger::try_init();

    let mut result = HashMap::new();

    for flow in bot.flows.iter() {
        if let Ok(parsed_flow) = parse_flow(&flow.content, &flow.name) {
            let mut vec = vec![];

            for instruction_type in parsed_flow.flow_instructions.keys() {
                if let InstructionScope::StepScope(step_name, ..) = instruction_type {
                    vec.push(step_name.to_owned());
                }
            }
            result.insert(flow.name.to_owned(), vec);
        }
    }
    result
}

pub fn validate_bot(bot: &CsmlBot) -> CsmlResult {
    let _ = env_logger::try_init();

    let mut flows = vec![];
    let mut errors = Vec::new();
    let mut imports = Vec::new();

    for flow in bot.flows.iter() {
        match parse_flow(&flow.content, &flow.name) {
            Ok(ast_flow) => {
                for (scope, ..) in ast_flow.flow_instructions.iter() {
                    if let InstructionScope::ImportScope(import_scope) = scope {
                        imports.push(import_scope.clone());
                    }
                }

                // flows.insert(flow.name.to_owned(), ast_flow);
                flows.push(FlowToValidate {
                    flow_name: flow.name.to_owned(),
                    ast: ast_flow,
                    raw_flow: &flow.content,
                });
            }
            Err(error) => {
                errors.push(error);
            }
        }
    }

    let mut warnings = vec![];
    // only use the linter if there is no error in the paring otherwise the linter will catch false errors
    if errors.is_empty() {
        lint_bot(
            &flows,
            &mut errors,
            &mut warnings,
            &bot.native_components,
            &bot.default_flow,
        );
    }

    CsmlResult::new(FlowToValidate::get_bot(flows), warnings, errors)
}

pub fn fold_bot(bot: &CsmlBot) -> String {
    let _ = env_logger::try_init();

    let mut flows = vec![];
    let mut errors = Vec::new();
    let mut imports = Vec::new();

    for flow in bot.flows.iter() {
        match parse_flow(&flow.content, &flow.name) {
            Ok(ast_flow) => {
                for (scope, ..) in ast_flow.flow_instructions.iter() {
                    if let InstructionScope::ImportScope(import_scope) = scope {
                        imports.push(import_scope.clone());
                    }
                }

                // flows.insert(flow.name.to_owned(), ast_flow);
                flows.push(FlowToValidate {
                    flow_name: flow.name.to_owned(),
                    ast: ast_flow,
                    raw_flow: &flow.content,
                });
            }
            Err(error) => {
                errors.push(error);
            }
        }
    }

    let mut warnings = vec![];
    // only use the fold if there is no error in the paring otherwise the linter will catch false errors

    fold(
        &flows,
        &mut errors,
        &mut warnings,
        &bot.native_components,
        &bot.default_flow,
    )
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
    let _ = env_logger::try_init();

    let mut msg_data = MessageData::default();

    let mut flow = context.flow.to_owned();
    let mut step = context.step.to_owned();

    let mut step_count = 0;

    let mut step_vars = match &context.hold {
        Some(hold) => get_hashmap_from_mem(&hold.step_vars, &flow),
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

    let env = match bot.env {
        Some(env) => json_to_literal(&env, Interval::default(), &flow).unwrap(),
        None => data::primitive::PrimitiveNull::get_literal(Interval::default()),
    };

    let mut previous_info = match &context.hold {
        Some(hold) => match &hold.previous {
            Some(previous) => Some(previous.clone()),
            None => None,
        },
        None => None,
    };

    while msg_data.exit_condition.is_none() {
        let ast = match flows.get(&flow) {
            Some(result) => result.to_owned(),
            None => {
                return MessageData::error_to_message(
                    Err(ErrorInfo {
                        position: Position {
                            flow: flow.clone(),
                            interval: Interval::default(),
                        },
                        message: format!("flow '{}' does not exist in this bot", flow),
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
            &env,
            vec![],
            0,
            &mut step_count,
            step_vars,
            previous_info.clone(),
            &custom,
            &native,
        );

        msg_data = msg_data + execute_step(&step, &ast, &mut data, &sender);
        previous_info = data.previous_info.clone();
        flow = data.context.flow.to_string();
        step = data.context.step.to_string();
        // add reset loops index
        step_vars = HashMap::new();
    }

    msg_data
}
