use crate::data::ast::Flow;
use crate::data::csml_bot::CsmlBot;
use crate::data::error_info::ErrorInfo;

use lazy_static::*;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

type CsmlRules = fn(flow_name: &str, flow: &Flow, bot: &CsmlBot, error: &mut Vec<ErrorInfo>);

lazy_static! {
    static ref FUNCTIONS: Vec<CsmlRules> = {
        let mut vector: Vec<CsmlRules> = Vec::new();

        vector.push(check_missing_flow);
        vector.push(check_valid_flow);
        vector.push(check_duplicate_step);
        vector.push(check_valid_goto_step);
        vector.push(check_valid_goto_flow);

        vector
    };
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn check_missing_flow(flow_name: &str, flow: &Flow, bot: &CsmlBot, error: &mut Vec<ErrorInfo>) {
    unimplemented!();
}

fn check_valid_flow(flow_name: &str, flow: &Flow, bot: &CsmlBot, error: &mut Vec<ErrorInfo>) {
    unimplemented!();
}

fn check_duplicate_step(flow_name: &str, flow: &Flow, bot: &CsmlBot, error: &mut Vec<ErrorInfo>) {
    unimplemented!();
}

fn check_valid_goto_step(flow_name: &str, flow: &Flow, bot: &CsmlBot, error: &mut Vec<ErrorInfo>) {
    unimplemented!();
}

fn check_valid_goto_flow(flow_name: &str, flow: &Flow, bot: &CsmlBot, error: &mut Vec<ErrorInfo>) {
    unimplemented!();
}

fn check_valid_builtin(flow_name: &str, flow: &Flow, bot: &CsmlBot, error: &mut Vec<ErrorInfo>) {
    unimplemented!();
}

fn check_valid_method(flow_name: &str, flow: &Flow, bot: &CsmlBot, error: &mut Vec<ErrorInfo>) {
    unimplemented!();
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn linter(flow_name: &str, flow: &Flow, bot: &CsmlBot) -> Vec<ErrorInfo> {
    vec![]
}