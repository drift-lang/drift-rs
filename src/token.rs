use std::fmt::{Debug, Formatter, Result};
use TokenKind::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Eof,
    Literal,
    Number,
    Str,
    Char,
    Float,
    Add,
    Sub,
    Mul,
    Div,
    Sur,
    RArrow,
    LArrow,
    Dot,
    Comma,
    Colon,
    Equal,
    Semicolon,
    Greater,
    Less,
    GrEq,
    LeEq,
    Addr,
    Or,
    Bang,
    BangEq,
    EqEq,
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Underline,
    Def,
    Ret,
    Aop,
    For,
    If,
    Ef,
    Nf,
    Out,
    Go,
    New,
    Use,
    Nil,
}

type KwMap = (&'static str, TokenKind);
type KwType = &'static [KwMap];

pub const KEYWORDS: KwType = &[
    ("def", Def),
    ("ret", Ret),
    ("aop", Aop),
    ("for", For),
    ("if", If),
    ("ef", Ef),
    ("nf", Nf),
    ("out", Out),
    ("go", Go),
    ("new", New),
    ("use", Use),
    ("nil", Nil),
];

pub fn to_kw(lit: &String) -> TokenKind {
    for v in KEYWORDS {
        if lit == v.0 {
            return v.1;
        }
    }
    Literal
}

pub struct Token {
    lit: String,
    line: u32,
    offset: i8,
    kind: TokenKind,
}

impl Token {
    pub fn new(lit: String, line: u32, offset: i8, kind: TokenKind) -> Self {
        Token {
            lit,
            line,
            offset,
            kind,
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{{ {}:{} {:?} {} }}",
            self.line, self.offset, self.kind, self.lit
        )
    }
}
