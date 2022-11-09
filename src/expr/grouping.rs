use std::fmt::Display;

use super::{Acceptor, Expr};

#[derive(Debug)]
pub struct Grouping<'g> {
    expr: Box<Expr<'g>>,
}

impl Acceptor for Grouping<'_> {
    fn accept(&self, n: &impl super::Visitor) {
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
