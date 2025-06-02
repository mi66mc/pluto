mod constants;
mod lexer;
mod parser;
mod evaluator;
mod utils;
mod builtins;
mod repl;

use utils::args::get_args;
use utils::colors::{self};
use std::fs;
use repl::repl::repl;

fn main() {
    let args = get_args();

    if args.len() < 2 {
        repl();
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let tokens = lexer::tokenizer::tokenize(&contents);
    let mut parser = parser::parser::Parser::new(tokens, contents);
    
    match parser.parse() {
        Ok(ast) => {
            let mut evaluator = evaluator::evaluator::Evaluator::new();
            match evaluator.evaluate(&ast) {
                Ok(_) => (),
                Err(e) => {
                    println!("{}", colors::error(&e));
                    std::process::exit(1);
                }
            }
        },
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
