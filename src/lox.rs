use std::{
    fs::read_to_string,
    io::{self, BufRead, Write},
};

use crate::lexer::Lexer;

pub struct Lox;

impl Lox {
    pub fn run_file(f: String) -> io::Result<()> {
        let src = read_to_string(f)?;

        Self::run(&src).unwrap();

        Ok(())
    }

    pub fn run_prompt() -> io::Result<()> {
        let sti = io::stdin();
        let mut sto = io::stdout();

        loop {
            print!("> ");
            sto.flush()?;

            match sti.lock().lines().next() {
                Some(Ok(s)) => Self::run(&s).ok(),
                _ => break,
            };
        }

        Ok(())
    }

    pub fn run(src: &str) -> Result<(), ()> {
        let tokens = Lexer::new(src).parse()?;
        for t in tokens {
            println!("{:?}", t)
        }
        Ok(())
    }

    pub fn error(line: usize, msg: String) {
        Self::report(line, "".into(), msg);
    }

    pub fn report(line: usize, whr: String, msg: String) {
        println!("[line {line}] Error {whr}: {msg}");
    }
}