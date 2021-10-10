use crate::token;
use crate::token::TokenKind::*;

pub struct Lexer {
    cs: Vec<char>,
    tokens: Vec<token::Token>,
    p: usize,
    line: u32,
    offset: i8,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Lexer {
            cs: code.chars().collect(),
            tokens: Vec::new(),
            p: 0,
            line: 1,
            offset: 0,
        }
    }

    pub fn lexical(mut self) -> Vec<token::Token> {
        self.dissemble();

        let len = self.cs.len();
        let mut new_line = false;

        while self.p < len {
            let mut c = self.cs[self.p];

            while self.is_space(c) {
                if c == '\n' {
                    self.line += 1;
                    self.offset = -1;
                    new_line = true;
                }

                if self.p + 1 >= len {
                    self.p += 1;
                    break;
                }

                self.p += 1;
                c = self.cs[self.p];

                if new_line {
                    self.offset += 1;
                }
            }

            if self.p >= len {
                break;
            }
            if new_line {
                new_line = false;
            }

            if self.is_digit(c) {}
            if self.is_ident(c) {}

            match c {
                '+' => self.emit("+", Add),
                '-' => {
                    if self.next('>') {
                        self.emit("->", RArrow);
                    } else {
                        self.emit("-", Sub);
                    }
                }
                ' ' | '\t' | '\r' | '\n' | '\0' => continue,
                '#' => loop {
                    self.p += 1;
                    if self.cs[self.p] == '\n' {
                        break;
                    }
                },
                _ => panic!("unknown character '{}' ASCII {}", c, c as u8),
            }
            self.p += 1;
        }
        self.emit("Eof", Eof);
        self.tokens
    }

    fn next(&mut self, other: char) -> bool {
        if self.p + 1 >= self.cs.len() {
            return false;
        }

        let c = self.cs[self.p];
        if c == other {
            self.p += 2;
            return true;
        }

        self.p += 1;
        return false;
    }

    fn emit(&mut self, lit: &'static str, kind: token::TokenKind) {
        let lit = String::from(lit);
        let tok = token::Token::new(lit, self.line, self.offset, kind);
        self.tokens.push(tok);
    }

    fn is_space(&self, c: char) -> bool {
        c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == '\0'
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_ident(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn dissemble(&self) {
        for (i, &v) in self.cs.iter().enumerate() {
            println!("\t{:03} {:>5} {:>20}", i, v, v as u8);
        }
    }
}
