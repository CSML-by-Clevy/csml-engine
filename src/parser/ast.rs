use crate::parser::tokens::*;
use crate::interpreter::message::Message;
use crate::error_format::data::ErrorInfo;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub, BitAnd, BitOr, Rem};

#[derive(PartialEq, Debug, Clone)]
pub struct Flow {
    pub flow_instructions: HashMap<InstructionType, Expr>,
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
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

#[derive(PartialEq, Debug, Clone)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub actions: Expr,
}

#[derive(PartialEq, Debug, Clone)]
pub enum GotoType {
    Step,
    Flow,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ObjectType {
    Goto(GotoType, Identifier),
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
        arg: Vec<Expr>,
        range: RangeInterval
    },
    ComplexLiteral(Vec<Expr>, RangeInterval),
    VecExpr(Vec<Expr>, RangeInterval),
    InfixExpr(Infix, Box<Expr>, Box<Expr>), // RangeInterval
    ForExpr(Identifier, Option<Identifier>, Box<Expr>, Vec<Expr>, RangeInterval),

    ObjectExpr(ObjectType), // RangeInterval ?
    IfExpr(IfStatement),
    BuilderExpr(Box<Expr>, Box<Expr>),

    IdentExpr(Identifier),
    LitExpr(Literal),
}

