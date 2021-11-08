use crate::data::{
    ast::Interval,
    warnings::*,
};
use crate::error_format::{ErrorInfo};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum StepBreakers {
    HOLD(Interval),
    GOTO {
        step: String,
        flow: String,
        interval: Interval
    }
}

#[derive(Debug, Clone)]
pub struct StepInfo<'a> {
    pub flow: String,
    pub step: String,
    pub raw_flow: &'a str,
    pub in_flow: String,
    pub step_breakers: Vec<StepBreakers>,
    pub interval: Interval,
}

#[derive(Debug)]
pub struct FunctionCallInfo<'a> {
    pub name: String,
    pub in_flow: &'a str,
    pub scope_type: ScopeType,
    pub is_permanent: bool,
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

#[derive(Debug, Clone)]
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
    pub in_function: i16,
    pub loop_scope: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeType {
    Function(String),
    Step(String)
}

#[derive(Debug)]
pub struct LinterInfo<'a> {
    pub flow_name: &'a str,
    pub scope_type: ScopeType,
    pub raw_flow: &'a str,
    pub goto_list: &'a mut Vec<StepInfo<'a>>,
    pub step_list: &'a mut HashSet<StepInfo<'a>>,
    pub function_list: &'a mut HashSet<FunctionInfo<'a>>,
    pub import_list: &'a mut HashSet<ImportInfo<'a>>,
    pub valid_closure_list: &'a mut Vec<FunctionCallInfo<'a>>,
    pub functions_call_list: &'a mut Vec<FunctionCallInfo<'a>>,
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
    pub fn new(
        flow: &str,
        step: &str,
        raw_flow: &'a str,
        in_flow: String,
        step_breakers: Vec<StepBreakers>,
        interval: Interval
    ) -> Self {
        Self {
            flow: flow.to_owned(),
            step: step.to_owned(),
            step_breakers,
            raw_flow,
            in_flow,
            interval,
        }
    }
}

impl State {
    pub fn new(in_function: i16) -> Self {
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
        scope_type: ScopeType,
        raw_flow: &'a str,
        goto_list: &'a mut Vec<StepInfo<'a>>,
        step_list: &'a mut HashSet<StepInfo<'a>>,
        function_list: &'a mut HashSet<FunctionInfo<'a>>,
        import_list: &'a mut HashSet<ImportInfo<'a>>,
        valid_closure_list: &'a mut Vec<FunctionCallInfo<'a>>,
        functions_call_list: &'a mut Vec<FunctionCallInfo<'a>>,
        errors: &'a mut Vec<ErrorInfo>,
        warnings: &'a mut Vec<Warnings>,
        native_components: &'a Option<serde_json::Map<String, serde_json::Value>>,
    ) -> Self {
        Self {
            flow_name,
            scope_type,
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

impl<'a> FunctionCallInfo<'a> {
    pub fn new(name: String, in_flow: &'a str, scope_type: ScopeType, is_permanent: bool, raw_flow: &'a str, interval: Interval) -> Self {
        Self {
            name,
            in_flow,
            scope_type,
            is_permanent,
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
