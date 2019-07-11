use serde::{
    ser::{SerializeMap, Serializer},
    Deserialize, Serialize,
};

use crate::parser::tokens::Span;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Flow {
    pub flow_instructions: HashMap<InstructionType, Expr>,
}

impl Serialize for Flow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_map(Some(self.flow_instructions.len()))?;
        for (k, v) in &self.flow_instructions {
            seq.serialize_entry(&k.to_string(), &v)?;
        }
        seq.end()
    }
}

#[derive(Serialize, Deserialize, Eq, Debug, Clone, Hash)]
pub enum InstructionType {
    StartFlow,
    NormalStep(SmartIdent),
}

impl PartialEq for InstructionType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (InstructionType::StartFlow, InstructionType::StartFlow) => true,
            (
                InstructionType::NormalStep(SmartIdent { ident: ident1, .. }),
                InstructionType::NormalStep(SmartIdent { ident: ident2, .. }),
            ) => ident1 == ident2,
            _ => false,
        }
    }
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            InstructionType::StartFlow => write!(f, "Start"),
            InstructionType::NormalStep(SmartIdent { ref ident, .. }) => write!(f, "{}", ident),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub actions: Expr,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum GotoType {
    Step,
    Flow,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ReservedFunction {
    Goto(GotoType, SmartIdent),
    Use(Box<Expr>),
    Say(Box<Expr>),
    Remember(SmartIdent, Box<Expr>),
    Assign(SmartIdent, Box<Expr>),
    As(SmartIdent, Box<Expr>),
    Import {
        step_name: SmartIdent,
        as_name: Option<SmartIdent>,
        file_path: Option<SmartIdent>,
    },
    Normal(SmartIdent, Box<Expr>),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum BlockType {
    Ask,
    Response,
    AskResponse(Option<SmartIdent>),
    Step,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum IfStatement {
    IfStmt {
        cond: Box<Expr>,
        consequence: Vec<Expr>,
        then_branch: Option<Box<IfStatement>>,
    },
    ElseStmt(Vec<Expr>),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Expr {
    Block {
        block_type: BlockType,
        arg: Vec<Expr>,
    },
    IfExpr(IfStatement),
    InfixExpr(Infix, Box<Expr>, Box<Expr>),
    FunctionExpr(ReservedFunction),
    ComplexLiteral(Vec<Expr>),
    VecExpr(Vec<Expr>),
    BuilderExpr(Box<Expr>, Box<Expr>),

    IdentExpr(SmartIdent),
    LitExpr(SmartLiteral),
}

impl Expr {
    pub fn new_literal(literal: Literal, span: Span) -> Expr {
        Expr::LitExpr(SmartLiteral {
            literal,
            interval: Interval {
                line: span.line,
                column: span.get_column() as u32,
            },
        })
    }

    pub fn new_ident(ident: String, span: Span) -> SmartIdent {
        SmartIdent {
            ident,
            interval: Interval {
                line: span.line,
                column: span.get_column() as u32,
            },
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Expr::ComplexLiteral(..) => "complex_literal".to_owned(),
            Expr::BuilderExpr(..) => "builder".to_owned(),
            Expr::VecExpr(..) => "Array".to_owned(),
            Expr::IdentExpr(SmartIdent { ident, .. }) => ident.to_owned(),
            Expr::LitExpr(SmartLiteral { literal, .. }) => literal.type_to_string(),
            Expr::FunctionExpr(..) => "function".to_owned(),
            Expr::Block { .. } => "block".to_owned(),
            Expr::IfExpr { .. } => "if".to_owned(),
            Expr::InfixExpr(..) => "infix".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub struct SmartIdent {
    pub ident: String,
    pub interval: Interval,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SmartLiteral {
    pub literal: Literal,
    pub interval: Interval,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Infix {
    Adition,
    Substraction,
    Divide,
    Multiply,

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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub struct Interval {
    pub line: u32,
    pub column: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Literal {
    #[serde(rename = "text")]
    StringLiteral(String),
    #[serde(rename = "int")]
    IntLiteral(i64),
    #[serde(rename = "float")]
    FloatLiteral(f64),
    #[serde(rename = "bool")]
    BoolLiteral(bool),
    #[serde(rename = "array")]
    ArrayLiteral(Vec<Literal>),
    #[serde(rename = "object")]
    ObjectLiteral {
        name: String,
        value: HashMap<String, Literal>,
    },
    #[serde(rename = "null")]
    Null,
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Literal) -> Option<Ordering> {
        match (self, other) {
            (Literal::StringLiteral(l1), Literal::StringLiteral(l2)) => l1.partial_cmp(l2),
            (Literal::IntLiteral(l1), Literal::IntLiteral(l2)) => l1.partial_cmp(l2),
            (Literal::FloatLiteral(l1), Literal::FloatLiteral(l2)) => l1.partial_cmp(l2),
            (Literal::BoolLiteral(l1), Literal::BoolLiteral(l2)) => l1.partial_cmp(l2),
            _ => None,
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Literal) -> bool {
        match (self, other) {
            (Literal::StringLiteral(l1), Literal::StringLiteral(l2)) => l1 == l2,
            (Literal::IntLiteral(l1), Literal::IntLiteral(l2)) => l1 == l2,
            (Literal::FloatLiteral(l1), Literal::FloatLiteral(l2)) => l1 == l2,
            (Literal::BoolLiteral(l1), Literal::BoolLiteral(l2)) => l1 == l2,
            _ => false,
        }
    }
}

impl Add for Literal {
    type Output = Result<Literal, String>;

    fn add(self, other: Literal) -> Result<Literal, String> {
        match (self, other) {
            (Literal::FloatLiteral(l1), Literal::IntLiteral(l2))        => Ok(Literal::FloatLiteral(l1 + l2 as f64)),
            (Literal::IntLiteral(l1), Literal::FloatLiteral(l2))        => Ok(Literal::FloatLiteral(l1 as f64 + l2)),
            (Literal::FloatLiteral(l1), Literal::FloatLiteral(l2))      => Ok(Literal::FloatLiteral(l1 + l2)),
            (Literal::IntLiteral(l1), Literal::IntLiteral(l2))          => Ok(Literal::IntLiteral(l1 + l2)),
            (Literal::BoolLiteral(l1), Literal::BoolLiteral(l2))        => Ok(Literal::IntLiteral(l1 as i64 + l2 as i64)),
            _                                                           => Err("Illegal operation + between types".to_owned())
            // (Literal::StringLiteral(l1), Literal::StringLiteral(l2))    => Literal::IntLiteral(l1 + l2),
        }
    }
}

impl Sub for Literal {
    type Output = Result<Literal, String>;

    fn sub(self, other: Literal) -> Result<Literal, String> {
        match (self, other) {
            (Literal::FloatLiteral(l1), Literal::IntLiteral(l2))        => Ok(Literal::FloatLiteral(l1 - l2 as f64)),
            (Literal::IntLiteral(l1), Literal::FloatLiteral(l2))        => Ok(Literal::FloatLiteral(l1 as f64 - l2)),
            (Literal::FloatLiteral(l1), Literal::FloatLiteral(l2))      => Ok(Literal::FloatLiteral(l1 - l2)),

            (Literal::IntLiteral(l1), Literal::IntLiteral(l2))          => Ok(Literal::IntLiteral(l1 - l2)),
            (Literal::BoolLiteral(l1), Literal::BoolLiteral(l2))        => Ok(Literal::IntLiteral(l1 as i64 - l2 as i64)),
            _                                                           => Err("Illegal operation - between types".to_owned())
            // (Literal::StringLiteral(l1), Literal::StringLiteral(l2))    => Literal::IntLiteral(l1 - l2),
        }
    }
}

impl Div for Literal {
    type Output = Result<Literal, String>;

    fn div(self, other: Literal) -> Result<Literal, String> {
        match (self, other) {
            (Literal::FloatLiteral(l1), Literal::IntLiteral(l2))        => {
                if l2 == 0 { return Err("Cannot divide by zero-valued".to_owned()) }
                Ok(Literal::FloatLiteral(l1 / l2 as f64))
            },
            (Literal::IntLiteral(l1), Literal::FloatLiteral(l2))        => {
                if l2 == 0.0 { return Err("Cannot divide by zero-valued".to_owned()) }
                Ok(Literal::FloatLiteral(l1 as f64 / l2))
            },
            (Literal::FloatLiteral(l1), Literal::FloatLiteral(l2))      => {
                if l2 == 0.0 { return Err("Cannot divide by zero-valued".to_owned()) }
                Ok(Literal::FloatLiteral(l1 / l2))
            },
            (Literal::IntLiteral(l1), Literal::IntLiteral(l2))          => {
                if l2 == 0 { return Err("Cannot divide by zero-valued".to_owned()) }
                Ok(Literal::IntLiteral(l1 / l2))
            },
            (Literal::BoolLiteral(l1), Literal::BoolLiteral(l2))        => {
                if !l2 { return Err("Cannot divide by zero-valued".to_owned()) }
                Ok(Literal::IntLiteral(l1 as i64 / l2 as i64))
            },
            _                                                           => Err("Illegal operation / between types".to_owned())
            // (Literal::StringLiteral(l1), Literal::StringLiteral(l2))    => Literal::IntLiteral(l1 / l2),
        }
    }
}

impl Mul for Literal {
    type Output = Result<Literal, String>;

    fn mul(self, other: Literal) -> Result<Literal, String> {
        match (self, other) {
            (Literal::FloatLiteral(l1), Literal::IntLiteral(l2))        => Ok(Literal::FloatLiteral(l1 * l2 as f64)),
            (Literal::IntLiteral(l1), Literal::FloatLiteral(l2))        => Ok(Literal::FloatLiteral(l1 as f64 * l2)),
            (Literal::FloatLiteral(l1), Literal::FloatLiteral(l2))      => Ok(Literal::FloatLiteral(l1 * l2)),

            (Literal::IntLiteral(l1), Literal::IntLiteral(l2))          => Ok(Literal::IntLiteral(l1 * l2)),
            (Literal::BoolLiteral(l1), Literal::BoolLiteral(l2))        => Ok(Literal::IntLiteral(l1 as i64 * l2 as i64)),
            _                                                           => Err("Illegal operation * between types".to_owned())
            // (Literal::StringLiteral(l1), Literal::StringLiteral(l2))    => Literal::IntLiteral(l1 * l2),
        }
    }
}

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Literal::StringLiteral(literal) => literal.to_owned(),
            Literal::IntLiteral(literal) => literal.to_string(),
            Literal::FloatLiteral(literal) => literal.to_string(),
            Literal::BoolLiteral(literal) => literal.to_string(),
            Literal::ArrayLiteral(vec) => format!("{:?}", vec),
            Literal::ObjectLiteral { name, value } => format!("{}: {:?}", name, value),
            Literal::Null => "NULL".to_owned(),
        }
    }

    pub fn type_to_string(&self) -> String {
        match self {
            Literal::StringLiteral(..) => "Text".to_owned(),
            Literal::IntLiteral(..) => "Numeric".to_owned(),
            Literal::FloatLiteral(..) => "Numeric".to_owned(),
            Literal::BoolLiteral(..) => "Bool".to_owned(),
            Literal::ArrayLiteral(..) => "Array".to_owned(),
            Literal::ObjectLiteral { .. } => "Object".to_owned(),
            Literal::Null => "NULL".to_owned(),
        }
    }
}
