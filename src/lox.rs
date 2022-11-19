pub mod expr;
mod interpreter;
pub mod parser;
pub mod scanner;
pub mod token;
pub mod token_type;

mod error;
pub use error::*;

use std::{
    fs::read_to_string,
    io::{self, Write},
    path::Path,
};

use crate::{interpreter::Interpreter, parser::Parser, scanner::Scanner};

#[derive(Debug)]
pub struct Lox;

impl Lox {
    pub fn run_file(f: impl AsRef<Path>) -> io::Result<()> {
        let fl = read_to_string(f.as_ref())?;
        Self::run(&fl);
        Ok(())
    }

    pub fn run_prompt() -> io::Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            print!("$ ");
            stdout.flush()?;

            let mut ln = String::new();
            match stdin.read_line(&mut ln) {
                Ok(_) => Self::run(&ln),
                Err(e) => return Err(e),
            };
        }
    }

    pub fn run(src: &str) {
        let mut scanner = Scanner::new(src);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        Interpreter::interpret(expr);
    }

    pub fn error(ln: usize, msg: &str) {
        Self::report(ln, "", msg)
    }

    pub fn report(ln: usize, loc: &str, msg: &str) {
        eprintln!("[line: {ln}] Error {loc}: {msg}")
    }
}
