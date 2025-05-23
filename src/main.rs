use parser::ast;

mod constants;
mod lexer;
mod parser;
mod evaluator;

fn main() {
    // Example usage of the lexer
    let input = "let x = 5 + 5; let y = x * 2;";
    let tokens = lexer::tokenizer::tokenize(input);
    for token in &tokens {
        println!("{:?}", token);
    }

    // Example usage of the parser
    let mut ast = parser::parser::Parser::new(&tokens);
    let ast = ast.parse().unwrap();
    println!("{:?}", ast);

    // Example usage of the evaluator
    let mut evaluator = evaluator::evaluator::Evaluator::new(&tokens);
    let result = evaluator.evaluate().unwrap();
    println!("Result: {:?}", result);
}
