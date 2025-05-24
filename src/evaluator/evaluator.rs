use std::collections::HashMap;

use crate::parser::ast::{ASTNode, ASTNodeTrait};
use crate::parser::parser::Parser;
use crate::constants::token::Token;

#[derive(Clone, Debug)]
pub enum Value {
    Number(i64),
    BuiltInFunction(fn(Vec<Value>) -> Value),
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
                        Value::Number(n) => println!("{}", n),
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
                let l = self.eval(left)?;
                let r = self.eval(right)?;
                match op.as_str() {
                    "+" => match (l, r) {
                        (Value::Number(lv), Value::Number(rv)) => Ok(Value::Number(lv + rv)),
                        _ => Err("Invalid types for addition".into()),
                    },
                    "-" => match (l, r) {
                        (Value::Number(lv), Value::Number(rv)) => Ok(Value::Number(lv - rv)),
                        _ => Err("Invalid types for subtraction".into()),
                    },
                    "*" => match (l, r) {
                        (Value::Number(lv), Value::Number(rv)) => Ok(Value::Number(lv * rv)),
                        _ => Err("Invalid types for multiplication".into()),
                    },
                    "/" => match (l, r) {
                        (Value::Number(lv), Value::Number(rv)) => {
                            if rv == 0 {
                                Err("Division by zero".into())
                            } else {
                                Ok(Value::Number(lv / rv))
                            }
                        }
                        _ => Err("Invalid types for division".into()),
                    },
                    _ => Err(format!("Unknown operator: {}", op)),
                }
            }

            ASTNode::NumberLiteral(n) => Ok(Value::Number(*n)),

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
}
