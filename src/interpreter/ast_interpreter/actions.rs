use crate::data::execution_context::ExecutionContext;
use crate::data::{
    ast::*, literal::ContentType, message::*, primitive::null::PrimitiveNull, Data, Literal,
    Memories, MemoryType, MessageData, MSG,
};
use crate::error_format::*;
use crate::interpreter::variable_handler::{
    exec_path_actions, expr_to_literal, get_var_from_mem, interval::*, memory::*,
};
use crate::parser::ExitCondition;
use std::sync::mpsc;

fn get_var_info<'a>(
    expr: &'a Expr,
    path: Option<&[(Interval, PathState)]>,
    data: &'a mut Data,
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<
    (
        &'a mut Literal,
        String,
        MemoryType,
        Option<Vec<(Interval, PathLiteral)>>,
    ),
    ErrorInfo,
> {
    match expr {
        Expr::PathExpr { literal, path } => get_var_info(literal, Some(path), data, root, sender),
        Expr::IdentExpr(var) => match search_in_memory_type(var, data) {
            Ok(_) => get_var_from_mem(var.to_owned(), path, data, root, sender),
            Err(_) => {
                let lit = PrimitiveNull::get_literal(var.interval.to_owned());
                data.step_vars.insert(var.ident.to_owned(), lit);
                get_var_from_mem(var.to_owned(), path, data, root, sender)
            }
        },
        e => Err(ErrorInfo::new(
            interval_from_expr(e),
            ERROR_GET_VAR_INFO.to_owned(),
        )),
    }
}

pub fn match_actions(
    function: &ObjectType,
    mut root: MessageData,
    data: &mut Data,
    instruction_index: Option<usize>,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {
    match function {
        ObjectType::Say(arg) => {
            let msg = Message::new(expr_to_literal(arg, None, data, &mut root, sender)?);
            MSG::send(&sender, MSG::Message(msg.clone()));
            Ok(Message::add_to_message(root, MessageType::Msg(msg)))
        }
        ObjectType::Use(arg) => {
            expr_to_literal(arg, None, data, &mut root, sender)?;
            Ok(root)
        }
        ObjectType::Do(DoType::Update(old, new)) => {
            let new_value = expr_to_literal(new, None, data, &mut root, sender)?;
            let (lit, name, mem_type, path) = get_var_info(old, None, data, &mut root, sender)?;
            exec_path_actions(lit, Some(new_value), &path, &ContentType::get(&lit))?;
            save_literal_in_mem(
                lit.to_owned(),
                name,
                &mem_type,
                true,
                data,
                &mut root,
                sender,
            );

            Ok(root)
        }
        ObjectType::Do(DoType::Exec(expr)) => {
            expr_to_literal(expr, None, data, &mut root, sender)?;
            Ok(root)
        }
        ObjectType::Goto(GotoType::Step, step_name) => {
            MSG::send(
                &sender,
                MSG::Next(ExecutionContext::new(
                    ExecutionContext::get_flow(),
                    step_name.ident.clone(),
                )),
            );

            ExecutionContext::set_step(&step_name.ident);

            if step_name.ident == "end" {
                root.exit_condition = Some(ExitCondition::End);
            }

            Ok(root)
        }
        ObjectType::Goto(GotoType::Flow, flow_name) => {
            MSG::send(
                &sender,
                MSG::Next(ExecutionContext::new(
                    flow_name.ident.clone(),
                    "start".to_string(),
                )),
            );

            ExecutionContext::set_flow(&flow_name.ident);
            ExecutionContext::set_step("start");

            Ok(root)
        }
        ObjectType::Remember(name, variable) => {
            let lit = expr_to_literal(variable, None, data, &mut root, sender)?;
            root.add_to_memory(&name.ident, lit.clone());

            MSG::send(
                &sender,
                MSG::Memory(Memories::new(name.ident.to_owned(), lit.clone())),
            );

            data.context.current.insert(name.ident.to_owned(), lit);
            Ok(root)
        }
        // ObjectType::Import {
        //     step_name: name, ..
        // } => {
        //     if let Some(Expr::Scope { scope: actions, .. }) = data
        //         .flow
        //         .flow_instructions
        //         .get(&InstructionType::NormalStep(name.ident.to_owned()))
        //     {
        //         match interpret_scope(actions, data, instruction_index, sender) {
        //             Ok(root2) => Ok(root + root2),
        //             Err(err) => Err(ErrorInfo::new(
        //                 interval_from_reserved_fn(function),
        //                 format!("{} {:?}", ERROR_IMPORT_FAIL, err),
        //             )),
        //         }
        //     } else {
        //         Err(ErrorInfo::new(
        //             interval_from_reserved_fn(function),
        //             format!("{} {}", name.ident, ERROR_IMPORT_STEP_FLOW),
        //         ))
        //     }
        // }
        reserved => Err(ErrorInfo::new(
            interval_from_reserved_fn(reserved),
            ERROR_START_INSTRUCTIONS.to_owned(),
        )),
    }
}
