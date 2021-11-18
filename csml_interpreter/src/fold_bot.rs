
use crate::data::{warnings::*,};
use crate::error_format::{ErrorInfo};

use crate::linter::{
    FlowToValidate, FunctionInfo, ImportInfo, LinterInfo, 
    StepInfo, StepBreakers, FunctionCallInfo, ScopeType,
    linter::{validate_flow_ast, validate_gotos, validate_functions, validate_imports}
};

use std::collections::{HashSet, HashMap};


////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn fold_bot(
    flows: &[FlowToValidate],
    errors: &mut Vec<ErrorInfo>,
    warnings: &mut Vec<Warnings>,
    native_components: &Option<serde_json::Map<String, serde_json::Value>>,
    default_flow: &str,
) -> String {
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

    let flow_list = make_flow_list(&linter_info.step_list);
    make_fold(default_flow, flow_list, &linter_info)
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn make_flow_list<'a>(step_list: &HashSet<StepInfo<'a>> ) -> HashMap<String, Vec<StepInfo<'a>>> {

    step_list.iter().fold(HashMap::new(), |mut acc: HashMap<String, Vec<StepInfo<'a>> >, goto_info| {

        if let Some(flow_gotos) = acc.get_mut(&goto_info.in_flow) {

            // goto_info
            flow_gotos.push(goto_info.to_owned());

        } else {
            acc.insert(goto_info.in_flow.clone(), vec![goto_info.to_owned()]);
        }

        acc
    })
}

fn update_step_names<'a>(
    default_flow: &str,
    flow: &mut Vec<String>,
    current_flow_name: &str,
    step_list: &HashSet<StepInfo<'a>>
) {
    for step in step_list.iter() {
        if step.flow != current_flow_name || 
            (step.step == "start" && default_flow == current_flow_name)
        {
            continue
        }

        let line = (step.interval.start_line - 1) as usize;
        let column = (step.interval.start_column - 1) as usize;

        let (first, second) = flow[line].split_at(column);

        let mut split_line: Vec<String> = second.split(':').map(|s| s.to_string()).collect();
        split_line[0] = format!("{}{}_{}:", first, step.in_flow, step.step);

        flow[line] = split_line.concat();
    }
}

fn make_update_fn_name_list<'a>(
    flow: &mut Vec<String>,
    flow_name: &str,
    flow_imports: &Vec<&ImportInfo<'a>>,
    function_list: &HashSet<FunctionInfo<'a>>,
    functions_call_list: &Vec<FunctionCallInfo<'a>>
) {
    for function in function_list.iter() {
        if function.in_flow == flow_name {
            update_fn_name(flow, function)
        }
    }

    for function_call in functions_call_list.iter() {
        if function_call.in_flow == flow_name {

            let import = flow_imports.iter().find(|&&import| {
                &import.as_name == &function_call.name
            });

            if let Some(&import) = import {
                let fn_name = match &import.original_name {
                    Some(name) => name,
                    None => &import.as_name,
                };

                for function in function_list.iter() {
                    match &import.from_flow {
                        Some(from) if function.in_flow == from => {
                            update_fn_call_name(flow, function, function_call);
                        }
                        None if &function.name == fn_name => {
                            update_fn_call_name(flow, function, function_call);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn update_fn_call_name(
    flow: &mut Vec<String>,
    function: &FunctionInfo,
    function_call: &FunctionCallInfo
) {
    let line = (function_call.interval.start_line - 1) as usize;
    let column = (function_call.interval.start_column - 1) as usize;

    let (first, second) = flow[line].split_at(column);

    let mut split_line: Vec<String> = second.split('(').map(|s| s.to_string()).collect();

    split_line[0] = format!("{} {}_{}(", first, function.in_flow, function.name);

    flow[line] = split_line.concat();
}

fn update_fn_name(
    flow: &mut Vec<String>,
    function: &FunctionInfo,
) {
    let line = (function.interval.start_line - 1) as usize;

    let mut split_line: Vec<String> = flow[line].split('(').map(|s| s.to_string()).collect();

    split_line[0] = format!("fn {}_{}(", function.in_flow, function.name);

    flow[line] = split_line.concat();
}

fn update_goto_names(
    flow: &mut Vec<String>,
    default_flow: &str,
    flow_gotos: &[StepInfo]
) {
    for step_info in flow_gotos.iter() {

        for goto_info in step_info.step_breakers.iter() {
            if let StepBreakers::GOTO {
                step: step_name,
                flow: flow_name,
                interval
            } = goto_info {

                let line = (interval.start_line - 1) as usize;
                let column = interval.start_column as usize;

                let (first, second) = flow[line].split_at(column);
                let mut split_line: Vec<String> = second.split(' ').map(|s| s.to_string()).collect();

                let new_goto_name = match step_name {
                    step if step == "end" => format!("end"),
                    step if step == "start" && flow_name == default_flow => format!("start"),
                    step => format!("{}_{} ", flow_name, step)
                };

                split_line[0] = format!("{}{}", first, new_goto_name,);

                flow[line] = split_line.concat();
            }
        }
    }
}

fn make_fold<'a>(
    default_flow: &str,
    flow_list: HashMap<String, Vec<StepInfo<'a>> >,
    linter_info: &LinterInfo,
) -> String {
    let mut main_flow: Vec<String> = Vec::new();

    for (flow_name, flow_steps) in flow_list.iter() {

        let flow = match flow_steps.get(0) {
            Some(step_info) => step_info.raw_flow,
            None => continue
        };

        let mut split_flow: Vec<String> = flow.split('\n').map(|s| s.to_string()).collect();

        update_goto_names(
            &mut split_flow,
            default_flow,
            flow_steps
        );

        let flow_imports = linter_info.import_list
        .iter()
        .fold(Vec::new(), |mut acc, import| {
            if import.in_flow == flow_name {
                acc.push(import)
            }
            acc
        });

        make_update_fn_name_list(
            &mut split_flow,
            flow_name,
            &flow_imports,
            &linter_info.function_list,
            &linter_info.functions_call_list
        );

        update_step_names(
            default_flow,
            &mut split_flow,
            flow_name,
            &linter_info.step_list
        );

        remove_imports(
            &mut split_flow,
            &flow_imports,
        );

        main_flow.append(&mut split_flow);
    }

    main_flow.join("\n")
}

fn remove_imports<'a>(
    flow: &mut Vec<String>,
    flow_imports: &Vec<&ImportInfo<'a>>,
) {
    let mut index_corrector = 1;
    for import in flow_imports.iter() {
        let mut line = import.interval.start_line as i32 - index_corrector as i32;

        if line < 0 {
            line = 0;
        }

        flow.remove(line as usize);

        index_corrector += 1;
    }
}
