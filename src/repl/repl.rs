use crate::lexer::tokenizer::tokenize;
use crate::parser::parser::Parser;
use crate::evaluator::evaluator::Evaluator;
use std::io::{self, Write};

pub fn repl() {
    println!("Welcome to the Pluto REPL!");
    println!("Type 'exit' to quit.");

    let base_env = Vec::new();
    let mut env = Evaluator::new(&base_env);

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read line");
            continue;
        }
        let input = input.trim();

        if input == "exit" {
            break;
        }
        if input.is_empty() {
            continue;
        }

        let tokens = tokenize(input);
        let mut parser = Parser::new(&tokens);

        match parser.parse() {
            Ok(ast) => match env.evaluate_ast(ast) {
                Ok(val) => {
                    let s = val.to_string();
                    if s != "0" && !s.is_empty() {
                        println!("{}", s);
                    }
                }
                Err(e) => println!("Runtime error: {}", e),
            },
            Err(e) => println!("Parse error: {}", e),
        }
    }
}