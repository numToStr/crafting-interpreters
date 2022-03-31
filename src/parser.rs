//! expression     → equality ;
//! equality       → comparison ( ( "!=" | "==" ) comparison )* ;
//! comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
//! term           → factor ( ( "-" | "+" ) factor )* ;
//! factor         → unary ( ( "/" | "*" ) unary )* ;
//! unary          → ( "!" | "-" ) unary | primary ;
//! primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;

use crate::{
    gen_ast::{Binary, Expr, Grouping, Literal, Unary},
    token::Token,
    token_type::TokenType,
};

#[derive(Debug)]
pub struct Parser<'p> {
    tokens: &'p [Token],
    current: usize,
}

impl<'p> Parser<'p> {
    pub fn new(tokens: &'p [Token]) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Box<Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Box<Expr> {
        self.equality()
    }

    /// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Box<Expr> {
        let mut left = self.comparison();

        while self.one_of(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison();
            left = Box::new(Expr::Binary(Binary { left, op, right }))
        }

        left
    }

    /// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Box<Expr> {
        let mut left = self.term();

        while self.one_of(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous();
            let right = self.term();
            left = Box::new(Expr::Binary(Binary { left, op, right }));
        }

        left
    }

    /// term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Box<Expr> {
        let mut left = self.factor();

        while self.one_of(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.previous();
            let right = self.factor();
            left = Box::new(Expr::Binary(Binary { left, op, right }))
        }

        left
    }

    /// factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Box<Expr> {
        let mut left = self.unary();

        while self.one_of(&[TokenType::Slash, TokenType::Star]) {
            let op = self.previous();
            let right = self.unary();
            left = Box::new(Expr::Binary(Binary { left, op, right }))
        }

        left
    }

    /// unary          → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Box<Expr> {
        if self.one_of(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary();
            return Box::new(Expr::Unary(Unary { op, right }));
        }

        self.primary().unwrap()
    }

    /// primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Option<Box<Expr>> {
        match self.tokens.get(self.current).cloned().unwrap().typ {
            TokenType::LeftParen => {
                let expr = self.expression();
                self.consume(TokenType::RightParen);
                Some(Box::new(Expr::Grouping(Grouping { expr })))
            }
            x @ (TokenType::True
            | TokenType::False
            | TokenType::Nil
            | TokenType::Number(_)
            | TokenType::String(_)) => {
                self.advance();
                Some(Box::new(Expr::Literal(Literal { value: x })))
            }
            _ => {
                println!("WTF");
                None
            }
        }
    }

    fn consume(&mut self, ty: TokenType) {
        if self.check(&ty) {
            self.advance();
        } else {
            println!("WTF")
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.at_end() {
            if self.previous().typ == TokenType::Eof {
                return;
            }

            match self.peek().typ {
                TokenType::Class
                | TokenType::Fn
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {}
            };

            self.advance();
        }
    }

    fn one_of(&mut self, tys: &[TokenType]) -> bool {
        for ty in tys.iter() {
            if self.check(ty) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, ty: &TokenType) -> bool {
        if self.at_end() {
            return false;
        }

        return &self.peek().typ == ty;
    }

    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1
        }
        self.previous()
    }

    fn at_end(&self) -> bool {
        self.peek().typ == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).cloned().unwrap()
    }
}
