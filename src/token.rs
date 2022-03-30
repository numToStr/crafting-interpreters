use std::fmt::Display;

use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    pub typ: TokenType,
    pub lexeme: Option<String>,
    pub line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?} {:?}", self.line, self.typ, self.lexeme)
    }
}

impl Token {
    pub fn new(typ: TokenType, lexeme: Option<String>, line: usize) -> Self {
        Self { typ, lexeme, line }
    }
}
