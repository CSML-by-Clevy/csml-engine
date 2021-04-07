use crate::data::{
    ast::*, tokens::Span,
    position::Position,
    primitive::{PrimitiveClosure, PrimitiveType},
    warnings::*,
    Literal,
};
use crate::error_format::{convert_error_from_interval, gen_error_info, ErrorInfo};
use crate::interpreter::variable_handler::interval::interval_from_expr;
use crate::linter::{FlowToValidate, FunctionInfo, ImportInfo, LinterInfo, State, StepInfo};

use std::collections::HashSet;

pub const ERROR_GOTO_IN_FN: &str = "'goto' action is not allowed in function scope";
pub const ERROR_REMEMBER_IN_FN: &str = "'remember' action is not allowed in function scope";
pub const ERROR_SAY_IN_FN: &str = "'say' action is not allowed in function scope";
pub const ERROR_RETURN_IN_FN: &str = "'return' action is not allowed outside function scope";
pub const ERROR_BREAK_IN_LOOP: &str = "'break' action is not allowed outside loop";
pub const ERROR_CONTINUE_IN_LOOP: &str = "'continue' action is not allowed outside loop";
pub const ERROR_HOLD_IN_LOOP: &str = "'hold' action is not allowed in function scope";

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn validate_expr_literals(to_be_literal: &Expr, state: &mut State, linter_info: &mut LinterInfo) {
    match to_be_literal {
        Expr::ObjectExpr(ObjectType::As(_, value)) => {
            validate_expr_literals(value, state, linter_info)
        }
        Expr::PathExpr { literal, path } => {
            validate_expr_literals(literal, state, linter_info);
            for (_, node) in path.iter() {
                match node {
                    PathState::ExprIndex(expr) => validate_expr_literals(expr, state, linter_info),
                    PathState::Func(Function { args, .. }) => {
                        validate_expr_literals(args, state, linter_info)
                    }
                    _ => {}
                }
            }
        }
        Expr::ObjectExpr(ObjectType::BuiltIn(Function {
            name,
            args,
            interval,
        })) => {
            if name == "Object" {
                linter_info
                    .warnings
                    .push(Warnings::new(linter_info.flow_name, interval.to_owned(), WARNING_OBJECT));
            } else if name == "Fn" {
                linter_info
                    .warnings
                    .push(Warnings::new(linter_info.flow_name,interval.to_owned(), WARNING_FN));
            }
            validate_expr_literals(args, state, linter_info);
        }
        Expr::MapExpr { object, .. } => {
            for (_, expr) in object.iter() {
                validate_expr_literals(expr, state, linter_info);
            }
        }
        Expr::VecExpr(vec, ..) | Expr::ComplexLiteral(vec, ..) => {
            for expr in vec.iter() {
                validate_expr_literals(expr, state, linter_info);
            }
        }
        Expr::InfixExpr(_, exp_1, exp_2) => {
            validate_expr_literals(exp_1, state, linter_info);
            validate_expr_literals(exp_2, state, linter_info);
        }
        Expr::LitExpr { literal, .. } => {
            if literal.primitive.get_type() == PrimitiveType::PrimitiveClosure {
                if let Ok(closure) = Literal::get_value::<PrimitiveClosure>(
                    &literal.primitive,
                    linter_info.flow_name,
                    literal.interval,
                    format!(""),
                ) {
                    if let Expr::Scope { scope, .. } = &*closure.func {
                        state.in_function = true;
                        validate_scope(scope, state, linter_info);
                        state.in_function = false;
                    };
                }
            }
        }
        Expr::ObjectExpr(ObjectType::Assign(target, new)) => {
            validate_expr_literals(target, state, linter_info);
            validate_expr_literals(new, state, linter_info);
        }
        Expr::IdentExpr(..) => {}
        _ => {}
    }
}

