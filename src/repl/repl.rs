use crate::lexer::tokenizer::tokenize;
use crate::parser::parser::Parser;
use crate::evaluator::evaluator::Evaluator;
use crate::utils::colors::{self, blue, green, red, yellow, bold};
use std::io::{self, Write};

const HELP_TEXT: &str = r#"
Available Commands:
  :help, :h     - Show this help message
  :clear, :c    - Clear the screen
  :exit, :q     - Exit the REPL
  :reset        - Reset the environment
"#;

fn print_welcome_message() {
    println!("\n{}", blue("=== Pluto Programming Language REPL ==="));
    println!("Type {} for available commands", green(":help"));
    println!("Type {} to quit\n", green(":exit"));
}

fn print_error(msg: &str, input: &str, position: Option<usize>) {
    println!("{}", colors::error(msg));
    
    if let Some(pos) = position {
        let line = input.lines().next().unwrap_or("");
        println!("  {}", yellow("|"));
        println!("  {} {}", yellow("|"), line);
        println!("  {} {}", yellow("|"), red(&format!("{}^", " ".repeat(pos))));
    }
}

enum Command {
    Help,
    Clear,
    Exit,
    Reset,
    Unknown,
}

fn parse_command(cmd: &str) -> Command {
    let cmd = cmd.trim();
    match cmd {
        ":help" | ":h" => Command::Help,
        ":clear" | ":c" => Command::Clear,
        ":exit" | ":q" => Command::Exit,
        ":reset" => Command::Reset,
        _ => Command::Unknown,
    }
}

fn handle_special_command(cmd: &str, env: &mut Evaluator) -> bool {
    match parse_command(cmd) {
        Command::Help => {
            println!("{}", blue(HELP_TEXT));
            true
        }
        Command::Clear => {
            print!("\x1B[2J\x1B[1;1H");
            print_welcome_message();
            true
        }
        Command::Exit => {
            println!("{}", green("Goodbye!"));
            std::process::exit(0);
        }
        Command::Reset => {
            let tokens = Vec::new();
            *env = Evaluator::new(tokens);
            println!("{}", green("Environment reset."));
            true
        }
        Command::Unknown => false
    }
}

pub fn repl() {
    print_welcome_message();

    let base_tokens = Vec::new();
    let mut env = Evaluator::new(base_tokens);
    let mut input_buffer = String::new();
    let mut brace_count = 0;
    let mut paren_count = 0;

    loop {
        let prompt = if input_buffer.is_empty() {
            format!("{} ", bold(">>"))
        } else {
            format!("{} ", bold(".."))
        };
        
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                let line_trimmed = line.trim_end().to_string();
                if line_trimmed.is_empty() && !input_buffer.is_empty() {
                    let input = input_buffer.clone();
                    input_buffer.clear();
                    brace_count = 0;
                    paren_count = 0;
                    
                    if !handle_special_command(&input, &mut env) {
                        let tokens = tokenize(&input);
                        let mut parser = Parser::new(tokens);
                        
                        match parser.parse() {
                            Ok(ast) => match env.evaluate_ast(ast) {
                                Ok(val) => {
                                    let s = val.to_string();
                                    if s != "0" && !s.is_empty() {
                                        println!("{}", blue(&s));
                                    }
                                }
                                Err(e) => print_error(&e, &input, None),
                            },
                            Err(e) => print_error(&e, &input, Some(parser.current)),
                        }
                    }
                } else if line_trimmed.starts_with(':') {
                    if !handle_special_command(&line_trimmed, &mut env) {
                        println!("{}", red("Unknown command. Type :help for available commands."));
                    }
                } else {
                    for c in line_trimmed.chars() {
                        match c {
                            '{' => brace_count += 1,
                            '}' => brace_count -= 1,
                            '(' => paren_count += 1,
                            ')' => paren_count -= 1,
                            _ => {}
                        }
                    }

                    if !input_buffer.is_empty() {
                        input_buffer.push('\n');
                    }
                    input_buffer.push_str(&line_trimmed);

                    if brace_count == 0 && paren_count == 0 && !line_trimmed.ends_with('\\') {
                        let input = input_buffer.clone();
                        input_buffer.clear();
                        
                        if !handle_special_command(&input, &mut env) {
                            let tokens = tokenize(&input);
                            let mut parser = Parser::new(tokens);
                            
                            match parser.parse() {
                                Ok(ast) => match env.evaluate_ast(ast) {
                                    Ok(val) => {
                                        let s = val.to_string();
                                        if s != "0" && !s.is_empty() {
                                            println!("{}", blue(&s));
                                        }
                                    }
                                    Err(e) => print_error(&e, &input, None),
                                },
                                Err(e) => print_error(&e, &input, Some(parser.current)),
                            }
                        }
                    }
                }
            }
            Err(error) => {
                println!("{}", colors::error(&error.to_string()));
                break;
            }
        }
    }
}