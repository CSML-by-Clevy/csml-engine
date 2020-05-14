use crate::data::ast::Interval;
use crate::linter::data::Linter;
use crate::error_format::ErrorInfo;
use crate::data::position::Position;
use crate::error_format::gen_error_info;

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
        error.push(gen_error_info(
            Position::new(Interval::new_as_u32(0, 0)),
            "LINTER: Need to have at least one Flow".to_owned(),
        ));
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
                error.push(gen_error_info(
                    Position::new(Interval::new_as_u32(0, 0)),
                    format!("LINTER: Flow '{}' need to have a 'start' step", flow),
                ));
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
                        error.push(gen_error_info(
                            Position::new(*vector_step.last().unwrap()),
                            format!("LINTER: Duplicate step '{}' in flow '{}'", step, flow),
                        ));
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
                error.push(gen_error_info(
                    Position::new(goto.interval),
                    format!("LINTER: Step '{}' doesn't exist", goto.step),
                ));
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn lint_flow(mut error: &mut Vec<ErrorInfo>) {
    let linter = Linter::get_linter();

    for f in FUNCTIONS.iter() {
        f(&linter, &mut error);
    }
}
