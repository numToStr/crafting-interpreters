use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Token<'t> {
    pub ty: TokenType<'t>,
    pub lexeme: &'t str,
    pub ln: usize,
    // literal: todo!(),
}

impl<'t> Token<'t> {
    pub fn new(ty: TokenType<'t>, lexeme: &'t str, ln: usize) -> Self {
        Self { ty, lexeme, ln }
    }
}
