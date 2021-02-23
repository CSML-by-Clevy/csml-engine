use crate::data::ast::Interval;
use crate::data::csml_bot::CsmlBot;
use crate::data::position::Position;
use crate::error_format::gen_error_info;
use crate::error_format::ErrorInfo;
use crate::linter::data::Linter;

use lazy_static::*;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

type CsmlRules = fn(bot: &CsmlBot, linter: &Linter, error: &mut Vec<ErrorInfo>);

lazy_static! {
    static ref FUNCTIONS: Vec<CsmlRules> = {
        let mut vector: Vec<CsmlRules> = Vec::new();

        vector.push(check_missing_flow);
        vector.push(check_valid_flow);
        vector.push(check_duplicate_step);
        vector.push(check_valid_goto);

        vector
    };
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn check_missing_flow(_bot: &CsmlBot, linter: &Linter, error: &mut Vec<ErrorInfo>) {
    if linter.flow.is_empty() {
        error.push(gen_error_info(
            Position::new(Interval::new_as_u32(0, 0, 0, None, None)),
            "LINTER: Need to have at least one Flow".to_owned(),
        ));
    }
}

fn check_valid_flow(_bot: &CsmlBot, linter: &Linter, error: &mut Vec<ErrorInfo>) {
    for flow in linter.flow.keys() {
        Position::set_flow(&flow);
        let mut result = false;

        if let Some(hashmap) = linter.flow.get(flow) {
            for step in hashmap.keys() {
                if step == "start" {
                    result = true;
                }
            }
            Position::set_step("");

            if !result {
                error.push(gen_error_info(
                    Position::new(Interval::new_as_u32(0, 0, 0, None, None)),
                    format!("LINTER: Flow '{}' need to have a 'start' step", flow),
                ));
            }
        }
    }
}

fn check_duplicate_step(_bot: &CsmlBot, linter: &Linter, error: &mut Vec<ErrorInfo>) {
    for flow in linter.flow.keys() {
        Position::set_flow(&flow);
        if let Some(hashmap_step) = linter.flow.get(flow) {
            for step in hashmap_step.keys() {
                Position::set_step(&step);
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

fn check_valid_goto(_bot: &CsmlBot, linter: &Linter, error: &mut Vec<ErrorInfo>) {
    for goto in linter.goto.iter() {
        Position::set_flow(&goto.src_flow);
        Position::set_step(&goto.src_step);

        match linter.flow.get(&goto.dst_flow) {
            Some(step) => {
                if !step.contains_key(&goto.dst_step) && goto.dst_step != "end" {
                    error.push(gen_error_info(
                        Position::new(goto.interval),
                        format!("LINTER: Step '{}' doesn't exist", goto.dst_step),
                    ));
                }
            }
            None => {
                error.push(gen_error_info(
                    Position::new(goto.interval),
                    format!("LINTER: Flow '{}' doesn't exist", goto.dst_flow),
                ));
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn lint_flow(bot: &CsmlBot, mut error: &mut Vec<ErrorInfo>) {
    let linter = Linter::get_linter();

    for f in FUNCTIONS.iter() {
        f(bot, &linter, &mut error);
    }
}
