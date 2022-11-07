use std::{env, io};

mod ast_printer;
use lox::Lox;

fn main() -> io::Result<()> {
    let mut args = env::args();

    args.next();

    match args.next() {
        Some(f) => Lox::run_file(f),
        None => Lox::run_prompt(),
    }
}
