use crate::data::{
    ast::Interval,
    warnings::*,
};
use crate::error_format::{ErrorInfo};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct StepInfo<'a> {
    pub flow: String,
    pub step: String,
    pub raw_flow: &'a str,
    pub interval: Interval,
}

#[derive(Debug)]
pub struct FunctionInfo<'a> {
    pub name: String,
    pub in_flow: &'a str,
    pub raw_flow: &'a str,
    pub interval: Interval,
}

#[derive(Debug)]
pub struct ImportInfo<'a> {
    pub as_name: String,
    pub original_name: Option<String>,
    pub from_flow: Option<String>,
    pub in_flow: &'a str,
    pub raw_flow: &'a str,
    pub interval: Interval,
}

#[derive(Debug)]
pub struct State {
    pub in_function: bool,
    pub loop_scope: usize,
}

#[derive(Debug)]
pub struct LinterInfo<'a> {
    pub flow_name: &'a str,
    pub raw_flow: &'a str,
    pub goto_list: &'a mut Vec<StepInfo<'a>>,
    pub step_list: &'a mut HashSet<StepInfo<'a>>,
    pub function_list: &'a mut HashSet<FunctionInfo<'a>>,
    pub import_list: &'a mut HashSet<ImportInfo<'a>>,
    pub valid_closure_list: &'a mut Vec<String>,
    pub functions_call_list: &'a mut Vec<(String, Interval)>,
    pub errors: &'a mut Vec<ErrorInfo>,
    pub warnings: &'a mut Vec<Warnings>,
    pub native_components: &'a Option<serde_json::Map<String, serde_json::Value>>,
}

////////////////////////////////////////////////////////////////////////////////
// Hash FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl<'a> Hash for StepInfo<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.step.hash(state);
        self.flow.hash(state)
    }
}

impl<'a> PartialEq for StepInfo<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.step == other.step && self.flow == other.flow
    }
}

impl<'a> Eq for StepInfo<'a> {}

impl<'a> Hash for FunctionInfo<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.in_flow.hash(state)
    }
}

impl<'a> PartialEq for FunctionInfo<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.in_flow == other.in_flow
    }
}

impl<'a> Eq for FunctionInfo<'a> {}

impl<'a> Hash for ImportInfo<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_name.hash(state);
        self.in_flow.hash(state)
    }
}

impl<'a> PartialEq for ImportInfo<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.as_name == other.as_name && self.in_flow == other.in_flow
    }
}

impl<'a> Eq for ImportInfo<'a> {}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl<'a> StepInfo<'a> {
    pub fn new(flow: &str, step: &str, raw_flow: &'a str, interval: Interval) -> Self {
        Self {
            flow: flow.to_owned(),
            step: step.to_owned(),
            raw_flow,
            interval,
        }
    }
}

impl State {
    pub fn new(in_function: bool) -> Self {
        Self {
            in_function,
            loop_scope: 0,
        }
    }

    pub fn enter_loop(&mut self) {
        self.loop_scope = self.loop_scope + 1
    }

    pub fn exit_loop(&mut self) {
        self.loop_scope = self.loop_scope - 1
    }
}

impl<'a> LinterInfo<'a> {
    pub fn new(
        flow_name: &'a str,
        raw_flow: &'a str,
        goto_list: &'a mut Vec<StepInfo<'a>>,
        step_list: &'a mut HashSet<StepInfo<'a>>,
        function_list: &'a mut HashSet<FunctionInfo<'a>>,
        import_list: &'a mut HashSet<ImportInfo<'a>>,
        valid_closure_list: &'a mut Vec<String>,
        functions_call_list: &'a mut Vec<(String, Interval)>,
        errors: &'a mut Vec<ErrorInfo>,
        warnings: &'a mut Vec<Warnings>,
        native_components: &'a Option<serde_json::Map<String, serde_json::Value>>,
    ) -> Self {
        Self {
            flow_name,
            raw_flow,
            goto_list,
            step_list,
            function_list,
            import_list,
            valid_closure_list,
            functions_call_list,
            errors,
            warnings,
            native_components
        }
    }
}

impl<'a> FunctionInfo<'a> {
    pub fn new(name: String, in_flow: &'a str, raw_flow: &'a str, interval: Interval) -> Self {
        Self {
            name,
            in_flow,
            raw_flow,
            interval,
        }
    }
}

impl<'a> ImportInfo<'a> {
    pub fn new(
        as_name: String,
        original_name: Option<String>,
        from_flow: Option<String>,
        in_flow: &'a str,
        raw_flow: &'a str,
        interval: Interval,
    ) -> Self {
        Self {
            as_name,
            original_name,
            from_flow,
            in_flow,
            raw_flow,
            interval,
        }
    }
}
