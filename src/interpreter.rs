use std::borrow::Cow;

use crate::{
    expr::{
        binary::Binary, grouping::Grouping, literal::Literal, unary::Unary, Expr, ExprAcceptor,
        ExprVisitor,
    },
    statement::{Expression, Print, Statement, StmtAcceptor, StmtVisitor},
    token::Token,
    token_type::TokenType,
    RuntimeError,
};

#[derive(Debug)]
pub struct Interpreter {}

impl Interpreter {
    pub fn interpret<'o>(stmts: &'o [Statement<'o>]) -> Result<(), RuntimeError<'o>> {
        for stmt in stmts {
            Self::execute(&Self {}, stmt)?;
        }

        Ok(())
    }

    fn evaluate<'o>(&'o self, expr: &'o Expr<'o>) -> Result<Literal<'o>, RuntimeError<'o>> {
        expr.accept(self)
    }

    fn execute<'o>(&'o self, stmt: &'o Statement<'o>) -> Result<(), RuntimeError<'o>> {
        stmt.accept(self)
    }

    fn is_equal(&self, left: Literal, right: Literal) -> bool {
        match (left, right) {
            (Literal::Nil, Literal::Nil) => true,
            (Literal::Nil, _) => false,
            (Literal::Bool(l), Literal::Bool(r)) => l == r,
            (Literal::Bool(_), _) => false,
            (Literal::Number(l), Literal::Number(r)) => l == r,
            (Literal::Number(_), _) => false,
            (Literal::String(l), Literal::String(r)) => l == r,
            (Literal::String(_), _) => false,
        }
    }

    fn number_err<'n>(&self, op: Token<'n>) -> RuntimeError<'n> {
        RuntimeError::new(op, "Operator must be a number!")
    }
}

impl ExprVisitor for Interpreter {
    type O<'o> = Literal<'o>;
    type E<'e> = RuntimeError<'e>;

    fn visit_binary<'b>(&'b self, n: &'b Binary<'b>) -> Result<Self::O<'b>, Self::E<'b>> {
        let left = self.evaluate(&n.left)?;
        let right = self.evaluate(&n.right)?;
        match (n.op.ty, left, right) {
            (TokenType::Minus, Literal::Number(l), Literal::Number(r)) => {
                Ok(Literal::Number(l - r))
            }
            (TokenType::Slash, Literal::Number(l), Literal::Number(r)) => {
                Ok(Literal::Number(l / r))
            }
            (TokenType::Star, Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l * r)),
            (TokenType::Plus, Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l + r)),
            (TokenType::Plus, Literal::String(l), Literal::String(r)) => {
                Ok(Literal::String(Cow::Owned(l.into_owned() + &r)))
            }
            (TokenType::Greater, Literal::Number(l), Literal::Number(r)) => {
                Ok(Literal::Bool(l > r))
            }
            (TokenType::GreaterEqual, Literal::Number(l), Literal::Number(r)) => {
                Ok(Literal::Bool(l >= r))
            }
            (TokenType::Less, Literal::Number(l), Literal::Number(r)) => Ok(Literal::Bool(l < r)),
            (TokenType::LessEqual, Literal::Number(l), Literal::Number(r)) => {
                Ok(Literal::Bool(l <= r))
            }
            (TokenType::EqualEqual, l, r) => Ok(Literal::Bool(self.is_equal(l, r))),
            (TokenType::BangEqual, l, r) => Ok(Literal::Bool(!self.is_equal(l, r))),
            _ => todo!(),
        }
    }

    fn visit_grouping<'g>(&'g self, n: &'g Grouping<'g>) -> Result<Self::O<'g>, Self::E<'g>> {
        self.evaluate(&n.expr)
    }

    fn visit_literal<'l>(&self, n: &Literal<'l>) -> Result<Self::O<'l>, Self::E<'l>> {
        Ok(n.to_owned())
    }

    fn visit_unary<'u>(&'u self, n: &'u Unary<'u>) -> Result<Self::O<'u>, Self::E<'u>> {
        let right = self.evaluate(&n.right)?;
        match (n.op.ty, right) {
            (TokenType::Minus, Literal::Number(num)) => Ok(Literal::Number(-num)),
            (TokenType::Bang, Literal::Bool(b)) => Ok(Literal::Bool(!b)),
            (TokenType::Bang, Literal::Nil) => Ok(Literal::Bool(true)),
            (TokenType::Minus, _) => Err(self.number_err(n.op)),
            _ => unreachable!(),
        }
    }
}

impl StmtVisitor for Interpreter {
    type O<'o> = ();
    type E<'e> = RuntimeError<'e>;

    fn visit_expression<'b>(&'b self, n: &'b Expression<'b>) -> Result<Self::O<'b>, Self::E<'b>> {
        self.evaluate(&n.expr)?;
        Ok(())
    }

    fn visit_print<'g>(&'g self, n: &'g Print<'g>) -> Result<Self::O<'g>, Self::E<'g>> {
        let value = self.evaluate(&n.expr)?;
        println!("{value}");
        Ok(())
    }
}
