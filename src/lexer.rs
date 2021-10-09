use crate::token;

pub struct Lexer {
    code: String,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Lexer { code }
    }

    pub fn lexcial() -> Vec<token::Token> {
        todo!()
    }
}
