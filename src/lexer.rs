use crate::{lox::Lox, token::Token, token_type::TokenType};

#[derive(Debug)]
pub struct Lexer {
    start: usize,
    current: usize,
    line: usize,
    source: Vec<char>,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn parse(mut self) -> Result<Vec<Token>, ()> {
        while !self.at_end() {
            self.start = self.current;
            self.scan();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, None, self.line));

        Ok(self.tokens)
    }

    fn scan(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add(TokenType::LeftParen),
            ')' => self.add(TokenType::RightParen),
            '{' => self.add(TokenType::LeftBrace),
            '}' => self.add(TokenType::RightBrace),
            ',' => self.add(TokenType::Comma),
            '.' => self.add(TokenType::Dot),
            '-' => self.add(TokenType::Minus),
            '+' => self.add(TokenType::Plus),
            ';' => self.add(TokenType::Semicolon),
            '*' => self.add(TokenType::Star),
            '!' => {
                let tok = if self.is('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add(tok)
            }
            '=' => {
                let tok = if self.is('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add(tok)
            }
            '<' => {
                let tok = if self.is('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add(tok)
            }
            '>' => {
                let tok = if self.is('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add(tok)
            }
            '/' => {
                if self.is('/') {
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add(TokenType::Slash)
                }
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => {}
            x => {
                if x.is_alphabetic() {
                    self.identifier()
                } else {
                    Lox::report(self.line, x.into(), "Unexpected character!".into());
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let value = self
            .source
            .get(self.start..self.current)
            .unwrap()
            .iter()
            .collect::<String>();

        let tok = match value.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fn" => TokenType::Fn,
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
            _ => TokenType::Id(value),
        };

        self.add(tok);
    }

    fn number(&mut self) {
        loop {
            match self.peek() {
                '0'..='9' => {
                    self.advance();
                }
                '.' => {
                    if let '0'..='9' = self.peek_next() {
                        self.advance();
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        let value = self
            .source
            .get(self.start..self.current)
            .unwrap()
            .iter()
            .collect::<String>();

        self.add(TokenType::Number(value.parse().unwrap()))
    }

    fn string(&mut self) {
        if self.at_end() {
            Lox::error(self.line, "Unterminated string!".into());
            return;
        }

        while self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        // Closing "
        self.advance();

        let value = self
            .source
            .get(self.start + 1..self.current - 1)
            .unwrap()
            .iter()
            .collect::<String>();

        self.add(TokenType::String(value));
    }

    fn is(&mut self, c: char) -> bool {
        if self.at_end() {
            return false;
        }

        if self.next() == c {
            return true;
        }

        self.current += 1;

        false
    }

    fn peek(&mut self) -> char {
        if self.at_end() {
            return '\0';
        }

        self.next()
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.get(self.current + 1).copied().unwrap()
    }

    fn advance(&mut self) -> char {
        let cur = self.next();
        self.current += 1;
        cur
    }

    fn add(&mut self, ttype: TokenType) {
        let lexeme: String = self
            .source
            .get(self.start..self.current)
            .unwrap()
            .iter()
            .collect();

        self.tokens.push(Token::new(ttype, Some(lexeme), self.line))
    }

    fn at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    fn next(&self) -> char {
        self.source.get(self.current).copied().unwrap()
    }
}
