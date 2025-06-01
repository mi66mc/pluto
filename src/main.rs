mod constants;
mod lexer;
mod parser;
mod evaluator;
mod utils;
mod builtins;
mod repl;

use utils::args::get_args;
use utils::colors::{self, red, yellow};
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
    let mut parser = parser::parser::Parser::new(tokens);
    let parser_current = parser.current;
    
    match parser.parse() {
        Ok(_) => {
            let tokens = lexer::tokenizer::tokenize(&contents);
            let mut evaluator = evaluator::evaluator::Evaluator::new(tokens);
            match evaluator.evaluate() {
                Ok(_) => (),
                Err(e) => {
                    println!("{}", colors::error(&e));
                    std::process::exit(1);
                }
            }
        },
        Err(e) => {
            println!("{}", colors::error(&e));
            let tokens = lexer::tokenizer::tokenize(&contents);
            if let Some(token) = tokens.get(parser_current.saturating_sub(1)) {
                let line_number = contents[..token.position].matches('\n').count() + 1;
                let line_start = contents[..token.position].rfind('\n').map_or(0, |i| i + 1);
                let column = token.position - line_start + 1;
                
                println!("Error occurred at line {}:{}", line_number, column);
                let line = contents.lines().nth(line_number - 1).unwrap_or("");
                println!("  {} {}", yellow("|"), line);
                println!("  {} {}", yellow("|"), red(&format!("{}^", " ".repeat(column - 1))));
            }
            std::process::exit(1);
        }
    }
}