impl Expr {
    pub fn new_ident(ident: String, interval: Interval, index: Option< Box<Expr> >) -> Identifier {
        Identifier {
            ident,
            interval,
            index
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

#[derive( PartialEq, Debug, Clone, Eq, Hash)]
pub struct RangeInterval {
    pub start: Interval,
    pub end: Interval,
}

#[derive( PartialEq, Debug, Clone, Eq, Hash)]
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

#[derive(Debug, Clone)]
pub struct Identifier {
    pub ident: String,
    pub interval: Interval,
    pub index: Option< Box<Expr> >
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Identifier) -> bool {
        self.ident == other.ident
    }
}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Identifier) -> Option<Ordering> {
        self.ident.partial_cmp(&other.ident)
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    StringLiteral{
        value: String,
        interval: Interval
    },
    IntLiteral{
        value: i64,
        interval: Interval
    },
    FloatLiteral{
        value: f64,
        interval: Interval
    },
    BoolLiteral{
        value: bool,
        interval: Interval
    },
    ArrayLiteral{
        items: Vec<Literal>,
        interval: Interval
    },
    ObjectLiteral{
        properties: HashMap<String, Literal>,
        interval: Interval
    },
    FunctionLiteral{
        name: String,
        value: Box<Literal>,
        interval: Interval
    },
    Null{
        value: String,
        interval: Interval
    },
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Literal) -> Option<Ordering> {
        match (self, other) {
            (Literal::StringLiteral{value: l1, ..},
                Literal::StringLiteral{value: l2, ..}) => l1.partial_cmp(l2),

            (Literal::IntLiteral{value: l1, interval}, Literal::StringLiteral{value: l2, ..}) => match Literal::str_to_literal(l2, interval.to_owned()) {
                Literal::IntLiteral{value, ..} => l1.partial_cmp(&value),
                Literal::FloatLiteral{value, ..} => l1.partial_cmp(&(value as i64)),
                _ => None
            },
            (Literal::StringLiteral{value: l1, interval}, Literal::IntLiteral{value: l2, ..}) => match Literal::str_to_literal(l1, interval.to_owned()) {
                Literal::IntLiteral{value, ..} => value.partial_cmp(l2),
                Literal::FloatLiteral{value, ..} => (value as i64).partial_cmp(l2),
                _ => None
            },

            (Literal::FloatLiteral{value: l1, interval}, Literal::StringLiteral{value: l2, ..}) => match Literal::str_to_literal(l2, interval.to_owned()) {
                Literal::IntLiteral{value, ..} => l1.partial_cmp(&(value as f64)),
                Literal::FloatLiteral{value, ..} => l1.partial_cmp(&value),
                _ => None
            },
            (Literal::StringLiteral{value: l1, interval}, Literal::FloatLiteral{value: l2, ..}) => match Literal::str_to_literal(l1, interval.to_owned()) {
                Literal::IntLiteral{value, ..} => (value as f64).partial_cmp(l2),
                Literal::FloatLiteral{value, ..} => value.partial_cmp(l2),
                _ => None
            },


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
            (
                Literal::IntLiteral{value: l1, interval},
                Literal::StringLiteral{value: l2, ..}
            ) => match Literal::str_to_literal(l2, interval.to_owned()) {
                Literal::IntLiteral{value, ..} => *l1 == value,
                Literal::FloatLiteral{value, ..} => *l1 == value as i64,
                _ => false
            },
            (Literal::StringLiteral{value: l1, interval}, Literal::IntLiteral{value: l2, ..}) => match Literal::str_to_literal(l1, interval.to_owned()) {
                Literal::IntLiteral{value, ..} => *l2 == value,
                Literal::FloatLiteral{value, ..} => *l2 == value as i64 ,
                _ => false
            },

            (Literal::FloatLiteral{value: l1, interval}, Literal::StringLiteral{value: l2, ..}) => match Literal::str_to_literal(l2, interval.to_owned()) {
                Literal::IntLiteral{value, ..} => *l1 == value as f64,
                Literal::FloatLiteral{value, ..} => *l1 == value,
                _ => false
            },
            (Literal::StringLiteral{value: l1, interval}, Literal::FloatLiteral{value: l2, ..}) => match Literal::str_to_literal(l1, interval.to_owned()) {
                Literal::IntLiteral{value, ..} => *l2 == value as f64,
                Literal::FloatLiteral{value, ..} => *l2 == value ,
                _ => false
            },
            (Literal::IntLiteral{value: l1, ..}, Literal::IntLiteral{value: l2, ..}) => l1 == l2,
            (Literal::FloatLiteral{value: l1, ..}, Literal::IntLiteral{value: l2, ..}) => *l1 == *l2 as f64,
            (Literal::IntLiteral{value: l1, ..}, Literal::FloatLiteral{value: l2, ..}) => *l1 as f64 == *l2,
            (Literal::FloatLiteral{value: l1, ..}, Literal::FloatLiteral{value: l2, ..}) => l1 == l2,
            (Literal::BoolLiteral{value: l1, ..}, Literal::BoolLiteral{value: l2, ..}) => l1 == l2,
            (Literal::ArrayLiteral{items: l1, ..}, Literal::ArrayLiteral{items: l2, ..}) => l1 == l2,
            (Literal::FunctionLiteral{name: l1, ..}, Literal::FunctionLiteral{name: l2, ..}) => l1 == l2,
            _ => {
                println!("plopsdsfehgewfdtyiewgfkh");
                false
            },
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
            Literal::ArrayLiteral{..} => Message::lit_to_json(self.to_owned()).to_string(),
            Literal::ObjectLiteral{..} => Message::lit_to_json(self.to_owned()).to_string(),
            Literal::FunctionLiteral{..} => Message::lit_to_json(self.to_owned()).to_string(),
            Literal::Null{value, ..} => value.to_owned(),
        }
    }

