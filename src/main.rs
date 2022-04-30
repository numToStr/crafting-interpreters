mod gen_ast;
mod interpreter;
mod lexer;
mod lox;
mod parser;
mod printer;
mod token;
mod token_type;

use std::env;

use lox::Lox;

fn main() -> std::io::Result<()> {
    let mut args = env::args();

    args.next();

    match args.next() {
        Some(f) => Lox::run_file(f),
        None => Lox::run_prompt(),
    }
}
