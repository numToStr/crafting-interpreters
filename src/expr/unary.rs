use std::fmt::Display;

use crate::token::Token;

use super::{Acceptor, Expr};

#[derive(Debug)]
pub struct Unary<'u> {
    op: Token<'u>,
    right: Box<Expr<'u>>,
}

impl Acceptor for Unary<'_> {
    fn accept(&self, n: &impl super::Visitor) {
        n.visit(self)
    }
}

impl Display for Unary<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.op.lexeme, self.right)
    }
}

impl<'u> Unary<'u> {
    pub fn new(op: Token<'u>, right: Box<Expr<'u>>) -> Self {
        Self { op, right }
    }
}
