use crate::{token::Token, token_type::TokenType};

macro_rules! gen_ast {
    ($($name:ident, $ty: ident { $($f: ident : $ff: ty),* },)*) => {
        #[derive(Debug)]
        pub enum Expr {
            $($ty($ty),)*
        }

        impl Expr {
            pub fn accept<T>(self, visitor: impl Visitor<T>) -> Result<T, ()> {
                match self {
                    $(Self::$ty(x)  => x.accept(visitor),)*
                }
            }
        }

        pub trait Visitor<T> {
            $(
                fn $name(&self, expr: $ty) -> Result<T, ()>;
            )*
        }

        $(
            #[derive(Debug)]
            pub struct $ty {
                $(pub $f: $ff),*
            }

            impl $ty {
                pub fn accept<T>(self, visitor: impl Visitor<T>) -> Result<T, ()> {
                    visitor.$name(self)
                }
            }
        )*
    };
}

gen_ast!(
    visit_binary, Binary { left: Box<Expr>, op: Token, right: Box<Expr> },
    visit_grouping, Grouping { expr: Box<Expr> },
    visit_literal, Literal { value: TokenType },
    visit_unary, Unary { op: Token, right: Box<Expr> },
);
