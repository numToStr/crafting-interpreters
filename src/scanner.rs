use crate::{token::Token, token_type::TokenType, Lox};

#[derive(Debug)]
pub struct Scanner<'s> {
    src: &'s str,
    tokens: Vec<Token<'s>>,
    start: usize,
    current: usize,
    ln: usize,
}

impl<'s> Scanner<'s> {
    pub fn new(src: &'s str) -> Self {
        Self {
            src,
            tokens: vec![],
            start: 0,
            current: 0,
            ln: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token<'s>> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens.push(Token::new(TokenType::Eof, "", self.ln));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let t = if self.metch('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(t)
            }
            '=' => {
                let t = if self.metch('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(t)
            }
            '<' => {
                let t = if self.metch('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(t)
            }
            '>' => {
                let t = if self.metch('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(t)
            }
            '/' => {
                if self.metch('/') {
                    while self.peek() != '\n' && !self.is_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.ln += 1,
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if c.is_alphabetic() => self.ident(),
            _ => Lox::error(self.ln, "Unexpected Character!"),
        };
    }

    fn ident(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let id = self.src.get(self.start..self.current).unwrap();
        let token = match id {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fn,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            x => TokenType::Id(x),
        };
        self.add_token(token);
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume '.'
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let n: f64 = self
            .src
            .get(self.start..self.current)
            .unwrap()
            .parse()
            .unwrap();
        self.add_token(TokenType::Number(n))
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_end() {
            if self.peek() == '\n' {
                self.ln += 1;
            }
            self.advance();
        }

        if self.is_end() {
            return Lox::error(self.ln, "Unterminated String!");
        }

        self.advance(); // Closing "

        // Trim the surrounding qutoes
        let s = self.src.get(self.start + 1..self.current - 1).unwrap();
        self.add_token(TokenType::String(s))
    }

    fn metch(&mut self, c: char) -> bool {
        if self.is_end() || self.src.chars().nth(self.current).unwrap() != c {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_end() {
            return '\0';
        }

        self.src.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.src.len() {
            return '\0';
        }

        self.src.chars().nth(self.current + 1).unwrap()
    }

    fn is_end(&self) -> bool {
        self.current == self.src.len()
    }

    fn advance(&mut self) -> char {
        let c = self.src.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, ty: TokenType<'s>) {
        if let Some(lexeme) = self.src.get(self.start..self.current) {
            self.tokens.push(Token::new(ty, lexeme, self.ln))
        }
    }
}