fn validate_if_scope(if_statement: &IfStatement, state: &mut State, linter_info: &mut LinterInfo) {
    match if_statement {
        IfStatement::IfStmt {
            consequence,
            then_branch,
            ..
        } => {
            match then_branch {
                Some(else_scope) => validate_if_scope(else_scope, state, linter_info),
                None => {}
            };

            validate_scope(consequence, state, linter_info);
        }
        IfStatement::ElseStmt(block, ..) => validate_scope(block, state, linter_info),
    }
}

fn validate_scope(scope: &Block, state: &mut State, linter_info: &mut LinterInfo) {
    for (action, _) in scope.commands.iter() {
        match action {
            Expr::ObjectExpr(ObjectType::Do(DoType::Update(target, new))) => {
                validate_expr_literals(target, state, linter_info);
                validate_expr_literals(new, state, linter_info);
            }
            Expr::ObjectExpr(ObjectType::Do(DoType::Exec(expr))) => {
                validate_expr_literals(expr, state, linter_info);
            }

            Expr::ObjectExpr(ObjectType::Return(value)) => {
                if !state.in_function {
                    linter_info.errors.push(gen_error_info(
                        Position::new(
                            interval_from_expr(value),
                            linter_info.flow_name,
                        ),
                        convert_error_from_interval(
                            Span::new(linter_info.raw_flow),
                            ERROR_RETURN_IN_FN.to_owned(),
                            interval_from_expr(value),
                        ),
                    ));
                }
            }
            Expr::ObjectExpr(ObjectType::Goto(goto, interval)) => {
                if state.in_function {
                    linter_info.errors.push(gen_error_info(
                        Position::new(interval.to_owned(), linter_info.flow_name,),
                        convert_error_from_interval(
                            Span::new(linter_info.raw_flow),
                            ERROR_GOTO_IN_FN.to_owned(),
                            interval.to_owned(),
                        ),
                    ));
                }
                match goto {
                    GotoType::Step(GotoValueType::Name(step)) => {
                        linter_info.goto_list.push(StepInfo::new(
                            linter_info.flow_name,
                            &step.ident,
                            linter_info.raw_flow,
                            interval.to_owned(),
                        ))
                    }
                    GotoType::Flow(GotoValueType::Name(flow)) => {
                        linter_info.goto_list.push(StepInfo::new(
                            &flow.ident,
                            "start",
                            linter_info.raw_flow,
                            interval.to_owned(),
                        ))
                    }
                    GotoType::StepFlow {
                        step: Some(GotoValueType::Name(step)),
                        flow: Some(GotoValueType::Name(flow)),
                    } => linter_info.goto_list.push(StepInfo::new(
                            &flow.ident,
                            &step.ident,
                            linter_info.raw_flow,
                            interval.to_owned(),
                    )),
                    GotoType::StepFlow {
                        step: None,
                        flow: Some(GotoValueType::Name(flow)),
                    } => linter_info.goto_list.push(StepInfo::new(
                            &flow.ident,
                            "start",
                            linter_info.raw_flow,
                            interval.to_owned(),
                    )),
                    GotoType::StepFlow {
                        step: Some(GotoValueType::Name(step)),
                        flow: None,
                    } => linter_info.goto_list.push(StepInfo::new(
                            &linter_info.flow_name,
                            &step.ident,
                            linter_info.raw_flow,
                            interval.to_owned(),
                    )),
                    GotoType::StepFlow {
                        step: None,
                        flow: None,
                    } => linter_info.goto_list.push(StepInfo::new(
                            &linter_info.flow_name,
                            "start",
                            linter_info.raw_flow,
                            interval.to_owned(),
                    )),
                    _ => {}
                }
            }

            Expr::ObjectExpr(ObjectType::Break(interval)) => {
                if state.loop_scope == 0 {
                    linter_info.errors.push(gen_error_info(
                        Position::new(interval.to_owned(), linter_info.flow_name,),
                        convert_error_from_interval(
                            Span::new(linter_info.raw_flow),
                            ERROR_BREAK_IN_LOOP.to_owned(),
                            interval.to_owned(),
                        ),
                    ));
                }
            }
            Expr::ObjectExpr(ObjectType::Continue(interval)) => {
                if state.loop_scope == 0 {
                    linter_info.errors.push(gen_error_info(
                        Position::new(interval.to_owned(), linter_info.flow_name,),
                        convert_error_from_interval(
                            Span::new(linter_info.raw_flow),
                            ERROR_CONTINUE_IN_LOOP.to_owned(),
                            interval.to_owned(),
                        ),
                    ));
                }
            }

            Expr::ObjectExpr(ObjectType::Hold(interval)) => {
                if state.in_function {
                    linter_info.errors.push(gen_error_info(
                        Position::new(interval.to_owned(), linter_info.flow_name,),
                        convert_error_from_interval(
                            Span::new(linter_info.raw_flow),
                            ERROR_HOLD_IN_LOOP.to_owned(),
                            interval.to_owned(),
                        ),
                    ));
                }
            }
            Expr::ObjectExpr(ObjectType::Say(value)) => {
                if state.in_function {
                    linter_info.errors.push(gen_error_info(
                        Position::new(interval_from_expr(value), linter_info.flow_name,),
                        convert_error_from_interval(
                            Span::new(linter_info.raw_flow),
                            ERROR_SAY_IN_FN.to_owned(),
                            interval_from_expr(value),
                        ),
                    ));
                }

                validate_expr_literals(value, state, linter_info);
            }

            Expr::ObjectExpr(ObjectType::Use(value)) => {
                linter_info
                    .warnings
                    .push(Warnings::new(linter_info.flow_name, interval_from_expr(value), WARNING_USE));
                validate_expr_literals(value, state, linter_info);
            }

            Expr::ObjectExpr(ObjectType::Remember(name, value)) => {
                if state.in_function {
                    linter_info.errors.push(gen_error_info(
                        Position::new(name.interval.to_owned(), linter_info.flow_name,),
                        convert_error_from_interval(
                            Span::new(linter_info.raw_flow),
                            ERROR_REMEMBER_IN_FN.to_owned(),
                            name.interval.to_owned(),
                        ),
                    ));
                }
                validate_expr_literals(value, state, linter_info);
            }

            Expr::IfExpr(if_statement) => {
                validate_if_scope(if_statement, state, linter_info);
            }
            Expr::ForEachExpr(_ident, _index, _expr, block, _range) => {
                state.enter_loop();
                validate_scope(block, state, linter_info);
                state.exit_loop();
            }
            _ => {}
        }
    }
}

