use crate::token;

pub struct Lexer {
    list: Vec<char>,
    p: usize,
    tokens: Vec<token::Token>,
    line: u32,
    offset: u8,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Lexer {
            list: code.chars().collect(),
            p: 0,
            tokens: Vec::new(),
            line: 1,
            offset: 0,
        }
    }

    pub fn lexical(&mut self) -> Vec<token::Token> {
        self.dissemble();

        while self.p < self.list.len() {
            self.p += 1;
        }
        vec![]
    }

    fn emit(&mut self, lit: String, kind: token::TokenKind) {
        self.tokens
            .push(token::Token::new(lit, self.line, self.offset, kind));
    }

    fn skip_whitespace(&mut self) {}

    fn lex_digit(&mut self) {}
    fn lex_ident(&mut self) {}

    fn dissemble(&self) {
        for (i, &v) in self.list.iter().enumerate() {
            println!("\t{:03} {:>5} {:>20}", i, v, v as u8);
        }
    }
}
