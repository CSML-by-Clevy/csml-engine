use serde::{
    ser::{SerializeMap, Serializer},
    Deserialize, Serialize,
};

use crate::parser::tokens::*;
use crate::error_format::data::ErrorInfo;
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

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
pub enum InstructionType {
    StartFlow,
    NormalStep(String),
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            InstructionType::StartFlow => write!(f, "Start"),
            InstructionType::NormalStep(ref ident) => write!(f, "{}", ident),
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
pub enum ObjectType {
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
    ElseStmt(Vec<Expr>, RangeInterval),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Expr {
    Block {
        block_type: BlockType,
        arg: Vec<Expr>,
        range: RangeInterval
    },
    ComplexLiteral(Vec<Expr>, RangeInterval),
    VecExpr(Vec<Expr>, RangeInterval),
    InfixExpr(Infix, Box<Expr>, Box<Expr>), // RangeInterval

    ObjectExpr(ObjectType), // RangeInterval ?
    IfExpr(IfStatement),
    BuilderExpr(Box<Expr>, Box<Expr>),

    IdentExpr(SmartIdent),
    LitExpr(SmartLiteral),
}

impl Expr {
    pub fn new_literal(literal: Literal, interval: Interval) -> Expr {
        Expr::LitExpr(SmartLiteral {
            literal,
            interval
        })
    }

    pub fn new_ident(ident: String, interval: Interval) -> SmartIdent {
        SmartIdent {
            ident,
            interval
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Expr::ComplexLiteral(..) => "complex_literal".to_owned(),
            Expr::BuilderExpr(..) => "builder".to_owned(),
            Expr::VecExpr(..) => "Array".to_owned(),
            Expr::IdentExpr(SmartIdent { ident, .. }) => ident.to_owned(),
            Expr::LitExpr(SmartLiteral { literal, .. }) => literal.type_to_string(),
            Expr::ObjectExpr(..) => "function".to_owned(),
            Expr::Block { .. } => "block".to_owned(),
            Expr::IfExpr { .. } => "if".to_owned(),
            Expr::InfixExpr(..) => "infix".to_owned(),
        }
    }
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
pub struct RangeInterval {
    pub start: Interval,
    pub end: Interval,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub struct Interval {
    pub line: u32,
    pub column: u32,
}

impl Interval {
   pub fn new(span: Span) -> Self{
        Interval {
            line: span.line,
            column: span.get_column() as u32,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Literal {
    StringLiteral{
        value: String,
    },
    IntLiteral{
        value: i64,
    },
    FloatLiteral{
        value: f64,
    },
    BoolLiteral{
        value: bool,
    },
    ArrayLiteral{
        items: Vec<Literal>,
    },
    ObjectLiteral{
        properties: HashMap<String, Literal>,
    },
    FunctionLiteral{
        name: String,
        value: Box<Literal>,
    },
    Null{
        value: String,
    },
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Literal) -> Option<Ordering> {
        match (self, other) {
            (Literal::StringLiteral{value: l1, ..},
                Literal::StringLiteral{value: l2, ..}) => l1.partial_cmp(l2),
            (Literal::IntLiteral{value: l1, ..}, 
                Literal::IntLiteral{value: l2, ..}) => l1.partial_cmp(l2),
            (Literal::FloatLiteral{value: l1, ..}, 
                Literal::FloatLiteral{value: l2, ..}) => l1.partial_cmp(l2),
            (Literal::BoolLiteral{value: l1, ..}, 
                Literal::BoolLiteral{value: l2, ..}) => l1.partial_cmp(l2),
            _   => None,
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Literal) -> bool {
        match (self, other) {
            (Literal::StringLiteral{value: l1, ..}, Literal::StringLiteral{value: l2, ..}) => l1 == l2,
            (Literal::IntLiteral{value: l1, ..}, Literal::IntLiteral{value: l2, ..}) => l1 == l2,
            (Literal::FloatLiteral{value: l1, ..}, Literal::FloatLiteral{value: l2, ..}) => l1 == l2,
            (Literal::BoolLiteral{value: l1, ..}, Literal::BoolLiteral{value: l2, ..}) => l1 == l2,
            (Literal::ArrayLiteral{items: l1, ..}, Literal::ArrayLiteral{items: l2, ..}) => l1 == l2,
            (Literal::FunctionLiteral{name: l1, ..}, Literal::FunctionLiteral{name: l2, ..}) => l1 == l2,
            _ => false,
        }
    }
}

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Literal::StringLiteral{value, ..} => value.to_owned(),
            Literal::IntLiteral{value, ..} => value.to_string(),
            Literal::FloatLiteral{value, ..} => value.to_string(),
            Literal::BoolLiteral{value, ..} => value.to_string(),
            Literal::ArrayLiteral{items, ..} => format!("{:?}", items), // serialize first
            Literal::ObjectLiteral{properties, ..} => format!("{:?}", properties),  // serialize first
            Literal::FunctionLiteral{..} => format!("{:?}", self),  // serialize first
            Literal::Null{value, ..} => value.to_owned(),
        }
    }

