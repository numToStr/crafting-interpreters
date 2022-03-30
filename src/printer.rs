use crate::{
    gen_ast::{Binary, Expr, Grouping, Literal, Unary, Visitor},
    token_type::TokenType,
};

#[derive(Debug, Clone, Copy)]
pub struct Printer {}

impl Printer {
    #[allow(unused)]
    fn print(self, expr: Expr) -> Result<String, ()> {
        expr.accept(self)
    }

    fn parenthesize(
        &self,
        op: Option<String>,
        left: Option<Box<Expr>>,
        right: Option<Box<Expr>>,
    ) -> Result<String, ()> {
        let mut string = String::new();

        string.push('(');
        string.push_str(&op.unwrap_or_else(|| "".to_string()));

        if let Some(x) = left {
            string.push(' ');
            string.push_str(&x.accept(*self)?);
        }

        if let Some(x) = right {
            string.push(' ');
            string.push_str(&x.accept(*self)?);
        }

        string.push(')');

        Ok(string)
    }
}

impl Visitor<String> for Printer {
    fn visit_binary(&self, expr: Binary) -> Result<String, ()> {
        self.parenthesize(expr.op.lexeme, Some(expr.left), Some(expr.right))
    }

    fn visit_grouping(&self, expr: Grouping) -> Result<String, ()> {
        self.parenthesize(Some("group".into()), Some(expr.expr), None)
    }

    fn visit_literal(&self, expr: Literal) -> Result<String, ()> {
        if let TokenType::Nil = expr.value {
            return Ok("nil".into());
        }
        Ok(format!("{:?}", expr.value))
    }

    fn visit_unary(&self, expr: Unary) -> Result<String, ()> {
        self.parenthesize(expr.op.lexeme, None, Some(expr.right))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        gen_ast::{Binary, Expr, Grouping, Literal, Unary},
        printer::Printer,
        token::Token,
        token_type::TokenType,
    };

    #[test]
    fn check_expr() {
        let exprs = Expr::Binary(Binary {
            left: Box::new(Expr::Unary(Unary {
                op: Token::new(TokenType::Minus, Some("-".into()), 1),
                right: Box::new(Expr::Literal(Literal {
                    value: TokenType::Number(123.0),
                })),
            })),
            op: Token::new(TokenType::Star, Some("*".into()), 1),
            right: Box::new(Expr::Grouping(Grouping {
                expr: Box::new(Expr::Literal(Literal {
                    value: TokenType::Number(45.67),
                })),
            })),
        });

        assert_eq!(
            Printer::print(Printer {}, exprs).unwrap(),
            "(* (- Number(123.0)) (group Number(45.67)))".to_string()
        );
    }
}
