use std::collections::HashMap;

use crate::parser::ast::{ASTNode, ASTNodeTrait};
use crate::parser::parser::Parser;
use crate::constants::token::Token;

#[derive(Clone, Debug)]
pub enum Value {
    Number(i64),
    Float(f64),
    BuiltInFunction(fn(Vec<Value>) -> Value),
    String(String),
    // add more
}

pub struct Evaluator<'a> {
    parser: Parser<'a>,
    env: HashMap<String, Value>,
}

impl<'a> Evaluator<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        let mut env: HashMap<String, Value> = HashMap::new();
        env.insert(
            "print".to_string(),
            Value::BuiltInFunction(|args| {
                for arg in args {
                    match arg {
                        Value::Number(n)    => println!("{}", n),
                        Value::Float(f)     => println!("{}", f),
                        Value::String(s) => println!("{}", s),
                        _ => println!("{:?}", arg),
                    }
                }
                Value::Number(0)
            }),
        );
        Evaluator {
            parser: Parser::new(tokens),
            env: env,
        }
    }

    pub fn evaluate(&mut self) -> Result<Value, String> {
        let ast = self.parser.parse()?;
        self.eval(&ast)
    }

    // -----------------------------------------------------
    // ------------------   CORE EVAL    -------------------
    // -----------------------------------------------------

    fn eval(&mut self, node: &ASTNode) -> Result<Value, String> {
        match node {
            ASTNode::Program(statements) => {
                let mut last = Value::Number(0);
                for stmt in statements {
                    last = self.eval(stmt)?;
                }
                Ok(last)
            }

            ASTNode::VariableDeclaration(name, Some(expr)) => {
                let val = self.eval(expr)?;
                self.env.insert(name.clone(), val.clone());
                Ok(val)
            }

            ASTNode::VariableDeclaration(_, None) => Ok(Value::Number(0)),

            ASTNode::BinaryExpression(left, op, right) => {
                let left_val = self.eval(left)?;
                let right_val = self.eval(right)?;
                Ok(self.eval_binary(left_val, op, right_val))
            }

            ASTNode::NumberLiteral(n) => Ok(Value::Number(*n)),

            ASTNode::FloatLiteral(f) => Ok(Value::Float(*f)),

            ASTNode::StringLiteral(s) => {
                Ok(Value::String(s.clone()))
            }

            ASTNode::Identifier(name) => {
                self.env
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("Undefined variable '{}'", name))
            }

            ASTNode::FunctionCall(name, args) => {
                if let Some(val) = self.env.get(name) {
                    match val {
                        Value::BuiltInFunction(f) => {
                            let f = *f;
                            let mut arg_values = Vec::new();
                            for arg in args {
                                arg_values.push(self.eval(arg)?);
                            }
                            let result = f(arg_values);
                            Ok(result)
                        }
                        _ => Err(format!("'{}' is not a function", name)),
                    }
                } else {
                    Err(format!("Unknown function '{}'", name))
                }
            }

            _ => Err("Unsupported AST node".into()),
        }
    }

    fn eval_binary(&self, left: Value, op: &str, right: Value) -> Value {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => match op {
                "+" => Value::Number(a + b),
                "-" => Value::Number(a - b),
                "*" => Value::Number(a * b),
                _ => panic!("Unknown operator"),
            },
            (Value::Number(a), Value::Float(b)) => match op {
                "+" => Value::Float(a as f64 + b),
                "-" => Value::Float(a as f64 - b),
                "*" => Value::Float(a as f64 * b),
                "/" => Value::Float(a as f64 / b),
                _ => panic!("Unknown operator"),
            },
            (Value::Float(a), Value::Number(b)) => match op {
                "+" => Value::Float(a + b as f64),
                "-" => Value::Float(a - b as f64),
                "*" => Value::Float(a * b as f64),
                "/" => Value::Float(a / b as f64),
                _ => panic!("Unknown operator"),
            },
            (Value::Float(a), Value::Float(b)) => match op {
                "+" => Value::Float(a + b),
                "-" => Value::Float(a - b),
                "*" => Value::Float(a * b),
                "/" => Value::Float(a / b),
                _ => panic!("Unknown operator"),
            },
            (Value::String(a), Value::String(b)) => match op {
                "+" => Value::String(a + &b),
                _ => panic!("Unknown operator"),
            },
            (Value::String(a), Value::Number(b)) => match op {
                "+" => Value::String(a + &b.to_string()),
                _ => panic!("Unknown operator"),
            },
            (Value::String(a), Value::Float(b)) => match op {
                "+" => Value::String(a + &b.to_string()),
                _ => panic!("Unknown operator"),
            }
            _ => panic!("Type error"),
        }
    }
}
