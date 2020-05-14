use crate::data::ast::Flow;
use crate::data::error_info::ErrorInfo;
use crate::data::warnings::Warnings;

use std::collections::HashMap;


////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct CsmlResult {
    pub flows: Option<HashMap<String, Flow>>,
    pub warnings: Option<Vec<Warnings>>,
    pub errors: Option<Vec<ErrorInfo>>,
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

impl CsmlResult {
    pub fn new(flows: HashMap<String, Flow>, warnings: Vec<Warnings>, errors: Vec<ErrorInfo>) -> Self {
        let flows = match flows.is_empty() {
            false => Some(flows),
            true => None,
        };

        let warnings = match warnings.is_empty() {
            false => Some(warnings),
            true => None,
        };

        let errors = match errors.is_empty() {
            false => Some(errors),
            true => None,
        };

        Self {
            flows,
            warnings,
            errors,
        }
    }
}