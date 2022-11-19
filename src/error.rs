use std::fmt::Display;

use crate::token::Token;

#[derive(Debug)]
pub enum ParseError {
    Missing,
}

#[derive(Debug)]
pub struct RuntimeError<'e> {
    pub token: Token<'e>,
    pub msg: &'e str,
}

impl<'e> Display for RuntimeError<'e> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [line: {}]", self.msg, self.token.ln)
    }
}

impl<'e> RuntimeError<'e> {
    pub fn new(token: Token<'e>, msg: &'e str) -> Self {
        Self { token, msg }
    }
}