    pub fn type_to_string(&self) -> String {
        match self {
            Literal::StringLiteral{..} => "string".to_owned(),
            Literal::IntLiteral{..} => "int".to_owned(),
            Literal::FloatLiteral{..} => "float".to_owned(),
            Literal::BoolLiteral{..} => "bool".to_owned(),
            Literal::ArrayLiteral{..} => "array".to_owned(),
            Literal::ObjectLiteral {..} => "object".to_owned(),
            Literal::FunctionLiteral{name, ..} => name.to_owned(),
            Literal::Null{value, ..} => value.to_owned(),
        }
    }

    pub fn float(value: f64) -> Self {
        Literal::FloatLiteral{
            value
        }
    }

    pub fn int(value: i64) -> Self {
        Literal::IntLiteral{
            value
        }
    }

    pub fn boolean(value: bool) -> Self {
        Literal::BoolLiteral{
            value
        }
    }

    pub fn string(value: String) -> Self {
        Literal::StringLiteral{
            value,
        }
    }

    pub fn array(items: Vec<Literal>) -> Self {
        Literal::ArrayLiteral{
            items,
        }
    }

    pub fn object(properties: HashMap<String, Literal>) -> Self {
        Literal::ObjectLiteral{
            properties,
        }
    }

    pub fn name_object(name: String, value: &Literal) -> Self {
        Literal::FunctionLiteral{
            name,
            value: Box::new(value.to_owned())
        }
    }

    pub fn lit_to_obj(properties: Literal, name: String) -> Self {
        let mut obj: HashMap<String, Literal> = HashMap::new();

        obj.insert(name, properties);
        Literal::object(obj)
    }

