mod constants;
mod lexer;
mod parser;
mod evaluator;
mod utils;
mod builtins;

use utils::args::get_args;
use std::fs;

fn main() {
    // Example usage of the lexer
    // let input = "let x = 5 + 5; let y = x * 2;";
    // let tokens = lexer::tokenizer::tokenize(input);
    // for token in &tokens {
    //     println!("{:#?}", token);
    // }

    // // Example usage of the parser
    // let mut ast = parser::parser::Parser::new(&tokens);
    // let ast = ast.parse().unwrap();
    // println!("{:#?}", ast);

    // // Example usage of the evaluator
    // let mut evaluator = evaluator::evaluator::Evaluator::new(&tokens);
    // let result = evaluator.evaluate().unwrap();
    // println!("Result: {:?}", result);

    let args = get_args();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let tokens = lexer::tokenizer::tokenize(&contents);
    for token in &tokens {
        println!("{:?}", token);
    }

    let mut evaluator = evaluator::evaluator::Evaluator::new(&tokens);
    evaluator.evaluate().unwrap();
}
