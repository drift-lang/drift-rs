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
}

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
