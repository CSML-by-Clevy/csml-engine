use nom::*;
use nom::types::*;
use nom_locate::LocatedSpan;
use std::iter::Enumerate;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

pub type Span<'a> = LocatedSpan<CompleteByteSlice<'a>>;

// ####################################################################################
pub const PORT: &str = "3002";

pub const WHITE_SPACE: &str = " \t\n\r";
pub const START_COMMENT: &str = "/*";
pub const END_COMMENT: &str = "*/";


pub const EQUAL: &str = "==";
pub const ASSIGN: &str = "=";

pub const OR: &str = "||";
pub const AND: &str = "&&";


pub const GREATER_THAN_EQUAL: &str = ">=";
pub const LESS_THAN_EQUAL: &str = "<=";
pub const GREATER_THAN: &str = ">";
pub const LESS_THAN: &str = "<";


pub const COMMA: &str = ",";
pub const DOT: &str = ".";
pub const SEMICOLON: &str = ";";
pub const COLON: &str = ":";

pub const L_PAREN: &str = "(";
pub const R_PAREN: &str = ")";
pub const L_BRACE: &str = "{";
pub const R_BRACE: &str = "}";

pub const L_BRACKET: &str = "[";
pub const R_BRACKET: &str = "]";


pub const IF: &str = "if";
pub const FLOW: &str = "flow";
pub const GOTO: &str = "goto";
pub const IMPORT: &str = "import";
pub const STEP: &str = "step";
pub const AS: &str = "as";

pub const REMEMBER: &str = "remember";
pub const RETRY: &str = "retry";
pub const SAY: &str = "say";
pub const ASK: &str = "ask";
pub const RESPOND: &str = "respond";


pub const TRUE: &str = "true";
pub const FALSE: &str = "false";


pub const FROM: &str = "from";
pub const FILE: &str = "file";


pub const TYPING: &str = "Typing";
pub const WAIT: &str = "Wait";
pub const TEXT: &str = "Text";
pub const URL: &str = "Url";
pub const IMAGE: &str = "Image";
pub const ONE_OF: &str = "OneOf";
pub const QUESTION: &str = "Question";

pub const METEO: &str = "Meteo";
pub const WTTJ: &str = "WTTJ";
pub const GET_GSHEET: &str = "GetGSheet";
pub const APPEND_GSHEET: &str = "AppendGsheet";
pub const HUB_SPOT: &str = "Hubspot";

pub const EVENT: &str = "event";

pub const PAST: &str = "past";
pub const MEMORY: &str = "memory";
pub const METADATA: &str = "metadata";

pub const GET_VALUE: &str = "getvalue";
pub const FIRST: &str = "first";

// ####################################################################################

#[derive(PartialEq, Debug, Clone)]
pub enum Token<'a> {
    // Special Tokens
    Illegal(Span<'a>),
    EOF,

    // identifiers
    Ident(String, Span<'a>),
    ReservedFunc(String, Span<'a>),
    StringLiteral(String, Span<'a>),
    ComplexString(Vec<Token<'a>>),
    IntLiteral(i64, Span<'a>),
    BoolLiteral(bool, Span<'a>),

    // statements
    If(Span<'a>),
    Flow(Span<'a>),
    Goto(Span<'a>),
    Assign(Span<'a>),
    Remember(Span<'a>),
    Import(Span<'a>),

    Step(Span<'a>),
    FromFile(Span<'a>),
    As(Span<'a>),

    // operators
    Equal(Span<'a>),
    GreaterThan(Span<'a>),
    LessThan(Span<'a>),
    GreaterThanEqual(Span<'a>),
    LessThanEqual(Span<'a>),
    And(Span<'a>),
    Or(Span<'a>),

    // punctuations
    Space,
    NewLine,
    L2Brace,
    R2Brace,

    Comma(Span<'a>),
    Dot(Span<'a>),
    Colon(Span<'a>),
    SemiColon(Span<'a>),
    LParen(Span<'a>),
    RParen(Span<'a>),
    LBrace(Span<'a>),
    RBrace(Span<'a>),
    LBracket(Span<'a>),
    RBracket(Span<'a>),
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
pub struct Tokens<'a> {
    pub tok: &'a [Token<'a>],
    pub start: usize,
    pub end: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(vec: &'a [Token]) -> Self {
        Tokens {
            tok: vec,
            start: 0,
            end: vec.len(),
        }
    }
}

impl<'a> InputLength for Tokens<'a> {
    // #[inline]
    fn input_len(&self) -> usize {
        self.tok.len()
    }
}

impl<'a> AtEof for Tokens<'a> {
    // #[inline]
    fn at_eof(&self) -> bool {
        true
    }
}

impl<'a> InputTake for Tokens<'a> {
    // #[inline]
    fn take(&self, count: usize) -> Self {
        Tokens {
            tok: &self.tok[0..count],
            start: 0,
            end: count,
        }
    }

    // #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.tok.split_at(count);
        let first = Tokens {
            tok: prefix,
            start: 0,
            end: prefix.len(),
        };
        let second = Tokens {
            tok: suffix,
            start: 0,
            end: suffix.len(),
        };
        (second, first)
    }
}

impl<'a> InputLength for Token<'a> {
    // #[inline]
    fn input_len(&self) -> usize {
        1
    }
}

impl<'a> Slice<Range<usize>> for Tokens<'a> {
    // #[inline]
    fn slice(&self, range: Range<usize>) -> Self {
        Tokens {
            tok: self.tok.slice(range.clone()),
            start: self.start + range.start,
            end: self.start + range.end,
        }
    }
}

impl<'a> Slice<RangeTo<usize>> for Tokens<'a> {
    // #[inline]
    fn slice(&self, range: RangeTo<usize>) -> Self {
        self.slice(0..range.end)
    }
}

impl<'a> Slice<RangeFrom<usize>> for Tokens<'a> {
    // #[inline]
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        self.slice(range.start..self.end - self.start)
    }
}

impl<'a> Slice<RangeFull> for Tokens<'a> {
    // #[inline]
    fn slice(&self, _: RangeFull) -> Self {
        Tokens {
            tok: self.tok,
            start: self.start,
            end: self.end,
        }
    }
}

impl<'a> InputIter for Tokens<'a> {
    type Item = &'a Token<'a>;
    type RawItem = Token<'a>;
    type Iter = Enumerate<::std::slice::Iter<'a, Token<'a>>>;
    type IterElem = ::std::slice::Iter<'a, Token<'a>>;

    // #[inline]
    fn iter_indices(&self) -> Enumerate<::std::slice::Iter<'a, Token<'a>>> {
        self.tok.iter().enumerate()
    }
    // #[inline]
    fn iter_elements(&self) -> ::std::slice::Iter<'a, Token<'a>> {
        self.tok.iter()
    }
    // #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::RawItem) -> bool,
    {
        self.tok.iter().position(|b| predicate(b.clone()))
    }
    // #[inline]
    fn slice_index(&self, count: usize) -> Option<usize> {
        if self.tok.len() >= count {
            Some(count)
        } else {
            None
        }
    }
}