fn validate_gotos(linter_info: &mut LinterInfo) {
    for goto_info in linter_info.goto_list.iter() {
        if goto_info.step == "end" {
            continue;
        }

        if let None = linter_info.step_list.get(&goto_info) {
            linter_info.errors.push(gen_error_info(
                Position::new(goto_info.interval.to_owned(), linter_info.flow_name,),
                convert_error_from_interval(
                    Span::new(goto_info.raw_flow),
                    format!(
                        "step {} at flow {} dose not exist",
                        goto_info.step, goto_info.flow
                    ),
                    goto_info.interval.to_owned(),
                ),
            ));
        }
    }
}

fn gen_function_error(
    errors: &mut Vec<ErrorInfo>,
    raw_flow: &str,
    flow_name: &str,
    interval: Interval,
    message: String,
) {
    errors.push(gen_error_info(
        Position::new(interval.to_owned(), flow_name,),
        convert_error_from_interval(Span::new(raw_flow), message, interval),
    ));
}

fn validate_imports(linter_info: &mut LinterInfo) {
    'outer: for import_info in linter_info.import_list.iter() {
        if let Some(_) = linter_info.function_list.get(&FunctionInfo::new(
            import_info.as_name.to_owned(),
            import_info.in_flow,
            import_info.raw_flow,
            import_info.interval.to_owned(),
        )) {
            gen_function_error(
                linter_info.errors,
                import_info.raw_flow,
                linter_info.flow_name,
                import_info.interval.to_owned(),
                format!(
                    "import failed a function named '{}' already exist in current flow '{}'",
                    import_info.as_name, import_info.in_flow
                ),
            );
        };

        match import_info {
            ImportInfo {
                as_name,
                original_name,
                from_flow: Some(flow),
                raw_flow,
                interval,
                ..
            } => {
                let as_name = match original_name {
                    Some(name) => name,
                    None => as_name,
                };

                if let None = linter_info.function_list.get(&FunctionInfo::new(
                    as_name.to_owned(),
                    flow,
                    raw_flow,
                    interval.to_owned(),
                )) {
                    gen_function_error(
                        linter_info.errors,
                        raw_flow,
                        linter_info.flow_name,
                        interval.to_owned(),
                        format!(
                            "import failed function '{}' not found in flow '{}'",
                            as_name, flow
                        ),
                    );
                };
            }
            ImportInfo {
                as_name,
                original_name,
                raw_flow,
                interval,
                ..
            } => {
                let as_name = match original_name {
                    Some(name) => name,
                    None => as_name,
                };

                for function in linter_info.function_list.iter() {
                    if &function.name == as_name {
                        continue 'outer;
                    }
                }

                gen_function_error(
                    linter_info.errors,
                    raw_flow,
                    linter_info.flow_name,
                    interval.to_owned(),
                    format!("function '{}' not found in bot", as_name,),
                );
            }
        }
    }
}

