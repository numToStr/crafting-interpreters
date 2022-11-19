use std::fmt::Display;

use crate::RuntimeError;

use super::{literal::Literal, Acceptor, Expr, Visitor};

#[derive(Debug)]
pub struct Grouping<'g> {
    pub expr: Box<Expr<'g>>,
}

impl Acceptor for Grouping<'_> {
    type O<'o> = Literal<'o> where Self: 'o;
    type E<'e> = RuntimeError<'e> where Self: 'e;
    fn accept<'a>(
        &'a self,
        n: &'a impl Visitor<O<'a> = Literal<'a>, E<'a> = RuntimeError<'a>>,
    ) -> Result<Self::O<'a>, Self::E<'a>> {
        n.visit_grouping(self)
    }
}

impl Display for Grouping<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(group {})", self.expr)
    }
}

impl<'g> Grouping<'g> {
    pub fn new(expr: Box<Expr<'g>>) -> Self {
        Self { expr }
    }
}
