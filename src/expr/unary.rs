use std::fmt::Display;

use crate::{token::Token, RuntimeError};

use super::{literal::Literal, ExprAcceptor, Expr, ExprVisitor};

#[derive(Debug)]
pub struct Unary<'u> {
    pub op: Token<'u>,
    pub right: Box<Expr<'u>>,
}

impl ExprAcceptor for Unary<'_> {
    type O<'o> = Literal<'o> where Self: 'o;
    type E<'e> = RuntimeError<'e> where Self: 'e;
    fn accept<'a>(
        &'a self,
        n: &'a impl ExprVisitor<O<'a> = Literal<'a>, E<'a> = RuntimeError<'a>>,
    ) -> Result<Self::O<'a>, Self::E<'a>> {
        n.visit_unary(self)
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
