use nom::*;
use nom::types::*;
use nom_locate::LocatedSpan;
use std::iter::Enumerate;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

pub type Span<'a> = LocatedSpan<CompleteByteSlice<'a>>;

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
    Flow(Span<'a>),
    Goto(Span<'a>),
    Assign(Span<'a>),
    Remember(Span<'a>),
    If(Span<'a>),

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
    DoubleQuote(Span<'a>),

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
