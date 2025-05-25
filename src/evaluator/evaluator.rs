use std::collections::HashMap;

use crate::parser::ast::{ASTNode, ASTNodeTrait};
use crate::parser::parser::Parser;
use crate::constants::token::Token;
use std::fmt;
use std::io::Write;

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Number(i64),
    Float(f64),
    BuiltInFunction(fn(Vec<Value>) -> Value),
    String(String),
    Module(HashMap<String, Value>)
    // add more
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "{}", s),
            Value::BuiltInFunction(_) => write!(f, "<built-in function>"),
            Value::Module(_) => write!(f, "<module>"),
        }
    }
}

pub struct Evaluator<'a> {
    parser: Parser<'a>,
    env: HashMap<String, Value>,
}

impl<'a> Evaluator<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        let mut env: HashMap<String, Value> = HashMap::new();

        // -----------------------------------------------------
        // ------------------   BUILTINS   ---------------------
        // -----------------------------------------------------

        let mut math = HashMap::new();

        math.insert("pi".to_string(), Value::Float(std::f64::consts::PI));

        math.insert("pow".to_string(), Value::BuiltInFunction(|args| {
            let a = match args.get(0) {
                Some(Value::Float(f)) => *f,
                Some(Value::Number(n)) => *n as f64,
                _ => return Value::Float(0.0),
            };
            let b = match args.get(1) {
                Some(Value::Float(f)) => *f,
                Some(Value::Number(n)) => *n as f64,
                _ => return Value::Float(0.0),
            };
            Value::Float(a.powf(b))
        }));

        env.insert("Math".to_string(), Value::Module(math));
        
        // -----------------------------------------------------
        // ------------------   GENERAL ------------------------
        // -----------------------------------------------------

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

        env.insert(
            "type".to_string(),
            Value::BuiltInFunction(|args| {
                if let Some(arg) = args.get(0) {
                    match arg {
                        Value::Bool(_) => Value::String("Bool".to_string()),
                        Value::Number(_) => Value::String("Number".to_string()),
                        Value::Float(_) => Value::String("Float".to_string()),
                        Value::String(_) => Value::String("String".to_string()),
                        _ => Value::String("UNKNOWN".to_string()),
                    }
                } else {
                    Value::String("UNKNOWN".to_string())
                }
            }),
        );

        env.insert(
            "input".to_string(), 
            Value::BuiltInFunction(|args| {
                let r;
                if let Some(Value::String(prompt)) = args.get(0) {
                    let mut input = String::new();
                    print!("{}", prompt);
                    let _ = std::io::stdout().flush();
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                
                    r = input.trim().to_string();
                } else {
                    let mut input = String::new();
                    let _ = std::io::stdout().flush();
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                
                    r = input.trim().to_string();
                }
                Value::String(r)
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

            ASTNode::VariableDeclaration(name, maybe_expr) => {
                let val = if let Some(expr) = maybe_expr {
                    self.eval(expr)?
                } else {
                    Value::Number(0)
                };
                self.env.insert(name.clone(), val.clone());
                Ok(val)
            }

            ASTNode::Assignment(name, expr) => {
                let new_val = self.eval(expr)?;
                if let Some(val) = self.env.get_mut(name) {
                    *val = new_val.clone();
                    Ok(new_val)
                } else {
                    Err(format!("Undefined variable '{}'", name))
                }
            }

            ASTNode::BinaryExpression(left, op, right) => {
                let left_val = self.eval(left)?;
                let right_val = self.eval(right)?;
                self.eval_binary(left_val, op, right_val)
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

            ASTNode::MethodCall(obj, method, args) => {
                let obj_val = self.eval(obj)?;
                match obj_val {
                    Value::Module(ref map) => {
                        if let Some(Value::BuiltInFunction(f)) = map.get(method) {
                            let arg_vals = args.iter().map(|a| self.eval(a)).collect::<Result<Vec<_>, _>>()?;
                            return Ok(f(arg_vals));
                        }
                    }
                    Value::String(ref s) => {
                        match method.as_str() {
                            "len" => return Ok(Value::Number(s.len() as i64)),
                            "to_upper" => return Ok(Value::String(s.to_uppercase())),
                            "to_lower" => return Ok(Value::String(s.to_lowercase())),
                            "char_at" => {
                                if let Some(arg) = args.get(0) {
                                    if let Value::Number(index) = self.eval(arg)? {
                                        if index >= 0 && (index as usize) < s.len() {
                                            return Ok(Value::String(s.chars().nth(index as usize).unwrap().to_string()));
                                        }
                                    }
                                }
                                return Err("Index out of bounds".into());
                            },
                            "to_int" => {
                                if let Ok(num) = s.parse::<i64>() {
                                    return Ok(Value::Number(num));
                                } else {
                                    return Err(format!("Cannot convert '{}' to int", s));
                                }
                            },
                            "to_float" => {
                                if let Ok(num) = s.parse::<f64>() {
                                    return Ok(Value::Float(num));
                                } else {
                                    return Err(format!("Cannot convert '{}' to float", s));
                                }
                            },
                            _ => {}
                        }
                    }
                    Value::Number(_) | Value::Float(_) => {
                        match method.as_str() {
                            "to_string" => return Ok(Value::String(obj_val.to_string())),
                            "type" => return Ok(Value::String("Number".to_string())),
                            _ => {}
                        }
                    }
                    _ => {}
                }
                Err(format!("No such method '{}' for '{}'", method, obj.to_string()))
            }

            ASTNode::MemberAccess(object, member) => {
                let obj_val = self.eval(object)?;
                if let Value::Module(ref map) = obj_val {
                    if let Some(val) = map.get(member) {
                        return Ok(val.clone());
                    }
                }
                Err(format!("No such member '{}' for '{}'", member, object.to_string()))
            }

            ASTNode::BooleanLiteral(b) => Ok(Value::Bool(*b)),

            ASTNode::IfStatement(condition, then_branch, else_branch) => {
                let cond_val = self.eval(condition)?;
                match cond_val {
                    Value::Bool(true) => self.eval(then_branch),
                    Value::Bool(false) => {
                        if let Some(else_b) = else_branch {
                            self.eval(else_b)
                        } else {
                            Ok(Value::Number(0))
                        }
                    }
                    _ => Err("Condition in 'if' must be a boolean".to_string()),
                }
            }

            ASTNode::UnaryExpression(op, expr) => {
                let val = self.eval(expr)?;
                match (op.as_str(), val) {
                    ("!", Value::Bool(b)) => Ok(Value::Bool(!b)),
                    _ => Err("Unsupported unary operation".to_string()),
                }
            }

            _ => Err("Unsupported AST node".into()),
        }
    }

        fn eval_binary(&self, left: Value, op: &str, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => match op {
                "+" => Ok(Value::Number(a + b)),
                "-" => Ok(Value::Number(a - b)),
                "*" => Ok(Value::Number(a * b)),
                "/" => Ok(Value::Number(a / b)),
                "%" => Ok(Value::Number(a % b)),
                "==" => Ok(Value::Bool(a == b)),
                "!=" => Ok(Value::Bool(a != b)),
                "<" => Ok(Value::Bool(a < b)),
                ">" => Ok(Value::Bool(a > b)),
                "<=" => Ok(Value::Bool(a <= b)),
                ">=" => Ok(Value::Bool(a >= b)),
                _ => Err(format!("Unknown number operator: {}", op)),
            },
            (Value::Float(a), Value::Float(b)) => match op {
                "+" => Ok(Value::Float(a + b)),
                "-" => Ok(Value::Float(a - b)),
                "*" => Ok(Value::Float(a * b)),
                "/" => Ok(Value::Float(a / b)),
                "%" => Ok(Value::Float(a % b)),
                "==" => Ok(Value::Bool(a == b)),
                "!=" => Ok(Value::Bool(a != b)),
                "<" => Ok(Value::Bool(a < b)),
                ">" => Ok(Value::Bool(a > b)),
                "<=" => Ok(Value::Bool(a <= b)),
                ">=" => Ok(Value::Bool(a >= b)),
                _ => Err(format!("Unknown float operator: {}", op)),
            },
            (Value::String(a), Value::String(b)) => match op {
                "+" => Ok(Value::String(a + &b)),
                "==" => Ok(Value::Bool(a == b)),
                "!=" => Ok(Value::Bool(a != b)),
                _ => Err(format!("Unknown string operator: {}", op)),
            },
            (Value::Number(a), Value::Float(b)) => match op {
                "+" => Ok(Value::Float(a as f64 + b)),
                "-" => Ok(Value::Float(a as f64 - b)),
                "*" => Ok(Value::Float(a as f64 * b)),
                "/" => Ok(Value::Float(a as f64 / b)),
                "%" => Ok(Value::Float(a as f64 % b)),
                "==" => Ok(Value::Bool(a as f64 == b)),
                "!=" => Ok(Value::Bool(a as f64 != b)),
                "<" => Ok(Value::Bool((a as f64) < b)),
                ">" => Ok(Value::Bool(a as f64 > b)),
                "<=" => Ok(Value::Bool(a as f64 <= b)),
                ">=" => Ok(Value::Bool(a as f64 >= b)),
                _ => Err(format!("Unknown mixed operator: {}", op)),
            },
            (Value::Float(a), Value::Number(b)) => match op {
                "+" => Ok(Value::Float(a + b as f64)),
                "-" => Ok(Value::Float(a - b as f64)),
                "*" => Ok(Value::Float(a * b as f64)),
                "/" => Ok(Value::Float(a / b as f64)),
                "%" => Ok(Value::Float(a % b as f64)),
                "==" => Ok(Value::Bool(a == b as f64)),
                "!=" => Ok(Value::Bool(a != b as f64)),
                "<" => Ok(Value::Bool(a < b as f64)),
                ">" => Ok(Value::Bool(a > b as f64)),
                "<=" => Ok(Value::Bool(a <= b as f64)),
                ">=" => Ok(Value::Bool(a >= b as f64)),
                _ => Err(format!("Unknown mixed operator: {}", op)),
            },
            (Value::String(a), Value::Number(b)) => match op {
                "+" => Ok(Value::String(a + &b.to_string())),
                _ => Err(format!("Unknown string-number operator: {}", op)),
            },
            (Value::String(a), Value::Float(b)) => match op {
                "+" => Ok(Value::String(a + &b.to_string())),
                _ => Err(format!("Unknown string-float operator: {}", op)),
            },
            (Value::Bool(a), Value::Bool(b)) => match op {
                "&&" => Ok(Value::Bool(a && b)),
                "||" => Ok(Value::Bool(a || b)),
                "==" => Ok(Value::Bool(a == b)),
                "!=" => Ok(Value::Bool(a != b)),
                _ => Err(format!("Unknown boolean operator: {}", op)),
            },
            _ => Err("Type error: incompatible types for binary operation".to_string()),
        }
    }
}
