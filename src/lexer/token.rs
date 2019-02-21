use nom::*;
use std::iter::Enumerate;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Illegal,
    EOF,
    NewL,
    // identifier and literals
    Ident(String),
    StringLiteral(String),
    Label(String),
    IntLiteral(i64),
    // BoolLiteral(bool),
    // statements
    Assign,
    If,
    End,
    Retry, // macro goto sur current label
    Goto,

    // operators
    GreaterThan,
    LessThan,

    // reserved words
    Function,
    Remember,
    // Goto,
    // Show,
    // Typing,
    // Say,

    // punctuations
    Comma,
    Colon,
    SemiColon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    L2Brace,
    R2Brace,
    LBracket,
    RBracket,
}
