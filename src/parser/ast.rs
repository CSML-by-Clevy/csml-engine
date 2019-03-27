use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

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
        cond: Box<Expr>,
        consequence: Vec<Expr>,
    },
    InfixExpr(Infix, Box<Expr>, Box<Expr>),
    // BuilderExpr(Box<Expr>, Box<Expr>),
    Goto(Ident),
    LitExpr(Literal),
    IdentExpr(Ident),
    VecExpr(Vec<Expr>),
    Empty,
    // ArrayLit(Vec<Literal>),
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub enum Literal {
    StringLiteral(String),
    IntLiteral(i64),
    BoolLiteral(bool),
}

impl Ord for Literal {
    fn cmp(&self, other: &Literal) -> Ordering {
        match (self, other) {
            (Literal::StringLiteral(l1), Literal::StringLiteral(l2))    => l1.cmp(l2),
            (Literal::IntLiteral(l1), Literal::IntLiteral(l2))          => l1.cmp(l2),
            (Literal::BoolLiteral(l1), Literal::BoolLiteral(l2))        => l1.cmp(l2),
            _                                                           => Ordering::Less
        }
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Literal) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Literal) -> bool {
        match (self, other) {
            (Literal::StringLiteral(l1), Literal::StringLiteral(l2))    => l1 == l2,
            (Literal::IntLiteral(l1), Literal::IntLiteral(l2))          => l1 == l2,
            (Literal::BoolLiteral(l1), Literal::BoolLiteral(l2))        => l1 == l2,
            _                                                           => false
        }
    }
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