    pub fn is_valid(&self) -> Self {
        match self {
            Literal::StringLiteral{..} => self.to_owned(),
            Literal::IntLiteral{..} => self.to_owned(),
            Literal::FloatLiteral{..} => self.to_owned(),
            Literal::BoolLiteral{value: false, interval} => Literal::boolean(false, interval.to_owned()),
            Literal::BoolLiteral{value: true, interval} => Literal::boolean(true, interval.to_owned()),
            Literal::ArrayLiteral{..} => self.to_owned(),
            Literal::ObjectLiteral{..} => self.to_owned(),
            Literal::FunctionLiteral{..} => self.to_owned(),
            Literal::Null{interval, ..} => Literal::boolean(false, interval.to_owned()),
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

    pub fn str_to_literal(stirng: &str, interval: Interval)  -> Literal {
        match (i64::from_str(stirng), f64::from_str(stirng)) {
            (Ok(int), _) =>  Literal::int(int, interval),
            (_, Ok(float)) => Literal::float(float, interval),
            (_, _) => Literal::string(stirng.to_owned(), interval)
        }
    }

    pub fn get_interval(&self)  -> Interval {
        match self {
            Literal::StringLiteral{interval, ..} => interval.to_owned(),
            Literal::IntLiteral{interval, ..} => interval.to_owned(),
            Literal::FloatLiteral{interval, ..} => interval.to_owned(),
            Literal::BoolLiteral{interval, ..} => interval.to_owned(),
            Literal::ArrayLiteral{interval, ..} => interval.to_owned(),
            Literal::ObjectLiteral{interval, ..} => interval.to_owned(),
            Literal::FunctionLiteral{interval, ..} => interval.to_owned(),
            Literal::Null{interval, ..} => interval.to_owned()
        }
    }

    pub fn float(value: f64, interval: Interval) -> Self {
        Literal::FloatLiteral{
            value,
            interval
            
        }
    }

    pub fn int(value: i64, interval: Interval) -> Self {
        Literal::IntLiteral{
            value,
            interval
        }
    }

    pub fn boolean(value: bool, interval: Interval) -> Self {
        Literal::BoolLiteral{
            value,
            interval
        }
    }

    pub fn string(value: String, interval: Interval) -> Self {
        Literal::StringLiteral{
            value,
            interval
        }
    }

    pub fn array(items: Vec<Literal>, interval: Interval) -> Self {
        Literal::ArrayLiteral{
            items,
            interval
        }
    }

    pub fn object(properties: HashMap<String, Literal>, interval: Interval) -> Self {
        Literal::ObjectLiteral{
            properties,
            interval
        }
    }

    pub fn name_object(name: String, value: &Literal, interval: Interval) -> Self {
        Literal::FunctionLiteral{
            name,
            value: Box::new(value.to_owned()),
            interval
        }
    }

    pub fn lit_to_obj(properties: Literal, name: String, interval: Interval) -> Self {
        let mut obj: HashMap<String, Literal> = HashMap::new();

        obj.insert(name, properties);
        Literal::object(obj, interval)
    }

    pub fn null(interval: Interval) -> Self {
        Literal::Null{
            value: NULL.to_owned(),
            interval
        }
    }
}

fn convert_to_numeric(var: &str, interval: Interval) -> Result<Literal, ErrorInfo> {
    let lit = Literal::str_to_literal(var, interval);
    match lit {
        Literal::StringLiteral{..} => Err(ErrorInfo {
                message: "Illegal operation between types".to_owned(),
                interval: lit.get_interval(),
        }),
        lit => Ok(lit)
    }
}

impl Add for Literal {
    type Output = Result<Literal, ErrorInfo>;

    fn add(self, other: Literal) -> Result<Literal, ErrorInfo> {
        match (self, other) {
            (
                Literal::StringLiteral{value: l1, interval: interval1}, 
                Literal::StringLiteral{value: l2, interval: interval2}
            )    => {
                let lit1 = convert_to_numeric(&l1, interval1)?;
                let lit2 = convert_to_numeric(&l2, interval2)?;
                Ok( (lit1 + lit2)? )
            },
            (Literal::StringLiteral{value: l1, interval}, l2)    => {
                let lit = convert_to_numeric(&l1, interval)?;
                Ok( (lit + l2)? )
            },
            (l1, Literal::StringLiteral{value: l2, interval})    => {
                let lit = convert_to_numeric(&l2, interval)?;
                Ok( (l1 + lit)? )
            },
            (Literal::FloatLiteral{value: l1, interval}, Literal::IntLiteral{value: l2, ..})    => Ok(Literal::float(l1 + l2 as f64, interval.to_owned())),
            (Literal::IntLiteral{value: l1, interval}, Literal::FloatLiteral{value: l2, ..})    => Ok(Literal::float(l1 as f64 + l2, interval.to_owned())),
            (Literal::FloatLiteral{value: l1, interval}, Literal::FloatLiteral{value: l2, ..})  => Ok(Literal::float(l1 + l2, interval.to_owned())) ,
            (Literal::IntLiteral{value: l1, interval}, Literal::IntLiteral{value: l2, ..})      => Ok(Literal::int(l1 + l2, interval.to_owned())),
            (Literal::BoolLiteral{value: l1, interval}, Literal::BoolLiteral{value: l2, ..})    => Ok(Literal::int(l1 as i64 + l2 as i64, interval.to_owned())),
            (l1, _) => Err(ErrorInfo {
                message: "Illegal operation + between types".to_owned(),
                interval: l1.get_interval(),
            })
        }
    }
}

impl Sub for Literal {
    type Output = Result<Literal, ErrorInfo>;

    fn sub(self, other: Literal) -> Result<Literal, ErrorInfo> {
        match (self, other) {
            (
                Literal::StringLiteral{value: l1, interval: interval1}, 
                Literal::StringLiteral{value: l2, interval: interval2}
            )    => {
                let lit1 = convert_to_numeric(&l1, interval1)?;
                let lit2 = convert_to_numeric(&l2, interval2)?;
                Ok( (lit1 - lit2)? )
            },
            (Literal::StringLiteral{value: l1, interval}, l2)    => {
                let lit = convert_to_numeric(&l1, interval)?;
                Ok( (lit - l2)? )
            },
            (l1, Literal::StringLiteral{value: l2, interval})    => {
                let lit = convert_to_numeric(&l2, interval)?;
                Ok( (l1 - lit)? )
            },
            (Literal::FloatLiteral{value: l1, interval}, Literal::IntLiteral{value: l2, ..})  => Ok(Literal::float(l1 - l2 as f64, interval.to_owned())),
            (Literal::IntLiteral{value: l1, interval}, Literal::FloatLiteral{value: l2, ..})  => Ok(Literal::float(l1 as f64 - l2, interval.to_owned())),
            (Literal::FloatLiteral{value: l1, interval}, Literal::FloatLiteral{value: l2, ..})=> Ok(Literal::float(l1 - l2, interval.to_owned())),

            (Literal::IntLiteral{value: l1, interval}, Literal::IntLiteral{value: l2, ..})    => Ok(Literal::int(l1 - l2, interval.to_owned())),
            (Literal::BoolLiteral{value: l1, interval}, Literal::BoolLiteral{value: l2, ..})  => Ok(Literal::int(l1 as i64 - l2 as i64, interval.to_owned())),
            (l1, _)                                                                           => Err(ErrorInfo {
                message: "Illegal operation - between types".to_owned(),
                interval: l1.get_interval(),
            })
        }
    }
}

impl Div for Literal {
    type Output = Result<Literal, ErrorInfo>;

    fn div(self, other: Literal) -> Result<Literal, ErrorInfo> {
        match (self, other) {
            (
                Literal::StringLiteral{value: l1, interval: interval1}, 
                Literal::StringLiteral{value: l2, interval: interval2}
            )    => {
                let lit1 = convert_to_numeric(&l1, interval1)?;
                let lit2 = convert_to_numeric(&l2, interval2)?;
                Ok( (lit1 / lit2)? )
            },
            (Literal::StringLiteral{value: l1, interval}, l2)    => {
                let lit = convert_to_numeric(&l1, interval)?;
                Ok( (lit / l2)? )
            },
            (l1, Literal::StringLiteral{value: l2, interval})    => {
                let lit = convert_to_numeric(&l2, interval)?;
                Ok( (l1 / lit)? )
            },
            (Literal::FloatLiteral{value: l1, interval}, Literal::IntLiteral{value: l2, ..})    => {
                if l2 == 0 { return Err(ErrorInfo {
                        message: "Cannot divide by zero-valued".to_owned(),
                        interval: interval.to_owned(),
                    }) 
                }
                Ok(Literal::float(l1 / l2 as f64, interval.to_owned(),) )
            },
            (Literal::IntLiteral{value: l1, interval}, Literal::FloatLiteral{value: l2, ..})    => {
                if l2 == 0.0 { return Err(ErrorInfo {
                        message: "Cannot divide by zero-valued".to_owned(),
                        interval: interval.to_owned(),
                    })
                }
                Ok(Literal::float(l1 as f64 / l2, interval.to_owned(),))
            },
            (Literal::FloatLiteral{value: l1, interval}, Literal::FloatLiteral{value: l2, ..})      => {
                if l2 == 0.0 { return Err(ErrorInfo {
                        message: "Cannot divide by zero-valued".to_owned(),
                        interval: interval.to_owned(),
                    })
                }
                Ok(Literal::float(l1 / l2, interval.to_owned(),))
            },
            (Literal::IntLiteral{value: l1, interval}, Literal::IntLiteral{value: l2, ..})      => {
                if l2 == 0 { return Err(ErrorInfo {
                        message: "Cannot divide by zero-valued".to_owned(),
                        interval: interval.to_owned(),
                    })
                }
                Ok(Literal::int(l1 / l2, interval.to_owned(),) )
            },
            (Literal::BoolLiteral{value: l1, interval}, Literal::BoolLiteral{value: l2, ..})    => {
                if !l2 {
                    return Err(ErrorInfo {
                        message: "Cannot divide by zero-valued".to_owned(),
                        interval: interval.to_owned(),
                    })
                }
                Ok(Literal::int(l1 as i64 / l2 as i64, interval.to_owned(),))
            },
            (l1, _)                                                     => Err(ErrorInfo {
                message: "Illegal operation / between types".to_owned(),
                interval: l1.get_interval(),
            })
        }
    }
}

impl Mul for Literal {
    type Output = Result<Literal, ErrorInfo>;

    fn mul(self, other: Literal) -> Result<Literal, ErrorInfo> {
        match (self, other) {
            (
                Literal::StringLiteral{value: l1, interval: interval1}, 
                Literal::StringLiteral{value: l2, interval: interval2}
            )    => {
                let lit1 = convert_to_numeric(&l1, interval1)?;
                let lit2 = convert_to_numeric(&l2, interval2)?;
                Ok( (lit1 * lit2)? )
            },
            (Literal::StringLiteral{value: l1, interval}, l2)    => {
                let lit = convert_to_numeric(&l1, interval)?;
                Ok( (lit * l2)? )
            },
            (l1, Literal::StringLiteral{value: l2, interval})    => {
                let lit = convert_to_numeric(&l2, interval)?;
                Ok( (l1 * lit)? )
            },
            (Literal::FloatLiteral{value: l1, interval}, Literal::IntLiteral{value: l2, ..})    => Ok(Literal::float(l1 * l2 as f64, interval.to_owned()) ),
            (Literal::IntLiteral{value: l1, interval}, Literal::FloatLiteral{value: l2, ..})    => Ok(Literal::float(l1 as f64 * l2, interval.to_owned() )),
            (Literal::FloatLiteral{value: l1, interval}, Literal::FloatLiteral{value: l2, ..})  => Ok(Literal::float(l1 * l2, interval.to_owned() )),
            (Literal::IntLiteral{value: l1, interval}, Literal::IntLiteral{value: l2, ..})      => Ok(Literal::int(l1 * l2, interval.to_owned() )),
            (Literal::BoolLiteral{value: l1, interval}, Literal::BoolLiteral{value: l2, ..})    => Ok(Literal::int(l1 as i64 * l2 as i64, interval.to_owned() )),
            (l1, _)                                                      => Err(ErrorInfo {
                message: "Illegal operation * between types".to_owned(),
                interval: l1.get_interval(),
            })
        }
    }
}

impl Rem for Literal {
    type Output = Result<Literal, ErrorInfo>;

    fn rem(self, other: Literal) -> Result<Literal, ErrorInfo> {
        match (self, other) {
            (
                Literal::StringLiteral{value: l1, interval: interval1}, 
                Literal::StringLiteral{value: l2, interval: interval2}
            )    => {
                let lit1 = convert_to_numeric(&l1, interval1)?;
                let lit2 = convert_to_numeric(&l2, interval2)?;
                Ok( (lit1 % lit2)? )
            },
            (Literal::StringLiteral{value: l1, interval}, l2)    => {
                let lit = convert_to_numeric(&l1, interval)?;
                Ok( (lit % l2)? )
            },
            (l1, Literal::StringLiteral{value: l2, interval})    => {
                let lit = convert_to_numeric(&l2, interval)?;
                Ok( (l1 % lit)? )
            },
            (Literal::FloatLiteral{value: l1, interval}, Literal::IntLiteral{value: l2, ..})    => Ok(Literal::float(l1 % l2 as f64, interval.to_owned()) ),
            (Literal::IntLiteral{value: l1, interval}, Literal::FloatLiteral{value: l2, ..})    => Ok(Literal::float(l1 as f64 % l2, interval.to_owned() )),
            (Literal::FloatLiteral{value: l1, interval}, Literal::FloatLiteral{value: l2, ..})  => Ok(Literal::float(l1 % l2, interval.to_owned() )),
            (Literal::IntLiteral{value: l1, interval}, Literal::IntLiteral{value: l2, ..})      => Ok(Literal::int(l1 % l2, interval.to_owned() )),
            (Literal::BoolLiteral{value: l1, interval}, Literal::BoolLiteral{value: l2, ..})    => Ok(Literal::int(l1 as i64 % l2 as i64, interval.to_owned() )),
            (l1, _)             => Err(ErrorInfo {
                message: "Illegal operation % between types".to_owned(),
                interval: l1.get_interval(),
            })
        }
    }
}

impl BitAnd for Literal {
    type Output = Self;

    fn bitand(self, other: Self) -> Literal {
        match (self, other) {
            (Literal::BoolLiteral{value: false, interval}, _)   => Literal::boolean(false, interval.to_owned()),
            (_, Literal::BoolLiteral{value: false, interval})   => Literal::boolean(false, interval.to_owned()),
            (Literal::Null{interval, ..}, _)                    => Literal::boolean(false, interval.to_owned()),
            (_, Literal::Null{interval, ..})                    => Literal::boolean(false, interval.to_owned()),
            (l1, ..)                                            => Literal::boolean(true, l1.get_interval()),
        }
    }
}

impl BitOr for Literal {
    type Output = Self;

    fn bitor(self, other: Self) -> Literal {
        match (self, other) {
            (
                Literal::BoolLiteral{value: false, interval},
                Literal::BoolLiteral{value: false, ..}
            )   => Literal::boolean(false, interval.to_owned()),
            (
                Literal::Null{interval, ..}, 
                Literal::Null{..}
            )   => Literal::boolean(false, interval.to_owned()),
            (
                Literal::Null{interval, ..},
                Literal::BoolLiteral{value: false, ..}
            )   => Literal::boolean(false, interval.to_owned()),
            (
                Literal::BoolLiteral{value: false, interval},
                Literal::Null{..},
            )   => Literal::boolean(false, interval.to_owned()),
            (l1, ..)    => Literal::boolean(true, l1.get_interval()),
        }
    }
}