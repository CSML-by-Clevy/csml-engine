use crate::data::{
    ast::*, tokens::{Span, BUILT_IN, COMPONENT},
    position::Position,
    primitive::{PrimitiveClosure, PrimitiveType},
    warnings::*,
    Literal,
};
use crate::error_format::{convert_error_from_interval, gen_error_info, gen_warning_info, gen_infinite_loop_error_msg, ErrorInfo};
use crate::interpreter::variable_handler::interval::interval_from_expr;
use crate::linter::{
    FlowToValidate, FunctionInfo, ImportInfo, LinterInfo, 
    State, StepInfo, StepBreakers, FunctionCallInfo, ScopeType
};

use std::collections::HashSet;

pub const ERROR_GOTO_IN_FN: &str = "'goto' action is not allowed in function scope";
pub const ERROR_REMEMBER_IN_FN: &str = "'remember' action is not allowed in function scope";
pub const ERROR_SAY_IN_FN: &str = "'say' action is not allowed in function scope";
pub const ERROR_RETURN_IN_FN: &str = "'return' action is not allowed outside function scope";
pub const ERROR_BREAK_IN_LOOP: &str = "'break' action is not allowed outside loop";
pub const ERROR_CONTINUE_IN_LOOP: &str = "'continue' action is not allowed outside loop";
pub const ERROR_HOLD_IN_LOOP: &str = "'hold' action is not allowed in function scope";


////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn lint_bot(
    flows: &[FlowToValidate],
    errors: &mut Vec<ErrorInfo>,
    warnings: &mut Vec<Warnings>,
    native_components: &Option<serde_json::Map<String, serde_json::Value>>,
    default_flow: &str,
) {
    let scope_type = ScopeType::Step("start".to_owned());
    let mut goto_list = vec![];
    let mut step_list = HashSet::new();
    let mut function_list = HashSet::new();
    let mut import_list = HashSet::new();
    let mut valid_closure_list = vec![];
    let mut functions_call_list = vec![];

    let mut linter_info = LinterInfo::new(
        "",
        scope_type,
        "",
        &mut goto_list,
        &mut step_list,
        &mut function_list,
        &mut import_list,
        &mut valid_closure_list,
        &mut functions_call_list,
        errors,
        warnings,
        native_components,
    );

    for flow in flows.iter() {
        linter_info.flow_name = &flow.flow_name;
        linter_info.raw_flow = flow.raw_flow;

        validate_flow_ast(flow, &mut linter_info);
    }

    validate_gotos(&mut linter_info);
    validate_imports(&mut linter_info);
    validate_functions(&mut linter_info);

    match infinite_loop_check(&linter_info, vec![], &mut vec![], default_flow.to_owned(), "start".to_owned()) {
        Some((infinite_loop, interval, flow)) => {
            linter_info.warnings.push(gen_warning_info(
                Position::new(interval, &flow),
                format!("infinite loop detected between:\n {}", gen_infinite_loop_error_msg(infinite_loop)),
            ));
        },
        None => {}
    }
}


pub fn validate_gotos(linter_info: &mut LinterInfo) {
    for goto_info in linter_info.goto_list.iter() {
        if goto_info.step == "end" {
            continue;
        }

        if let None = linter_info.step_list.get(&goto_info) {
            linter_info.errors.push(gen_error_info(
                Position::new(goto_info.interval.to_owned(), &goto_info.in_flow,),
                convert_error_from_interval(
                    Span::new(goto_info.raw_flow),
                    format!(
                        "step {} at flow {} does not exist",
                        goto_info.step, goto_info.flow
                    ),
                    goto_info.interval.to_owned(),
                ),
            ));
        }
    }
}

