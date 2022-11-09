use std::fmt::{Debug, Display};

use self::{binary::Binary, grouping::Grouping, literal::Literal, unary::Unary};

pub mod binary;
pub mod grouping;
pub mod literal;
pub mod unary;

pub trait Visitor {
    fn visit_binary(&self, n: &impl Acceptor);
    fn visit_grouping(&self, n: &impl Acceptor);
    fn visit_literal(&self, n: &impl Acceptor);
    fn visit_unary(&self, n: &impl Acceptor);
}

pub trait Acceptor {
    fn accept(&self, n: &impl Visitor);
}

#[derive(Debug)]
pub enum Expr<'e> {
    Binary(Binary<'e>),
    Grouping(Grouping<'e>),
    Literal(Literal<'e>),
    Unary(Unary<'e>),
}

impl Acceptor for Expr<'_> {
    fn accept(&self, n: &impl Visitor) {
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
