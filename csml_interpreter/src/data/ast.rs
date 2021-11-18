use crate::data::tokens::*;
use crate::data::{ArgsType, Literal};

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flow {
    pub flow_instructions: HashMap<InstructionScope, Expr>,
    pub flow_type: FlowType,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum FlowType {
    Normal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportScope {
    pub name: String,
    pub original_name: Option<String>,
    pub from_flow: Option<String>,
    pub interval: Interval,
}

impl Hash for ImportScope {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for ImportScope {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for ImportScope {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstructionScope {
    StepScope(String),
    FunctionScope { name: String, args: Vec<String> },
    ImportScope(ImportScope),

    // this Variant is use to store all duplicated instruction during parsing
    // and use by the linter to display them all as errors
    DuplicateInstruction(Interval, String),
}

impl Hash for InstructionScope {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            InstructionScope::StepScope(name) => name.hash(state),
            InstructionScope::FunctionScope { name, .. } => name.hash(state),
            InstructionScope::ImportScope(import_scope) => import_scope.hash(state),
            InstructionScope::DuplicateInstruction(interval, ..) => interval.hash(state),
        }
    }
}

impl PartialEq for InstructionScope {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (InstructionScope::StepScope(name1, ..), InstructionScope::StepScope(name2, ..)) => {
                name1 == name2
            }
            (
                InstructionScope::FunctionScope { name: name1, .. },
                InstructionScope::FunctionScope { name: name2, .. },
            ) => name1 == name2,
            (
                InstructionScope::ImportScope(import_scope1),
                InstructionScope::ImportScope(import_scope2),
            ) => import_scope1 == import_scope2,
            (
                InstructionScope::DuplicateInstruction(interval1, ..),
                InstructionScope::DuplicateInstruction(interval2, ..),
            ) => interval1 == interval2,
            _ => false,
        }
    }
}

impl Eq for InstructionScope {}

impl Display for InstructionScope {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            InstructionScope::StepScope(ref idents, ..) => write!(f, "{}", idents),
            InstructionScope::FunctionScope { name, .. } => write!(f, "{}", name),
            InstructionScope::ImportScope(ImportScope {
                name,
                original_name: _,
                from_flow,
                ..
            }) => write!(f, "import {} from {:?} ", name, from_flow),
            InstructionScope::DuplicateInstruction(index, ..) => {
                write!(f, "duplicate instruction at line {}", index.start_line)
            }
        }
    }
}

impl InstructionScope {
    pub fn get_info(&self) -> String {
        match self {
            InstructionScope::StepScope(name, ..) => format!("step {}", name),
            InstructionScope::FunctionScope { name, .. } => format!("function {}", name),
            InstructionScope::ImportScope(ImportScope { name, .. }) => format!("import {}", name),
            InstructionScope::DuplicateInstruction(_, info) => format!("duplicate {}", info),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub instruction_type: InstructionScope,
    pub actions: Expr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GotoValueType {
    Name(Identifier),
    Variable(Box<Expr>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GotoType {
    Step(GotoValueType),
    Flow(GotoValueType),
    StepFlow {
        step: Option<GotoValueType>,
        flow: Option<GotoValueType>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DoType {
    Update(AssignType, Box<Expr>, Box<Expr>),
    Exec(Box<Expr>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub interval: Interval,
    // TODO: update to Vec<Expr>
    pub args: Box<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForgetMemory {
    ALL,
    SINGLE(Identifier),
    LIST(Vec<Identifier>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PreviousType {
    Step(Interval),
    Flow(Interval),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssignType {
    Assignment,
    AdditionAssignment,
    SubtractionAssignment,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectType {
    Goto(GotoType, Interval),
    Previous(PreviousType, Interval),
    Hold(Interval),
    Say(Box<Expr>),
    Debug(Box<Expr>, Interval),
    Return(Box<Expr>),
    Do(DoType),
    Use(Box<Expr>),

    Remember(Identifier, Box<Expr>),
    Assign(AssignType, Box<Expr>, Box<Expr>),
    Forget(ForgetMemory, Interval),

    As(Identifier, Box<Expr>),

    BuiltIn(Function),
    Break(Interval),
    Continue(Interval),
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct InstructionInfo {
    pub index: usize,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub commands: Vec<(Expr, InstructionInfo)>,
    pub commands_count: usize,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Hook {
    pub index: i64,
    pub name: String,
    pub step: String,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            commands: Vec::new(),
            commands_count: 0,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum BlockType {
    LoopBlock,
    Block,
    IfLoop,
    Step,
    Function,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IfStatement {
    IfStmt {
        cond: Box<Expr>,
        consequence: Block,
        then_branch: Option<Box<IfStatement>>,
        last_action_index: usize,
    },
    ElseStmt(Block, Interval),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    Scope {
        block_type: BlockType,
        scope: Block,
        range: Interval,
    },
    ForEachExpr(Identifier, Option<Identifier>, Box<Expr>, Block, Interval),
    WhileExpr(Box<Expr>, Block, Interval),
    ComplexLiteral(Vec<Expr>, Interval),
    MapExpr {
        object: HashMap<String, Expr>,
        is_in_sub_string: bool, // this value is use to determine if this object was declare inside a string or not
        interval: Interval,
    },
    VecExpr(Vec<Expr>, Interval),
    InfixExpr(Infix, Box<Expr>, Box<Expr>),
    PostfixExpr(Vec<Postfix>, Box<Expr>),
    ObjectExpr(ObjectType),
    IfExpr(IfStatement),

    PathExpr {
        literal: Box<Expr>,
        path: Vec<(Interval, PathState)>,
    },
    IdentExpr(Identifier),

    LitExpr {
        literal: Literal,
        in_in_substring: bool, // this value is use to determine if this literal was declare inside a string or not
    },
}

impl Expr {
    pub fn new_idents(ident: String, interval: Interval) -> Identifier {
        Identifier { ident, interval }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Infix {
    Addition,
    Subtraction,
    Divide,
    Multiply,
    Remainder,

    Match,
    NotMatch,

    Equal,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual,
    GreaterThan,
    LessThan,

    And,
    Or,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Postfix {
    Not,
}

#[derive(PartialEq, Debug, Clone, Eq, Hash, Copy, Serialize, Deserialize)]
pub struct Interval {
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: Option<u32>,
    pub end_column: Option<u32>,
    pub offset: usize,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            start_line: 0,
            start_column: 0,
            end_line: None,
            end_column: None,
            offset: 0,
        }
    }
}

impl Interval {
    pub fn new_as_u32(
        start_line: u32,
        start_column: u32,
        offset: usize,
        end_line: Option<u32>,
        end_column: Option<u32>,
    ) -> Self {
        Self {
            start_line,
            start_column,
            end_line,
            end_column,
            offset,
        }
    }

    pub fn new_as_span(span: Span) -> Self {
        Self {
            start_line: span.location_line(),
            start_column: span.get_column() as u32,
            end_line: None,
            end_column: None,
            offset: span.location_offset(),
        }
    }

    pub fn add_end(&mut self, end: Self) {
        self.end_line = Some(end.start_line);
        self.end_column = Some(end.start_column)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathState {
    ExprIndex(Expr),
    StringIndex(String),
    Func(Function),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathLiteral {
    VecIndex(usize),
    MapIndex(String),
    Func {
        name: String,
        interval: Interval,
        args: ArgsType,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identifier {
    pub ident: String,
    pub interval: Interval,
}

impl Identifier {
    pub fn new(ident: &str, interval: Interval) -> Self {
        Self {
            ident: ident.to_string(),
            interval,
        }
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.ident.partial_cmp(&other.ident)
    }
}
