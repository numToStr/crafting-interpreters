use std::fmt::{Debug, Display};

use crate::token_type::TokenType;

use self::{binary::Binary, grouping::Grouping, unary::Unary};

pub mod binary;
pub mod grouping;
pub mod unary;

pub trait Visitor {
    fn visit(&self, n: &impl Acceptor);
}

pub trait Acceptor {
    fn accept(&self, n: &impl Visitor);
}

#[derive(Debug)]
pub enum Expr<'e> {
    Binary(Binary<'e>),
    Grouping(Grouping<'e>),
    Literal(TokenType<'e>),
    Unary(Unary<'e>),
}

impl Acceptor for Expr<'_> {
    fn accept(&self, n: &impl Visitor) {
        n.visit(self)
    }
}

impl Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(x) => f.write_str(&x.to_string()),
            Self::Grouping(x) => f.write_str(&x.to_string()),
            Self::Literal(x) => x.fmt(f),
            Self::Unary(x) => f.write_str(&x.to_string()),
        }
    }
}
