use serde::{Deserialize, Serialize, ser::{Serializer, SerializeMap}};
use std::ops::{Sub, Add, Mul, Div};
use std::fmt::{Display, Formatter};
use std::collections::HashMap;
use std::cmp::Ordering;
// use crate::parser::tokens::Span;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Flow {
    pub flow_instructions: HashMap<InstructionType, Expr>
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum InstructionType {
    StartFlow(String),
    NormalStep(String)
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            InstructionType::StartFlow(ref name)     => write!(f, "{}", name),
            InstructionType::NormalStep(ref name)    => write!(f, "{}", name)
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub actions: Expr
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum GotoType {
    Step,
    File
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ReservedFunction {
    Goto(GotoType, String),
    Use(Box<Expr>),
    Say(Box<Expr>),
    Remember(String, Box<Expr>),
    Assign(String, Box<Expr>),
    As(String, Box<Expr>),
    Import{step_name: String, as_name: Option<String>, file_path: Option<String>},
    Normal(String, Box<Expr>)
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum BlockType {
    Ask,
    Response,
    AskResponse,
    Step
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Expr {
    Block {
        block_type: BlockType,
        arg: Vec<Expr>,
    },
    IfExpr {
        cond: Box<Expr>,
        consequence: Vec<Expr>,
        // #[serde(skip_serializing, skip_deserializing)]
        // span: Option<Span >
    },
    InfixExpr(Infix, Box<Expr>, Box<Expr>),
    FunctionExpr(ReservedFunction),
    ComplexLiteral(Vec<Expr>),
    VecExpr(Vec<Expr>),
    BuilderExpr(Box<Expr>, Box<Expr>),

    IdentExpr(String),
    LitExpr {
        lit: Literal,
        // #[serde(skip_serializing, skip_deserializing)]
        // span: Option<Span >
    }
}

impl Expr {
    pub fn new_literal(lit: Literal) -> Expr {
        Expr::LitExpr{lit}
        // span: Some(span)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)] //, PartialEq, PartialOrd
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
    VecLiteral(Vec<Literal>),
    #[serde(rename = "object")]
    ObjectLiteral(HashMap<String, Literal>),
    // NULL,
}

impl PartialOrd for Literal {
    fn partial_cmp(&self , other: &Literal ) -> Option<Ordering> {
        match (self, other) {
            (Literal::StringLiteral(l1), Literal::StringLiteral(l2))    => l1.partial_cmp(l2),
            (Literal::IntLiteral(l1), Literal::IntLiteral(l2))          => l1.partial_cmp(l2),
            (Literal::FloatLiteral(l1), Literal::FloatLiteral(l2))      => l1.partial_cmp(l2),
            (Literal::BoolLiteral(l1), Literal::BoolLiteral(l2))        => l1.partial_cmp(l2),
            _                                                           => None
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Literal) -> bool {
        match (self, other) {
            (Literal::StringLiteral(l1), Literal::StringLiteral(l2))    => l1 == l2,
            (Literal::IntLiteral(l1), Literal::IntLiteral(l2))          => l1 == l2,
            (Literal::FloatLiteral(l1), Literal::FloatLiteral(l2))      => l1 == l2,
            (Literal::BoolLiteral(l1), Literal::BoolLiteral(l2))        => l1 == l2,
            _                                                           => false
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
            Literal::StringLiteral(literal)    => literal.to_owned(),
            Literal::IntLiteral(literal)       => literal.to_string(),
            Literal::FloatLiteral(literal)     => literal.to_string(),
            Literal::BoolLiteral(literal)      => literal.to_string(),
            Literal::VecLiteral(vec)           => format!("{:?}", vec),
            Literal::ObjectLiteral(vec)        => format!("{:?}", vec)
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Infix {
    Adition,
    Substraction,
    Divide,
    Multiply,

    Equal,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual,
    GreaterThan,
    LessThan,
    And,
    Or,
}
