#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // SINGLE-CHARACTER TOKENS.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // ONE OR TWO CHARACTER TOKENS.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // LITERALS.
    Id(String),
    String(String),
    Number(f64),

    // KEYWORDS.
    And,
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
