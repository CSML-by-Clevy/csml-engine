use crate::parser::{literal::Literal, tokens::*};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug, Clone)]
pub struct Flow {
    pub flow_instructions: HashMap<InstructionType, Expr>,
    pub flow_type: FlowType
}

#[derive(PartialEq, Debug, Clone)]
pub enum FlowType {
    Normal,
    Recursive
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum InstructionType {
    NormalStep(String),
    //hook ?
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            InstructionType::NormalStep(ref ident) => write!(f, "{}", ident),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub actions: Expr,
}

#[derive(PartialEq, Debug, Clone)]
pub enum GotoType {
    SubStep,
    Step,
    Flow,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ObjectType {
    Goto(GotoType, Identifier),
    WaitFor(Identifier),
    Use(Box<Expr>),
    Say(Box<Expr>),
    Remember(Identifier, Box<Expr>),
    Assign(Identifier, Box<Expr>),
    As(Identifier, Box<Expr>),
    Import {
        step_name: Identifier,
        as_name: Option<Identifier>,
        file_path: Option<Identifier>,
    },
    Normal(Identifier, Box<Expr>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum BlockType {
    Ask,
    Response,
    AskResponse(Option<Identifier>),
    SubStep(Identifier),
    Step,
}

#[derive(PartialEq, Debug, Clone)]
pub enum IfStatement {
    IfStmt {
        cond: Box<Expr>,
        consequence: Vec<Expr>,
        then_branch: Option<Box<IfStatement>>,
    },
    ElseStmt(Vec<Expr>, RangeInterval),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Block {
        block_type: BlockType,
        arg: Vec<Expr>, // HM<line?, Expr> change to hashmap in order to make step by step interactions with the manager
        range: RangeInterval,
    },
    ComplexLiteral(Vec<Expr>, RangeInterval),
    VecExpr(Vec<Expr>, RangeInterval),
    InfixExpr(Infix, Box<Expr>, Box<Expr>), // RangeInterval
    ForExpr(
        Identifier,
        Option<Identifier>,
        Box<Expr>,
        Vec<Expr>,
        RangeInterval,
    ),

    ObjectExpr(ObjectType), // RangeInterval ?
    IfExpr(IfStatement),
    BuilderExpr(Box<Expr>, Box<Expr>),

    IdentExpr(Identifier),
    LitExpr(Literal),
}

impl Expr {
    pub fn new_ident(ident: String, interval: Interval, index: Option<Box<Self>>) -> Identifier {
        Identifier {
            ident,
            interval,
            index,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Infix {
    Adition,
    Substraction,
    Divide,
    Multiply,
    Remainder,

    Not,
    Match,

    Equal,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual,
    GreaterThan,
    LessThan,

    And,
    Or,
}

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub struct RangeInterval {
    pub start: Interval,
    pub end: Interval,
}

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub struct Interval {
    pub line: u32,
    pub column: u32,
}

impl Interval {
    pub fn new(span: Span) -> Self {
        Self {
            line: span.line,
            column: span.get_column() as u32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub ident: String,
    pub interval: Interval,
    pub index: Option<Box<Expr>>,
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
