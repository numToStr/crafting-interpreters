use crate::{
    gen_ast::{Expr, Visitor},
    token_type::TokenType,
};

#[derive(Debug, Clone, Copy)]
pub struct Interpreter;

impl Visitor<TokenType> for Interpreter {
    fn visit_literal(&self, expr: crate::gen_ast::Literal) -> Result<TokenType, ()> {
        Ok(expr.value)
    }

    fn visit_grouping(&self, expr: crate::gen_ast::Grouping) -> Result<TokenType, ()> {
        self.evaluate(*expr.expr)
    }

    fn visit_unary(&self, expr: crate::gen_ast::Unary) -> Result<TokenType, ()> {
        let right = self.evaluate(*expr.right)?;

        match (expr.op.typ, &right) {
            (TokenType::Minus, TokenType::Number(n)) => {
                dbg!(-n);
            }
            (TokenType::Bang, _) => {
                dbg!(!self.is_truthy(&right));
            }
            _ => {
                dbg!(0.0);
            }
        };

        unreachable!()
    }

    fn visit_binary(&self, expr: crate::gen_ast::Binary) -> Result<TokenType, ()> {
        let left = self.evaluate(*expr.left)?;
        let right = self.evaluate(*expr.right)?;

        match (left, right) {
            (TokenType::Number(l), TokenType::Number(r)) => match expr.op.typ {
                TokenType::Minus => {
                    dbg!(l - r);
                }
                TokenType::Slash => {
                    dbg!(l / r);
                }
                TokenType::Star => {
                    dbg!(l * r);
                }
                TokenType::Plus => {
                    dbg!(l + r);
                }
                TokenType::Greater => {
                    dbg!(l > r);
                }
                TokenType::GreaterEqual => {
                    dbg!(l >= r);
                }
                TokenType::Less => {
                    dbg!(l < r);
                }
                TokenType::LessEqual => {
                    dbg!(l >= r);
                }
                TokenType::BangEqual => {
                    dbg!(l != r);
                }
                TokenType::EqualEqual => {
                    dbg!(l == r);
                }
                _ => unreachable!(),
            },
            _ => panic!("Both operands should be numbers"),
        };

        Err(())
    }
}

impl Interpreter {
    fn evaluate(&self, expr: Expr) -> Result<TokenType, ()> {
        expr.accept(*self)
    }

    fn is_truthy(&self, tty: &TokenType) -> bool {
        !matches!(tty, TokenType::False | TokenType::Nil)
    }

    pub fn interpret(expr: Expr) {
        Self::evaluate(&Self {}, expr).ok();
    }
}
