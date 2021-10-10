use std::fmt::{Debug, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Eof,
    Literal,
    Number,
    String,
    Char,
    Float,
    Add,
    Sub,
    Mul,
    Div,
    Sur,

    Def,
    Ret,
    If,
    Aop,
    For,
    Ef,
    Nf,
    Out,
    Go,
    New,
    Use,
    Nil,
}

// pub const KLiteral: Vec<&'static str> = vec![
//     "def", "ret", "if", "aop", "for", "ef", "nf", "out", "go", "new", "use", "nil",
// ];

pub const K: Vec<TokenKind> = vec![TokenKind::Def];
pub const V: Vec<&'static str> = vec!["def"];

// use TokenKind::*;

pub struct Token {
    literal: String,
    line: u32,
    offset: u8,
    kind: TokenKind,
}

impl Token {
    pub fn new(literal: String, line: u32, offset: u8, kind: TokenKind) -> Self {
        Token {
            literal,
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
            self.line, self.offset, self.kind, self.literal
        )
    }
}
