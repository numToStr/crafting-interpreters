use std::fmt::Display;

use crate::token::Token;

use super::{Acceptor, Expr};

#[derive(Debug)]
pub struct Binary<'b> {
    left: Box<Expr<'b>>,
    op: Token<'b>,
    right: Box<Expr<'b>>,
}

impl Acceptor for Binary<'_> {
    fn accept(&self, n: &impl super::Visitor) {
        n.visit_binary(self)
    }
}

impl Display for Binary<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.op.lexeme, self.left, self.right)
    }
}

impl<'b> Binary<'b> {
    pub fn new(left: Box<Expr<'b>>, op: Token<'b>, right: Box<Expr<'b>>) -> Self {
        Self { left, op, right }
    }
}
