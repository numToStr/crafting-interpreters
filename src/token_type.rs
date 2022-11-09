#[derive(Debug, PartialEq, Clone, Copy)]
#[non_exhaustive]
pub enum TokenType<'t> {
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
    Id(&'t str),
    String(&'t str),
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
