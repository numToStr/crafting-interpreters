use std::fmt::{Debug, Display};

use crate::RuntimeError;

use self::{binary::Binary, grouping::Grouping, literal::Literal, unary::Unary};

pub mod binary;
pub mod grouping;
pub mod literal;
pub mod unary;

pub trait ExprVisitor {
    type O<'o>
    where
        Self: 'o;
    type E<'e>
    where
        Self: 'e;
    fn visit_binary<'b>(&'b self, n: &'b Binary<'b>) -> Result<Self::O<'b>, Self::E<'b>>;
    fn visit_grouping<'g>(&'g self, n: &'g Grouping<'g>) -> Result<Self::O<'g>, Self::E<'g>>;
    fn visit_literal<'l>(&'l self, n: &'l Literal<'l>) -> Result<Self::O<'l>, Self::E<'l>>;
    fn visit_unary<'u>(&'u self, n: &'u Unary<'u>) -> Result<Self::O<'u>, Self::E<'u>>;
}

pub trait ExprAcceptor {
    type O<'o>
    where
        Self: 'o;
    type E<'e>
    where
        Self: 'e;
    fn accept<'a>(
        &'a self,
        n: &'a impl ExprVisitor<O<'a> = Self::O<'a>, E<'a> = Self::E<'a>>,
    ) -> Result<Self::O<'a>, Self::E<'a>>;
}

#[derive(Debug)]
pub enum Expr<'e> {
    Binary(Binary<'e>),
    Grouping(Grouping<'e>),
    Literal(Literal<'e>),
    Unary(Unary<'e>),
}

impl ExprAcceptor for Expr<'_> {
    type O<'o> = Literal<'o> where Self: 'o;
    type E<'e> = RuntimeError<'e> where Self: 'e;
    fn accept<'a>(
        &'a self,
        n: &'a impl ExprVisitor<O<'a> = Literal<'a>, E<'a> = RuntimeError<'a>>,
    ) -> Result<Self::O<'a>, Self::E<'a>> {
        match self {
            Expr::Binary(x) => x.accept(n),
            Expr::Grouping(x) => x.accept(n),
            Expr::Literal(x) => x.accept(n),
            Expr::Unary(x) => x.accept(n),
        }
    }
}

impl Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(x) => f.write_str(&x.to_string()),
            Self::Grouping(x) => f.write_str(&x.to_string()),
            Self::Literal(x) => f.write_str(&x.to_string()),
            Self::Unary(x) => f.write_str(&x.to_string()),
        }
    }
}
