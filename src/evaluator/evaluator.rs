use std::collections::HashMap;
use std::fmt;

use crate::builtins::builtins::{default_env, float_methods, number_methods, string_methods, array_methods};
use crate::constants::token::Token;
use crate::parser::ast::{ASTNode, ASTNodeTrait};
use crate::parser::parser::Parser;

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Number(i64),
    Float(f64),
    BuiltInFunction(fn(Vec<Value>) -> Value),
    String(String),
    Module(HashMap<String, Value>),
    Array(Vec<Value>),
    // add more
}

pub trait PlutoMethod {
    fn call_method(&self, method: &str, args: Vec<Value>) -> Result<Value, String>;
}

impl PlutoMethod for Value {
    fn call_method(&self, method: &str, args: Vec<Value>) -> Result<Value, String> {
        match self {
            Value::Module(map) => {
                if let Some(Value::BuiltInFunction(f)) = map.get(method) {
                    return Ok(f(args));
                } else {
                    return Err(format!("No such method '{}' in module", method));
                }
            }
            Value::String(_) => {
                if let Some(f) = string_methods().get(method) {
                    f(self, args)
                } else {
                    Err(format!("No such method '{}' for String", method))
                }
            }
            Value::Number(_) => {
                if let Some(f) = number_methods().get(method) {
                    f(self, args)
                } else {
                    Err(format!("No such method '{}' for Number", method))
                }
            }
            Value::Float(_) => {
                if let Some(f) = float_methods().get(method) {
                    f(self, args)
                } else {
                    Err(format!("No such method '{}' for Float", method))
                }
            }
            Value::Array(_) => {
                if let Some(f) = array_methods().get(method) {
                    f(self, args)
                } else {
                    Err(format!("No such method '{}' for Array", method))
                }
            }
            _ => Err(format!("No such method '{}' for this type", method)),
        }
    }
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
            Value::Array(arr) => write!(f, "[{}]", arr.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")),
        }
    }
}

pub struct Evaluator<'a> {
    parser: Parser<'a>,
    pub env_stack: Vec<HashMap<String, Value>>,
}

