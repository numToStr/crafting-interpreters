use crate::{
    expr::{binary::Binary, grouping::Grouping, literal::Literal, unary::Unary, Expr},
    token::Token,
    token_type::TokenType,
    Lox,
};

pub struct Parser<'p> {
    tokens: &'p Vec<Token<'p>>,
    current: usize,
}

impl<'p> Parser<'p> {
    pub fn new(tokens: &'p Vec<Token<'p>>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expr<'p> {
        self.expression().unwrap()
    }

    fn expression(&mut self) -> Result<Expr<'p>, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr<'p>, ParseError> {
        let mut expr = self.comparision()?;
        while self.one_of(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparision()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), op, Box::new(right)))
        }
        Ok(expr)
    }

    fn comparision(&mut self) -> Result<Expr<'p>, ParseError> {
        let mut expr = self.term()?;
        while self.one_of(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), op, Box::new(right)))
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr<'p>, ParseError> {
        let mut expr = self.factor()?;
        while self.one_of(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), op, Box::new(right)))
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr<'p>, ParseError> {
        let mut expr = self.unary()?;
        while self.one_of(&[TokenType::Slash, TokenType::Star]) {
            let op = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), op, Box::new(right)))
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr<'p>, ParseError> {
        if self.one_of(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(Unary::new(op, Box::new(right))));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr<'p>, ParseError> {
        match self.peek().ty {
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Exepect ')' after expression!")?;
                Ok(Expr::Grouping(Grouping::new(Box::new(expr))))
            }
            x @ (TokenType::True
            | TokenType::False
            | TokenType::Nil
            | TokenType::Number(_)
            | TokenType::String(_)) => {
                self.advance();
                Ok(Expr::Literal(Literal::new(x)))
            }
            _ => {
                self.error(self.peek(), "Expect expression!")?;
                Err(ParseError::Missing)
            }
        }
    }

    fn consume(&mut self, t: TokenType, msg: &str) -> Result<Token<'p>, ParseError> {
        if self.check(&t) {
            return Ok(self.advance());
        }
        self.error(self.peek(), msg)
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

    fn check(&self, t: &TokenType) -> bool {
        if self.at_end() {
            return false;
        }
        &self.peek().ty == t
    }

    fn advance(&mut self) -> Token<'p> {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn at_end(&self) -> bool {
        self.peek().ty == TokenType::Eof
    }

    fn peek(&self) -> &Token<'p> {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> Token<'p> {
        self.tokens.get(self.current - 1).unwrap().to_owned()
    }

    fn error(&self, t: &Token<'p>, msg: &str) -> Result<Token<'p>, ParseError> {
        if t.ty == TokenType::Eof {
            Lox::report(t.ln, "at end", msg);
        } else {
            Lox::report(t.ln, &format!("at '{}'", t.lexeme), msg)
        }

        Err(ParseError::Missing)
    }

    fn synchronize(&mut self) -> Option<()> {
        self.advance();
        while !self.at_end() {
            if self.previous().ty == TokenType::Semicolon {
                return None;
            }

            match self.peek().ty {
                TokenType::Class
                | TokenType::For
                | TokenType::Fn
                | TokenType::If
                | TokenType::Print
                | TokenType::Return
                | TokenType::Var
                | TokenType::While => return None,
                _ => {}
            }
        }
        self.advance();
        Some(())
    }
}

#[derive(Debug)]
enum ParseError {
    Missing,
}
