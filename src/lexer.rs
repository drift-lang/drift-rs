use crate::token;
use crate::token::{Token, TokenKind::*};

type TokenMap = (&'static str, token::TokenKind);

#[derive(Debug)]
struct Entry(char, char, TokenMap, TokenMap);

const LEXER_ENTRYS: &'static [Entry] = &[
    Entry('-', '>', ("->", RArrow), ("-", Sub)),
    Entry('=', '=', ("==", EqEq), ("=", Equal)),
];

fn get_entry<'a>(p: char) -> Option<&'a Entry> {
    for i in LEXER_ENTRYS {
        if i.0 == p {
            return Some(i);
        }
    }
    None
}

pub struct Lexer {
    cs: Vec<char>,
    tokens: Vec<Token>,
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

    pub fn lexical(mut self) -> Vec<Token> {
        let len = self.cs.len();
        let mut new_line = true;

        while self.p < len {
            let mut c = self.cs[self.p];

            self.skip_whitespace(&mut c, &mut new_line);

            if self.p >= len {
                break;
            }
            if new_line {
                new_line = false;
            }

            if self.is_digit(c) {
                let mut lit = String::new();

                while self.is_digit(c) {
                    lit.push(c);
                    self.p += 1;
                    c = self.cs[self.p];
                }

                self.emit_lit(lit, Number);
                continue;
            }

            if self.is_ident(c) {
                let mut lit = String::new();

                while self.is_ident(c) {
                    lit.push(c);
                    self.p += 1;
                    c = self.cs[self.p];
                }

                let kind = token::to_kw(&lit);
                self.emit_lit(lit, kind);
                continue;
            }

            self.other(&mut c);
        }
        self.emit("EOF", Eof);
        self.tokens
    }

    fn skip_whitespace(&mut self, c: &mut char, new_line: &mut bool) {
        while self.is_space(*c) {
            if *c == '\n' {
                self.line += 1;
                self.offset = -1;
                *new_line = true;
            }

            if self.p + 1 >= self.cs.len() {
                self.p += 1;
                break;
            }
            self.p += 1;
            *c = self.cs[self.p];

            if *new_line {
                self.offset += 1;
            }
        }
    }

    fn next(&mut self, other: char) -> bool {
        let i = self.p + 1;

        if i >= self.cs.len() {
            return false;
        }
        if self.cs[i] == other {
            self.p += 1;
            return true;
        }
        return false;
    }

    fn emit(&mut self, lit: &'static str, kind: token::TokenKind) {
        let lit = String::from(lit);
        let tok = Token::new(lit, self.line, self.offset, kind);
        self.tokens.push(tok);
    }

    fn emit_lit(&mut self, lit: String, kind: token::TokenKind) {
        self.tokens
            .push(Token::new(lit, self.line, self.offset, kind));
    }

    fn other(&mut self, c: &mut char) {
        if let Some(en) = get_entry(*c) {
            if self.next(en.1) {
                self.emit(en.2 .0, en.2 .1);
            } else {
                self.emit(en.3 .0, en.3 .1);
            }
            self.p += 1;
        } else {
            match *c {
                '+' => self.emit("+", Add),
                '#' => loop {
                    self.p += 1;
                    if self.cs[self.p] == '\n' {
                        break;
                    }
                },
                _ => panic!("unknown character '{}' ASCII {}", c, *c as u8),
            }
        }
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
}
