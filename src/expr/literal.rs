use std::fmt::Display;

use crate::token_type::TokenType;

use super::Acceptor;

#[derive(Debug)]
pub struct Literal<'l> {
    value: TokenType<'l>,
}

impl Acceptor for Literal<'_> {
    fn accept(&self, n: &impl super::Visitor) {
        n.visit_literal(self)
    }
}

impl<'l> Literal<'l> {
    pub fn new(value: TokenType<'l>) -> Self {
        Self { value }
    }
}

impl Display for Literal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}
