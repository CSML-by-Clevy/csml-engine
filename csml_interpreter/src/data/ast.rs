use crate::data::tokens::*;
use crate::data::{ArgsType, Literal, Position};

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
    pub position: Position,
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
}

impl Hash for InstructionScope {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            InstructionScope::StepScope(name) => name.hash(state),
            InstructionScope::FunctionScope { name, .. } => name.hash(state),
            InstructionScope::ImportScope(import_scope) => import_scope.hash(state),
        }
    }
}

impl PartialEq for InstructionScope {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (InstructionScope::StepScope(name1), InstructionScope::StepScope(name2)) => {
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
            _ => false,
        }
    }
}

impl Eq for InstructionScope {}

impl Display for InstructionScope {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            InstructionScope::StepScope(ref idents) => write!(f, "{}", idents),
            InstructionScope::FunctionScope { name, .. } => write!(f, "{}", name),
            InstructionScope::ImportScope(ImportScope {
                name,
                original_name: _,
                from_flow,
                ..
            }) => write!(f, "import {} from {:?} ", name, from_flow),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub instruction_type: InstructionScope,
    pub actions: Expr,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum GotoType {
    Step(String),
    Flow(String),
    StepFlow { step: String, flow: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DoType {
    Update(Box<Expr>, Box<Expr>),
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
pub enum ObjectType {
    Goto(GotoType, Interval),
    Hold(Interval),
    Say(Box<Expr>),
    Debug(Box<Expr>, Interval),
    Return(Box<Expr>),
    Do(DoType),
    Use(Box<Expr>),

    Remember(Identifier, Box<Expr>),
    // Assign{old: Box<Expr>, new: Box<Expr>},
    Assign(Box<Expr>, Box<Expr>),

    As(Identifier, Box<Expr>),
    Import {
        step_name: Identifier,
        as_name: Option<Identifier>,
        file_path: Option<Identifier>,
    },
    Normal(Function),
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
    pub hooks: Vec<Hook>,
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
            hooks: Vec::new(),
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
        then_branch: Option<(Box<IfStatement>, InstructionInfo)>,
    },
    ElseStmt(Block, RangeInterval),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    Scope {
        block_type: BlockType,
        scope: Block,
        range: RangeInterval,
    },
    ForEachExpr(
        Identifier,
        Option<Identifier>,
        Box<Expr>,
        Block,
        RangeInterval,
    ),
    ComplexLiteral(Vec<Expr>, RangeInterval),
    MapExpr(HashMap<String, Expr>, RangeInterval),
    VecExpr(Vec<Expr>, RangeInterval),
    InfixExpr(Infix, Box<Expr>, Box<Expr>), // RangeInterval ?
    ObjectExpr(ObjectType),                 // RangeInterval ?
    IfExpr(IfStatement),

    PathExpr {
        literal: Box<Expr>,
        path: Vec<(Interval, PathState)>,
    },
    IdentExpr(Identifier),

    LitExpr(Literal),
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

    Not,
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

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct RangeInterval {
    pub start: Interval,
    pub end: Interval,
}

impl RangeInterval {
    pub fn new(start: Interval, end: Interval) -> Self {
        Self { start, end }
    }
}

#[derive(PartialEq, Debug, Clone, Eq, Hash, Copy, Serialize, Deserialize)]
pub struct Interval {
    pub line: u32,
    pub column: u32,
    pub offset: usize,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            line: 0,
            column: 0,
            offset: 0,
        }
    }
}

impl Interval {
    pub fn new_as_u32(line: u32, column: u32, offset: usize) -> Self {
        Self {
            line,
            column,
            offset,
        }
    }

    pub fn new_as_span(span: Span) -> Self {
        Self {
            line: span.location_line(),
            column: span.get_column() as u32,
            offset: span.location_offset(),
        }
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