impl<'a> Evaluator<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Evaluator {
            parser: Parser::new(tokens),
            env_stack: vec![default_env()], // default
        }
    }

    fn current_env_mut(&mut self) -> &mut HashMap<String, Value> {
        self.env_stack.last_mut().unwrap()
    }

    // fn current_env(&self) -> &HashMap<String, Value> {
    //     self.env_stack.last().unwrap()
    // }

    fn lookup(&self, name: &str) -> Option<Value> {
        for env in self.env_stack.iter().rev() {
            if let Some(val) = env.get(name) {
                return Some(val.clone());
            }
        }
        None
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

            ASTNode::Block(statements) => {
                self.env_stack.push(HashMap::new());
                let mut last = Value::Number(0);
                for stmt in statements {
                    last = self.eval(stmt)?;
                }
                self.env_stack.pop();
                Ok(last)
            }

            ASTNode::VariableDeclaration(name, maybe_expr) => {
                let val = if let Some(expr) = maybe_expr {
                    self.eval(expr)?
                } else {
                    Value::Number(0)
                };
                self.current_env_mut().insert(name.clone(), val.clone());
                Ok(val)
            }

            ASTNode::Assignment(name, expr) => {
                let new_val = self.eval(expr)?;
                for env in self.env_stack.iter_mut().rev() {
                    if let Some(val) = env.get_mut(name) {
                        *val = new_val.clone();
                        return Ok(new_val);
                    }
                }
                Err(format!("Undefined variable '{}'", name))
            }

            ASTNode::BinaryExpression(left, op, right) => {
                let left_val = self.eval(left)?;
                let right_val = self.eval(right)?;
                self.eval_binary(left_val, op, right_val)
            }

            ASTNode::NumberLiteral(n) => Ok(Value::Number(*n)),

            ASTNode::FloatLiteral(f) => Ok(Value::Float(*f)),

            ASTNode::StringLiteral(s) => Ok(Value::String(s.clone())),

            ASTNode::Identifier(name) => {
                self.lookup(name)
                    .ok_or_else(|| format!("Undefined variable '{}'", name))
            }

            ASTNode::FunctionCall(name, args) => {
                if let Some(val) = self.lookup(name) {
                    match val {
                        Value::BuiltInFunction(f) => {
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
                let arg_vals = args
                    .iter()
                    .map(|a| self.eval(a))
                    .collect::<Result<Vec<_>, _>>()?;
                match obj_val.call_method(method, arg_vals) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(format!(
                        "No such method '{}' for '{}': {}",
                        method,
                        obj.to_string(),
                        e
                    )),
                }
            }

            ASTNode::MemberAccess(object, member) => {
                let obj_val = self.eval(object)?;
                if let Value::Module(ref map) = obj_val {
                    if let Some(val) = map.get(member) {
                        return Ok(val.clone());
                    }
                }
                Err(format!(
                    "No such member '{}' for '{}'",
                    member,
                    object.to_string()
                ))
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

            ASTNode::ArrayLiteral(elements) => {
                let mut vals = Vec::new();
                for el in elements {
                    vals.push(self.eval(el)?);
                }
                Ok(Value::Array(vals))
            }

            ASTNode::IndexAccess(array_expr, index_expr) => {
                let array_val = self.eval(array_expr)?;
                let index_val = self.eval(index_expr)?;
                match (array_val, index_val) {
                    (Value::Array(arr), Value::Number(idx)) => {
                        let idx = idx as usize;
                        arr.get(idx)
                            .cloned()
                            .ok_or_else(|| "Array index out of bounds".to_string())
                    }
                    _ => Err("Indexing only supported for arrays with integer indices".to_string()),
                }
            }

            ASTNode::AssignmentIndex(array_expr, index_expr, value_expr) => {
                if let ASTNode::Identifier(var_name) = &**array_expr {
                    let index_val = self.eval(index_expr)?;
                    let value_val = self.eval(value_expr)?;
                    if let Some(val) = self.current_env_mut().get_mut(var_name) {
                        if let Value::Array(arr) = val {
                            if let Value::Number(idx) = index_val {
                                let idx = idx as usize;
                                if idx < arr.len() {
                                    arr[idx] = value_val.clone();
                                    return Ok(Value::Array(arr.clone()));
                                } else {
                                    return Err("Array index out of bounds".to_string());
                                }
                            } else {
                                return Err("Assignment only supported for arrays with integer indices".to_string());
                            }
                        } else {
                            return Err(format!("'{}' is not an array", var_name));
                        }
                    } else {
                        return Err(format!("'{}' is not an array", var_name));
                    }
                }
                let mut array_val = self.eval(array_expr)?;
                let index_val = self.eval(index_expr)?;
                let value_val = self.eval(value_expr)?;
                if let Value::Array(ref mut arr) = array_val {
                    if let Value::Number(idx) = index_val {
                        let idx = idx as usize;
                        if idx < arr.len() {
                            arr[idx] = value_val.clone();
                            Ok(Value::Array(arr.clone()))
                        } else {
                            Err("Array index out of bounds".to_string())
                        }
                    } else {
                        Err("Assignment only supported for arrays with integer indices".to_string())
                    }
                } else {
                    Err("Assignment only supported for arrays with integer indices".to_string())
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
                "==" => Ok(Value::Bool((a as f64) == b)),
                "!=" => Ok(Value::Bool((a as f64) != b)),
                "<" => Ok(Value::Bool((a as f64) < b)),
                ">" => Ok(Value::Bool((a as f64) > b)),
                "<=" => Ok(Value::Bool((a as f64) <= b)),
                ">=" => Ok(Value::Bool((a as f64) >= b)),
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
            (Value::Number(a), Value::String(b)) => match op {
                "+" => Ok(Value::String(a.to_string() + &b)),
                _ => Err(format!("Unknown number-string operator: {}", op)),
            },
            (Value::String(a), Value::Float(b)) => match op {
                "+" => Ok(Value::String(a + &b.to_string())),
                _ => Err(format!("Unknown string-float operator: {}", op)),
            },
            (Value::Float(a), Value::String(b)) => match op {
                "+" => Ok(Value::String(a.to_string() + &b)),
                _ => Err(format!("Unknown float-string operator: {}", op)),
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