fn validate_flow_ast(flow: &FlowToValidate, linter_info: &mut LinterInfo) {
    let mut is_step_start_present = false;

    for (instruction_scope, scope) in flow.ast.flow_instructions.iter() {
        match instruction_scope {
            InstructionScope::StepScope(step_name) => {
                if step_name == "start" {
                    is_step_start_present = true;
                }

                if let Expr::Scope { scope, range, .. } = scope {
                    linter_info.step_list.insert(StepInfo::new(
                        &flow.flow_name,
                        step_name,
                        linter_info.raw_flow,
                        range.to_owned(),
                    ));

                    validate_scope(scope, &mut State::new(false), linter_info);
                }
            }
            InstructionScope::FunctionScope { name, .. } => {
                if let Expr::Scope { scope, .. } = scope {
                    validate_scope(scope, &mut State::new(true), linter_info);
                }

                linter_info.function_list.insert(FunctionInfo::new(
                    name.to_owned(),
                    linter_info.flow_name,
                    linter_info.raw_flow,
                    interval_from_expr(scope),
                ));
            }
            InstructionScope::ImportScope(import_scope) => {
                linter_info.import_list.insert(ImportInfo::new(
                    import_scope.name.to_owned(),
                    import_scope.original_name.to_owned(),
                    import_scope.from_flow.to_owned(),
                    linter_info.flow_name,
                    linter_info.raw_flow,
                    import_scope.interval.to_owned(),
                ));
            }

            InstructionScope::DuplicateInstruction(interval, info) => {
                linter_info.errors.push(gen_error_info(
                    Position::new(interval.to_owned(), linter_info.flow_name,),
                    convert_error_from_interval(
                        Span::new(flow.raw_flow),
                        format!("duplicate {}", info),
                        interval.to_owned(),
                    ),
                ));
            }
        }
    }

    if !is_step_start_present {
        linter_info.errors.push(gen_error_info(
            Position::new(Interval::default(), linter_info.flow_name),
            format!("missing step 'start' in flow [{}]", flow.flow_name),
        ));
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn lint_bot(
    flows: &[FlowToValidate],
    errors: &mut Vec<ErrorInfo>,
    warnings: &mut Vec<Warnings>,
) {
    let mut goto_list = vec![];
    let mut step_list = HashSet::new();
    let mut function_list = HashSet::new();
    let mut import_list = HashSet::new();

    let mut linter_info = LinterInfo::new(
        "",
        "",
        &mut goto_list,
        &mut step_list,
        &mut function_list,
        &mut import_list,
        errors,
        warnings,
    );

    for flow in flows.iter() {
        linter_info.flow_name = &flow.flow_name;
        linter_info.raw_flow = flow.raw_flow;

        validate_flow_ast(flow, &mut linter_info);
    }

    validate_gotos(&mut linter_info);
    validate_imports(&mut linter_info);
}
