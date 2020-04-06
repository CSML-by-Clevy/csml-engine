use crate::data::ast::Interval;
use crate::data::error_info::ErrorInfo;
use crate::linter::Linter;
use lazy_static::*;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

type CsmlRules = fn(linter: &Linter, error: &mut Vec<ErrorInfo>);

lazy_static! {
    static ref FUNCTIONS: Vec<CsmlRules> = {
        let mut vector: Vec<CsmlRules> = Vec::new();

        vector.push(check_missing_flow);
        vector.push(check_valid_flow);
        vector.push(check_duplicate_step);
        vector.push(check_valid_goto_step);

        vector
    };
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn check_missing_flow(linter: &Linter, error: &mut Vec<ErrorInfo>) {
    if linter.flow.is_empty() {
        error.push(ErrorInfo {
            interval: Interval { line: 0, column: 0 },
            message: "ERROR: Need to have at least one Flow".to_owned(),
        });
    }
}

fn check_valid_flow(linter: &Linter, error: &mut Vec<ErrorInfo>) {
    for flow in linter.flow.keys() {
        let mut result = false;

        if let Some(hashmap) = linter.flow.get(flow) {
            for step in hashmap.keys() {
                if step == "start" {
                    result = true;
                }
            }

            if !result {
                error.push(ErrorInfo {
                    interval: Interval { line: 0, column: 0 },
                    message: format!("ERROR: Flow '{}' need to have a 'start' step", flow),
                });
            }
        }
    }
}

fn check_duplicate_step(linter: &Linter, error: &mut Vec<ErrorInfo>) {
    for flow in linter.flow.keys() {
        if let Some(hashmap_step) = linter.flow.get(flow) {
            for step in hashmap_step.keys() {
                if let Some(vector_step) = hashmap_step.get(step) {
                    if vector_step.len() > 1 {
                        error.push(ErrorInfo {
                            interval: *vector_step.last().unwrap(),
                            message: format!("ERROR: Duplicate step '{}' in flow '{}'", step, flow),
                        });
                    }
                }
            }
        }
    }
}

fn check_valid_goto_step(linter: &Linter, error: &mut Vec<ErrorInfo>) {
    for goto in linter.goto.iter() {
        if let Some(step) = linter.flow.get(&goto.flow) {
            if !step.contains_key(&goto.step) && goto.step != "end" {
                error.push(ErrorInfo {
                    interval: goto.interval,
                    message: format!("ERROR: Step '{}' doesn't exist", goto.step),
                });
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn linter(mut error: &mut Vec<ErrorInfo>) {
    let linter = Linter::get();

    for f in FUNCTIONS.iter() {
        f(&linter, &mut error);
    }
}
