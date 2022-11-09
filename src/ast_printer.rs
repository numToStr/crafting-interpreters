#![allow(dead_code)]

use lox::{
    expr::{binary::Binary, grouping::Grouping, literal::Literal, unary::Unary, Expr},
    token::Token,
    token_type::TokenType,
};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print() {
        let e = Expr::Binary(Binary::new(
            Box::new(Expr::Unary(Unary::new(
                Token::new(TokenType::Minus, "-", 1),
                Box::new(Expr::Literal(Literal::new(TokenType::Number(123.0)))),
            ))),
            Token::new(TokenType::Star, "*", 1),
            Box::new(Expr::Grouping(Grouping::new(Box::new(Expr::Literal(
                Literal::new(TokenType::Number(45.67)),
            ))))),
        ));

        println!("{e}");
    }
}
