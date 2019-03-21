use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Flow {
    pub accept: Vec<Expr>,
    pub steps: Vec<Step>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Step {
    FlowStarter { ident: Ident, list: Vec<Expr> },
    Block { label: Ident, actions: Vec<Expr> },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Expr {
    Reserved {
        fun: Ident,
        arg: Box<Expr>,
    },
    Action {
        builtin: Ident,
        args: Box<Expr>,
    },
    IfExpr {
        cond: Vec<Expr>,
        consequence: Vec<Expr>,
    },
    InfixExpr(Infix, Box<Expr>), //, Box<Expr>
    // BuilderExpr(Box<Expr>, Box<Expr>),
    Goto(Ident),
    LitExpr(Literal),
    IdentExpr(Ident),
    VecExpr(Vec<Expr>),
    Empty,
    // ArrayLit(Vec<Literal>),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Literal {
    StringLiteral(String),
    IntLiteral(i64),
    BoolLiteral(bool),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
pub struct Ident(pub String);

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Infix {
    // Plus,
    // Minus,
    // Divide,
    // Multiply,
    Equal,
    // NotEqual,
    GreaterThanEqual,
    LessThanEqual,
    GreaterThan,
    LessThan,
    And,
    Or,
}
