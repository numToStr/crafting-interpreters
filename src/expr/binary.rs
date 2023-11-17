use std::fmt::Display;

use crate::{token::Token, RuntimeError};

use super::{literal::Literal, ExprAcceptor, Expr, ExprVisitor};

#[derive(Debug)]
pub struct Binary<'b> {
    pub left: Box<Expr<'b>>,
    pub op: Token<'b>,
    pub right: Box<Expr<'b>>,
}

impl ExprAcceptor for Binary<'_> {
    type O<'o> = Literal<'o> where Self: 'o;
    type E<'e> = RuntimeError<'e> where Self: 'e;
    fn accept<'a>(
        &'a self,
        n: &'a impl ExprVisitor<O<'a> = Literal<'a>, E<'a> = RuntimeError<'a>>,
    ) -> Result<Self::O<'a>, Self::E<'a>> {
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