pub fn validate_imports(linter_info: &mut LinterInfo) {
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
                in_flow,
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
                        in_flow,
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

pub fn validate_functions(linter_info: &mut LinterInfo) {

    for info in linter_info.functions_call_list.iter() {

        let is_native_component = match linter_info.native_components {
            Some(native_component) => native_component.contains_key(&info.name),
            None  => false
        };

        if !is_native_component && 
            !BUILT_IN.contains(&info.name.as_str()) && 
            COMPONENT != info.name &&
            !validate_closure(&info, linter_info) &&
            !function_exist(&info, linter_info)
        {
            linter_info.errors.push(gen_error_info(
                Position::new(info.interval.to_owned(), info.in_flow,),
                convert_error_from_interval(
                    Span::new(info.raw_flow),
                    format!("function [{}] does not exist", info.name),
                    info.interval.to_owned(),
                ),
            ));
        }
    }
}

pub fn validate_flow_ast(flow: &FlowToValidate, linter_info: &mut LinterInfo) {
    let mut is_step_start_present = false;

    for (instruction_scope, scope) in flow.ast.flow_instructions.iter() {
        match instruction_scope {
            InstructionScope::StepScope(step_name) => {
                if step_name == "start" {
                    is_step_start_present = true;
                }
                linter_info.scope_type = ScopeType::Step(step_name.to_owned());

                if let Expr::Scope { scope, range, .. } = scope {
                    let mut step_breakers  = vec!();

                    validate_scope(scope, &mut State::new(0), linter_info, &mut Some(&mut step_breakers));

                    linter_info.step_list.insert(StepInfo::new(
                        &flow.flow_name,
                        step_name,
                        linter_info.raw_flow,
                        flow.flow_name.clone(),
                        step_breakers,
                        range.to_owned(),
                    ));
                }
            }
            InstructionScope::FunctionScope { name, .. } => {
                let save_step_name = linter_info.scope_type.clone();
                linter_info.scope_type = ScopeType::Function(name.to_owned());

                if let Expr::Scope { scope, .. } = scope {
                    validate_scope(scope, &mut State::new(1), linter_info, &mut None);
                }

                linter_info.scope_type = save_step_name;


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
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn register_closure(name: &Identifier, is_permanent: bool, expr: &Expr, linter_info: &mut LinterInfo) {
    if let Expr::LitExpr{ literal , ..} = expr {
        // register closure var name for function validation
        if literal.primitive.get_type() == PrimitiveType::PrimitiveClosure {
            linter_info.valid_closure_list.push(
                FunctionCallInfo::new(
                    name.ident.to_owned(),
                    linter_info.flow_name,
                    linter_info.scope_type.clone(),
                    is_permanent,
                    linter_info.raw_flow,
                    name.interval.to_owned(),
                )
            );
        }
    }
}

fn register_flow_breaker(step_breakers: &mut Option<&mut Vec<StepBreakers>>, breaker: StepBreakers) {
    if let Some(step_breakers) = step_breakers {
        step_breakers.push(breaker);
    }
}

fn is_in_list(list: &Vec<(String, String)>, flow: &str, step: &str) -> bool {
    list
    .iter()
    .any(|(next_flow, next_step)| flow == next_flow && step == next_step)
}

fn validate_expr_literals(to_be_literal: &Expr, state: &mut State, linter_info: &mut LinterInfo) {
    match to_be_literal {
        Expr::ObjectExpr(ObjectType::As(name, value)) => {
            register_closure(name, false, value, linter_info);

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

            linter_info.functions_call_list.push(
                FunctionCallInfo::new(
                    name.to_owned(),
                    linter_info.flow_name,
                    linter_info.scope_type.clone(),
                    false,
                    linter_info.raw_flow,
                    interval.to_owned(),
                )
            );

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
                        state.in_function += 1;
                        validate_scope(scope, state, linter_info, &mut None);
                        state.in_function -= 1;
                    };
                }
            }
        }
        Expr::ObjectExpr(ObjectType::Assign(_assign, target, new)) => {
            validate_expr_literals(target, state, linter_info);
            validate_expr_literals(new, state, linter_info);
        }
        Expr::IdentExpr(..) => {}
        _ => {}
    }
}

fn validate_if_scope(
    if_statement: &IfStatement,
    state: &mut State,
    linter_info: &mut LinterInfo,
    step_breakers: &mut Option<&mut Vec<StepBreakers>>
) {
    match if_statement {
        IfStatement::IfStmt {
            consequence,
            then_branch,
            ..
        } => {
            validate_scope(consequence, state, linter_info, step_breakers);

            match then_branch {
                Some(else_scope) => validate_if_scope(else_scope, state, linter_info, step_breakers),
                None => {}
            };
        }
        IfStatement::ElseStmt(block, ..) => validate_scope(block, state, linter_info, step_breakers),
    }
}

fn validate_scope(
    scope: &Block,
    state: &mut State,
    linter_info: &mut LinterInfo,
    step_breakers: &mut Option<&mut Vec<StepBreakers>>
) {
    for (action, _) in scope.commands.iter() {
        match action {
            Expr::ObjectExpr(ObjectType::Return(value)) => {
                if state.in_function == 0 {
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
                if state.in_function > 0 {
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
                        register_flow_breaker(step_breakers, StepBreakers::GOTO {
                                flow: linter_info.flow_name.to_owned(),
                                step: step.ident.to_owned(),
                                interval: interval.to_owned()
                            }
                        );

                        linter_info.goto_list.push(StepInfo::new(
                            linter_info.flow_name,
                            &step.ident,
                            linter_info.raw_flow,
                            linter_info.flow_name.to_owned(),
                            vec!(),
                            interval.to_owned(),
                        ))
                    }
                    GotoType::Flow(GotoValueType::Name(flow)) => {
                        register_flow_breaker(step_breakers, StepBreakers::GOTO {
                                flow: flow.ident.to_owned(),
                                step: "start".to_owned(),
                                interval: interval.to_owned()
                            }
                        );

                        linter_info.goto_list.push(StepInfo::new(
                            &flow.ident,
                            "start",
                            linter_info.raw_flow,
                            linter_info.flow_name.to_owned(),
                            vec!(),
                            interval.to_owned(),
                        ))
                    }
                    GotoType::StepFlow {
                        step: Some(GotoValueType::Name(step)),
                        flow: Some(GotoValueType::Name(flow)),
                    } => {
                        register_flow_breaker(step_breakers, StepBreakers::GOTO {
                                flow: flow.ident.to_owned(),
                                step: step.ident.to_owned(),
                                interval: interval.to_owned()
                            }
                        );

                        linter_info.goto_list.push(StepInfo::new(
                            &flow.ident,
                            &step.ident,
                            linter_info.raw_flow,
                            linter_info.flow_name.to_owned(),
                            vec!(),
                            interval.to_owned(),
                        ))
                    },
                    GotoType::StepFlow {
                        step: None,
                        flow: Some(GotoValueType::Name(flow)),
                    } => {
                        register_flow_breaker(step_breakers, StepBreakers::GOTO {
                                flow: flow.ident.to_owned(),
                                step: "start".to_owned(),
                                interval: interval.to_owned()
                            }
                        );

                        linter_info.goto_list.push(StepInfo::new(
                            &flow.ident,
                            "start",
                            linter_info.raw_flow,
                            linter_info.flow_name.to_owned(),
                            vec!(),
                            interval.to_owned(),
                        ))
                    },
                    GotoType::StepFlow {
                        step: Some(GotoValueType::Name(step)),
                        flow: None,
                    } => {
                        register_flow_breaker(step_breakers, StepBreakers::GOTO {
                                flow: linter_info.flow_name.to_owned(),
                                step: step.ident.to_owned(),
                                interval: interval.to_owned()
                            }
                        );

                        linter_info.goto_list.push(StepInfo::new(
                            &linter_info.flow_name,
                            &step.ident,
                            linter_info.raw_flow,
                            linter_info.flow_name.to_owned(),
                            vec!(),
                            interval.to_owned(),
                        ))
                    },
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

                register_flow_breaker(step_breakers, StepBreakers::HOLD(interval.clone()));

                if state.in_function > 0 {
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
                if state.in_function > 0 {
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

            Expr::ObjectExpr(ObjectType::Do(DoType::Update(_assign, target, new))) => {

                if let Expr::IdentExpr(name) = &**target {
                    register_closure(name, false,new, linter_info);
                }

                validate_expr_literals(target, state, linter_info);
                validate_expr_literals(new, state, linter_info);
            }
            Expr::ObjectExpr(ObjectType::Do(DoType::Exec(expr))) => {
                validate_expr_literals(expr, state, linter_info);
            }

            Expr::ObjectExpr(ObjectType::Remember(ref name, value)) => {
                register_closure(name, true,value, linter_info);

                if state.in_function > 0 {
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
                validate_if_scope(if_statement, state, linter_info, step_breakers);
            }
            Expr::ForEachExpr(_ident, _index, _expr, block, _range) => {
                state.enter_loop();
                validate_scope(block, state, linter_info, step_breakers);
                state.exit_loop();
            }
            Expr::WhileExpr(_expr, block, _range) => {
                state.enter_loop();
                validate_scope(block, state, linter_info, step_breakers);
                state.exit_loop();
            }
            _ => {}
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


fn function_exist(info: &FunctionCallInfo, linter_info: &LinterInfo) -> bool {
    match linter_info.function_list.iter().find(|&func| func.name == info.name && func.in_flow == info.in_flow) {
        Some(_) => return true,
        None => {},
    }

    match linter_info.import_list.iter().find(|&import| import.as_name == info.name && import.in_flow == info.in_flow) {
        Some(_) => true,
        None => false
    }
}

fn validate_closure(info: &FunctionCallInfo, linter_info: &LinterInfo) -> bool {
    match linter_info.valid_closure_list.iter().find(|&func| {
        func.name == info.name &&
        (
            func.scope_type == info.scope_type ||
            func.is_permanent
        )
    }) {
        Some(_) => true,
        None => false,
    }
}

fn add_to_step_list(
    hold_detected: bool,
    mut step_list: Vec<(String, String)>, // flow, step
    search_step_info: &StepInfo,
    flow: String,
    step: String
) -> Vec<(String, String)> {
    if !hold_detected {
        if step_list.is_empty() {
            step_list.push((search_step_info.flow.to_owned(), search_step_info.step.to_owned()));
        }
        step_list.push((flow, step));
    }

    step_list
}

fn infinite_loop_check(
    linter_info: &LinterInfo,
    mut step_list: Vec<(String, String)>, // flow, step
    close_list: &mut Vec<(String, String)>, // flow, step
    previews_flow: String,
    previews_step: String,
) -> Option<(Vec<(String, String)>, Interval, String)> {

    let search_step_info = StepInfo {
        flow: previews_flow.clone(),
        step: previews_step,
        raw_flow: "",
        in_flow: "".to_owned(),
        step_breakers: vec![],
        interval: Interval::default()
    };

    match linter_info.step_list.get(&search_step_info) {
        Some(step_info) => {
            let mut hold_detected = false;

            for breaker in step_info.step_breakers.iter() {

                match breaker {
                    StepBreakers::HOLD(_) => {
                        hold_detected = true;
                        step_list.clear();
                    },
                    StepBreakers::GOTO {flow, step, interval } => {
                        let is_infinite_loop = is_in_list(&step_list, flow, step);
                        if is_infinite_loop {
                            step_list.push((flow.to_owned(), step.to_owned()));
                            return Some((step_list.to_owned(), interval.to_owned(), previews_flow))
                        }

                        if is_in_list(&close_list, flow, step) {
                            continue
                        }
                        close_list.push((flow.to_owned(), step.to_owned()));

                        let new_step_list = add_to_step_list(
                            hold_detected, 
                            step_list.clone(),
                            &search_step_info,
                            flow.to_owned(),
                            step.to_owned()
                        );

                        match infinite_loop_check(
                            linter_info,
                            new_step_list,
                            close_list,
                            flow.to_owned(),
                            step.to_owned()
                        ) {
                            Some(infinite_loop_vec) => return Some(infinite_loop_vec),
                            None => {}
                        }
                    }
                }
            }
        }
        None => {} // we don't need to log non existent steps here because validate_gotos already do the work
    }

    return None
}