    pub fn null() -> Self {
        Literal::Null{
            value: NULL.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct SmartIdent {
    pub ident: String,
    pub interval: Interval,
}

impl PartialEq for SmartIdent {
    fn eq(&self, other: &SmartIdent) -> bool {
        self.ident == other.ident
    }
}

impl PartialOrd for SmartIdent {
    fn partial_cmp(&self, other: &SmartIdent) -> Option<Ordering> {
        self.ident.partial_cmp(&other.ident)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmartLiteral {
    pub literal: Literal,
    pub interval: Interval,
}

impl PartialOrd for SmartLiteral {
    fn partial_cmp(&self, other: &SmartLiteral) -> Option<Ordering> {
        self.literal.partial_cmp(&other.literal)
    }
}

impl PartialEq for SmartLiteral {
    fn eq(&self, other: &SmartLiteral) -> bool {
        self.literal == other.literal
    }
}

impl Add for SmartLiteral {
    type Output = Result<SmartLiteral, ErrorInfo>;

    fn add(self, other: SmartLiteral) -> Result<SmartLiteral, ErrorInfo> {
        match (self.literal, other.literal) {
            (Literal::FloatLiteral{value: l1, ..}, Literal::IntLiteral{value: l2, ..})      => Ok(SmartLiteral{literal: Literal::float(l1 + l2 as f64), interval: self.interval}),
            (Literal::IntLiteral{value: l1, ..}, Literal::FloatLiteral{value: l2, ..})      => Ok(SmartLiteral{literal: Literal::float(l1 as f64 + l2), interval: self.interval}),
            (Literal::FloatLiteral{value: l1, ..}, Literal::FloatLiteral{value: l2, ..})    => Ok(SmartLiteral{literal: Literal::float(l1 + l2), interval: self.interval}) ,
            (Literal::IntLiteral{value: l1, ..}, Literal::IntLiteral{value: l2, ..})        => Ok(SmartLiteral{literal: Literal::int(l1 + l2), interval: self.interval}),
            (Literal::BoolLiteral{value: l1, ..}, Literal::BoolLiteral{value: l2, ..})      => Ok(SmartLiteral{literal: Literal::int(l1 as i64 + l2 as i64), interval: self.interval}),
            _=> Err(ErrorInfo {
                message: "Illegal operation + between types".to_owned(),
                interval: self.interval,
            })
            // (Literal::StringLiteral(l1), Literal::StringLiteral(l2))    => Literal::IntLiteral(l1 + l2),
        }
    }
}

impl Sub for SmartLiteral {
    type Output = Result<SmartLiteral, ErrorInfo>;

    fn sub(self, other: SmartLiteral) -> Result<SmartLiteral, ErrorInfo> {
        match (self.literal, other.literal) {
            (Literal::FloatLiteral{value: l1, ..}, Literal::IntLiteral{value: l2, ..})  => Ok(SmartLiteral{literal: Literal::float(l1 - l2 as f64), interval: self.interval}),
            (Literal::IntLiteral{value: l1, ..}, Literal::FloatLiteral{value: l2, ..})  => Ok(SmartLiteral{literal: Literal::float(l1 as f64 - l2), interval: self.interval}),
            (Literal::FloatLiteral{value: l1, ..}, Literal::FloatLiteral{value: l2, ..})=> Ok(SmartLiteral{literal: Literal::float(l1 - l2), interval: self.interval}),

            (Literal::IntLiteral{value: l1, ..}, Literal::IntLiteral{value: l2, ..})    => Ok(SmartLiteral{literal: Literal::int(l1 - l2), interval: self.interval}),
            (Literal::BoolLiteral{value: l1, ..}, Literal::BoolLiteral{value: l2, ..})  => Ok(SmartLiteral{literal: Literal::int(l1 as i64 - l2 as i64), interval: self.interval}),
            _                                                                           => Err(ErrorInfo {
                message: "Illegal operation - between types".to_owned(),
                interval: self.interval,
            })
            // (Literal::StringLiteral(l1), Literal::StringLiteral(l2))    => Literal::IntLiteral(l1 - l2),
        }
    }
}

impl Div for SmartLiteral {
    type Output = Result<SmartLiteral, ErrorInfo>;

    fn div(self, other: SmartLiteral) -> Result<SmartLiteral, ErrorInfo> {
        match (self.literal, other.literal) {
            (Literal::FloatLiteral{value: l1, ..}, Literal::IntLiteral{value: l2, ..})        => {
                if l2 == 0 { return Err(ErrorInfo {
                        message: "Cannot divide by zero-valued".to_owned(),
                        interval: self.interval,
                    }) 
                }
                Ok(SmartLiteral{ literal: Literal::float(l1 / l2 as f64), interval: self.interval })
            },
            (Literal::IntLiteral{value: l1, ..}, Literal::FloatLiteral{value: l2, ..})        => {
                if l2 == 0.0 { return Err(ErrorInfo {
                        message: "Cannot divide by zero-valued".to_owned(),
                        interval: self.interval,
                    })
                }
                Ok(SmartLiteral{ literal: Literal::float(l1 as f64 / l2), interval: self.interval })
            },
            (Literal::FloatLiteral{value: l1, ..}, Literal::FloatLiteral{value: l2, ..})      => {
                if l2 == 0.0 { return Err(ErrorInfo {
                        message: "Cannot divide by zero-valued".to_owned(),
                        interval: self.interval,
                    })
                }
                Ok(SmartLiteral{ literal: Literal::float(l1 / l2), interval: self.interval })
            },
            (Literal::IntLiteral{value: l1, ..}, Literal::IntLiteral{value: l2, ..})          => {
                if l2 == 0 { return Err(ErrorInfo {
                        message: "Cannot divide by zero-valued".to_owned(),
                        interval: self.interval,
                    })
                }
                Ok(SmartLiteral{literal: Literal::int(l1 / l2) , interval: self.interval})
            },
            (Literal::BoolLiteral{value: l1, ..}, Literal::BoolLiteral{value: l2, ..})        => {
                if !l2 {
                    return Err(ErrorInfo {
                        message: "Cannot divide by zero-valued".to_owned(),
                        interval: self.interval,
                    })
                }
                Ok(SmartLiteral{literal: Literal::int(l1 as i64 / l2 as i64), interval: self.interval})
            },
            _                                                           => Err(ErrorInfo {
                message: "Illegal operation / between types".to_owned(),
                interval: self.interval,
            })
            // (Literal::StringLiteral(l1), Literal::StringLiteral(l2))    => Literal::IntLiteral(l1 / l2),
        }
    }
}

impl Mul for SmartLiteral {
    type Output = Result<SmartLiteral, ErrorInfo>;

    fn mul(self, other: SmartLiteral) -> Result<SmartLiteral, ErrorInfo> {
        match (self.literal, other.literal) {
            (Literal::FloatLiteral{value: l1, ..}, Literal::IntLiteral{value: l2, ..})    => Ok(SmartLiteral{ literal: Literal::float(l1 * l2 as f64) , interval: self.interval}),
            (Literal::IntLiteral{value: l1, ..}, Literal::FloatLiteral{value: l2, ..})    => Ok(SmartLiteral{ literal: Literal::float(l1 as f64 * l2) , interval: self.interval}),
            (Literal::FloatLiteral{value: l1, ..}, Literal::FloatLiteral{value: l2, ..})  => Ok(SmartLiteral{ literal: Literal::float(l1 * l2) , interval: self.interval}),
            (Literal::IntLiteral{value: l1, ..}, Literal::IntLiteral{value: l2, ..})      => Ok(SmartLiteral{ literal: Literal::int(l1 * l2) , interval: self.interval}),
            (Literal::BoolLiteral{value: l1, ..}, Literal::BoolLiteral{value: l2, ..})    => Ok(SmartLiteral{ literal: Literal::int(l1 as i64 * l2 as i64) , interval: self.interval}),
            _                                                       => Err(ErrorInfo {
                message: "Illegal operation * between types".to_owned(),
                interval: self.interval,
            })
            // (Literal::StringLiteral(l1), Literal::StringLiteral(l2))    => Literal::IntLiteral(l1 * l2),
        }
    }
}
