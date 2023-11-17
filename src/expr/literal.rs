use std::{borrow::Cow, fmt::Display};

use crate::RuntimeError;

use super::{ExprAcceptor, ExprVisitor};

#[derive(Debug, Clone)]
pub enum Literal<'l> {
    Nil,
    Bool(bool),
    Number(f64),
    String(Cow<'l, str>),
}

impl ExprAcceptor for Literal<'_> {
    type O<'o> = Literal<'o> where Self: 'o;
    type E<'e> = RuntimeError<'e> where Self: 'e;
    fn accept<'a>(
        &'a self,
        n: &'a impl ExprVisitor<O<'a> = Literal<'a>, E<'a> = RuntimeError<'a>>,
    ) -> Result<Self::O<'a>, Self::E<'a>> {
        n.visit_literal(self)
    }
}

impl Display for Literal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
