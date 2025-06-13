use std::collections::HashMap;
use std::fmt;

use crate::builtins::builtins::{default_env, float_methods, number_methods, string_methods, array_methods, hashmap_methods};
use crate::parser::ast::{ASTNode, ASTNodeTrait};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum EvalResult {
    Value(Value),
    Return(Value),
    Break,
    Continue,
}

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Number(i64),
    Float(f64),
    BuiltInFunction(fn(Vec<Value>) -> Value),
    UserFunction {
        params: Vec<(String, Option<Box<ASTNode>>)>,
        body: Box<ASTNode>,
        env: Vec<HashMap<String, (Value, bool)>>,
    },
    String(String),
    Module(HashMap<String, Value>),
    Array(Vec<Value>),
    HashMapV(HashMap<String, Value>),
    Null
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
            Value::HashMapV(_) => {
                if let Some(f) = hashmap_methods().get(method) {
                    f(self, args)
                } else {
                    Err(format!("No such method '{}' for HashMap", method))
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
            Value::Null => write!(f, "null"),
            Value::BuiltInFunction(_) => write!(f, "<built-in function>"),
            Value::Module(_) => write!(f, "<module>"),
            Value::UserFunction { params, body, env } => {
                let params_str = params.iter().map(|(name, _)| name.clone()).collect::<Vec<_>>().join(", ");
                write!(f, "<function: params=[{}], body={:?}, env_size={} >", params_str, body, env.len())
            }
            Value::Array(arr) => write!(f, "[{}]", arr.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")),
            Value::HashMapV(map) => {
                let pairs: Vec<String> = map.iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                write!(f, "{{{}}}", pairs.join(", "))
            }
        }
    }
}

pub struct Evaluator {
    pub env_stack: Vec<HashMap<String, (Value, bool)>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            env_stack: vec![default_env()], // default
        }
    }

    fn current_env_mut(&mut self) -> &mut HashMap<String, (Value, bool)> {
        self.env_stack.last_mut().unwrap()
    }

    fn lookup(&self, name: &str) -> Option<Value> {
        for env in self.env_stack.iter().rev() {
            if let Some((val, _)) = env.get(name) {
                return Some(val.clone());
            }
        }
        None
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.is_empty(),
            Value::Null => false,
            Value::HashMapV(map) => !map.is_empty(),
            _ => false
        }
    }

    pub fn evaluate(&mut self, ast: &ASTNode) -> Result<Value, String> {
        match self.eval(ast)? {
            EvalResult::Value(val) => Ok(val),
            EvalResult::Return(_) => Err("Unexpected 'return' outside of function".to_string()),
            EvalResult::Break => Err("Unexpected 'break' outside of loop".to_string()),
            EvalResult::Continue => Err("Unexpected 'continue' outside of loop".to_string()),
        }
    }

    pub fn evaluate_ast(&mut self, ast: ASTNode) -> Result<Value, String> {
        match self.eval(&ast)? {
            EvalResult::Value(val) => Ok(val),
            EvalResult::Return(val) => Ok(val),
            EvalResult::Break => Err("Unexpected 'break' outside of loop".to_string()),
            EvalResult::Continue => Err("Unexpected 'continue' outside of loop".to_string()),
        }
    }

    // -----------------------------------------------------
    // ------------------   CORE EVAL    -------------------
    // -----------------------------------------------------

    pub fn eval(&mut self, node: &ASTNode) -> Result<EvalResult, String> {
        match node {
            ASTNode::Program(statements) => {
                let mut last = Value::Null;
                for stmt in statements {
                    match self.eval(stmt)? {
                        EvalResult::Value(val) => last = val,
                        EvalResult::Return(val) => return Ok(EvalResult::Return(val)),
                        EvalResult::Break => return Ok(EvalResult::Break),
                        EvalResult::Continue => return Ok(EvalResult::Continue),
                    }
                }
                Ok(EvalResult::Value(last))
            }

            ASTNode::Block(statements) => {
                self.env_stack.push(HashMap::new());
                let mut last = Value::Null;
                
                for stmt in statements {
                    match self.eval(stmt)? {
                        EvalResult::Value(_) => last = Value::Null,
                        EvalResult::Return(val) => {
                            self.env_stack.pop();
                            return Ok(EvalResult::Return(val));
                        }
                        EvalResult::Break => {
                            self.env_stack.pop();
                            return Ok(EvalResult::Break);
                        }
                        EvalResult::Continue => {
                            self.env_stack.pop();
                            return Ok(EvalResult::Continue);
                        }
                    }
                }
                self.env_stack.pop();
                Ok(EvalResult::Value(last))
            }

            ASTNode::VariableDeclaration(name, maybe_expr) => {
                let val = if let Some(expr) = maybe_expr {
                    match self.eval(expr)? {
                        EvalResult::Value(v) => v,
                        EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                        EvalResult::Break => return Ok(EvalResult::Break),
                        EvalResult::Continue => return Ok(EvalResult::Continue),
                    }
                } else {
                    Value::Null
                };
                self.current_env_mut().insert(name.clone(), (val.clone(), false));
                Ok(EvalResult::Value(val))
            }

            ASTNode::ConstDeclaration(name, maybe_expr) => {
                if self.current_env_mut().contains_key(name) {
                    return Err(format!("Variable '{}' already declared", name));
                }
                let val = if let Some(expr) = maybe_expr {
                    match self.eval(expr)? {
                        EvalResult::Value(v) => v,
                        EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                        EvalResult::Break => return Ok(EvalResult::Break),
                        EvalResult::Continue => return Ok(EvalResult::Continue),
                    }
                } else {
                    Value::Null
                };
                self.current_env_mut().insert(name.clone(), (val.clone(), true));
                Ok(EvalResult::Value(val))
            }

            ASTNode::Assignment(name, expr) => {
                let value = match self.eval(expr)? {
                    EvalResult::Value(v) => v,
                    EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                    EvalResult::Break => return Ok(EvalResult::Break),
                    EvalResult::Continue => return Ok(EvalResult::Continue),
                };
                for env in self.env_stack.iter_mut().rev() {
                    if let Some((_, is_const)) = env.get(name) {
                        if *is_const {
                            return Err(format!("Cannot assign to constant '{}'", name));
                        }
                    }
                    if let Some((val, _)) = env.get_mut(name) {
                        *val = value.clone();
                        return Ok(EvalResult::Value(value));
                    }
                }
                Err(format!("Undefined variable '{}'", name))
            }

            ASTNode::BinaryExpression(left, op, right) => {
                let left_val = match self.eval(left)? {
                    EvalResult::Value(v) => v,
                    EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                    EvalResult::Break => return Ok(EvalResult::Break),
                    EvalResult::Continue => return Ok(EvalResult::Continue),
                };
                let right_val = match self.eval(right)? {
                    EvalResult::Value(v) => v,
                    EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                    EvalResult::Break => return Ok(EvalResult::Break),
                    EvalResult::Continue => return Ok(EvalResult::Continue),
                };
                Ok(EvalResult::Value(self.eval_binary(left_val, op, right_val)?))
            }

            ASTNode::NumberLiteral(n) => Ok(EvalResult::Value(Value::Number(*n))),

            ASTNode::FloatLiteral(f) => Ok(EvalResult::Value(Value::Float(*f))),

            ASTNode::StringLiteral(s) => Ok(EvalResult::Value(Value::String(s.clone()))),

            ASTNode::NullLiteral => Ok(EvalResult::Value(Value::Null)),

            ASTNode::Identifier(name) => {
                if let Some(val) = self.lookup(name) {
                    Ok(EvalResult::Value(val))
                } else {
                    Err(format!("Undefined variable '{}'", name))
                }
            }
            
            ASTNode::AnonymousFunction(params, body) => {
                Ok(EvalResult::Value(Value::UserFunction {
                    params: params.clone(),
                    body: body.clone(),
                    env: self.env_stack.clone(),
                }))
            }

            ASTNode::ImmediateInvocation(func, args) => {
                let func_val = match self.eval(func)? {
                    EvalResult::Value(v) => v,
                    EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                    EvalResult::Break => return Ok(EvalResult::Break),
                    EvalResult::Continue => return Ok(EvalResult::Continue),
                };

                match func_val {
                    Value::UserFunction { params, body, env } => {
                        let mut new_env = env.clone();
                        let mut local_env: HashMap<String, (Value, bool)> = HashMap::new();

                        let mut evaluated_args = Vec::new();
                        for (name, arg) in args {
                            let v = match self.eval(arg)? {
                                EvalResult::Value(v) => v,
                                EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                                EvalResult::Break => return Ok(EvalResult::Break),
                                EvalResult::Continue => return Ok(EvalResult::Continue),
                            };
                            evaluated_args.push((name, v));
                        }

                        let mut used_params = vec![false; params.len()];
                        
                        for (arg_name, value) in evaluated_args.iter() {
                            if let Some(name) = arg_name {
                                if let Some(pos) = params.iter().position(|p| &p.0 == name) {
                                    if used_params[pos] {
                                        return Err(format!("Parameter '{}' specified multiple times", name));
                                    }
                                    local_env.insert(name.clone(), (value.clone(), false));
                                    used_params[pos] = true;
                                } else {
                                    return Err(format!("Unknown parameter name '{}'", name));
                                }
                            }
                        }

                        let mut pos = 0;
                        for (param_name, default_value) in params.iter() {
                            if !used_params[pos] {
                                if let Some(arg_value) = evaluated_args.get(pos) {
                                    local_env.insert(param_name.clone(), (arg_value.1.clone(), false));
                                } else if let Some(default) = default_value {
                                    let default_val = match self.eval(default)? {
                                        EvalResult::Value(v) => v,
                                        EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                                        EvalResult::Break => return Ok(EvalResult::Break),
                                        EvalResult::Continue => return Ok(EvalResult::Continue),
                                    };
                                    local_env.insert(param_name.clone(), (default_val, false));
                                } else {
                                    return Err(format!("Missing argument for parameter '{}'", param_name));
                                }
                            }
                            pos += 1;
                        }

                        new_env.push(local_env);
                        let mut evaluator = Evaluator {
                            env_stack: new_env,
                        };
                        let result = evaluator.eval(&body)?;
                        match result {
                            EvalResult::Return(val) => Ok(EvalResult::Value(val)),
                            EvalResult::Value(val) => Ok(EvalResult::Value(val)),
                            EvalResult::Break => return Ok(EvalResult::Break),
                            EvalResult::Continue => return Ok(EvalResult::Continue),
                        }
                    }
                    _ => Err("Cannot invoke a non-function value".to_string()),
                }
            }

            ASTNode::FunctionCall(name, args) => {
                if let Some(val) = self.lookup(name) {
                    match val {
                        Value::BuiltInFunction(f) => {
                            let mut arg_values = Vec::new();
                            for (_, arg) in args {
                                let v = match self.eval(arg)? {
                                    EvalResult::Value(v) => v,
                                    EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                                    EvalResult::Break => return Ok(EvalResult::Break),
                                    EvalResult::Continue => return Ok(EvalResult::Continue),
                                };
                                arg_values.push(v);
                            }
                            let result = f(arg_values);
                            Ok(EvalResult::Value(result))
                        }
                        Value::UserFunction { params, body, env } => {
                            let mut new_env = env.clone();
                            let mut local_env: HashMap<String, (Value, bool)> = HashMap::new();

                            let mut evaluated_args = Vec::new();
                            for (name, arg) in args {
                                let v = match self.eval(arg)? {
                                    EvalResult::Value(v) => v,
                                    EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                                    EvalResult::Break => return Ok(EvalResult::Break),
                                    EvalResult::Continue => return Ok(EvalResult::Continue),
                                };
                                evaluated_args.push((name, v));
                            }

                            let mut used_params = vec![false; params.len()];
                            
                            for (arg_name, value) in evaluated_args.iter() {
                                if let Some(name) = arg_name {
                                    if let Some(pos) = params.iter().position(|p| &p.0 == name) {
                                        if used_params[pos] {
                                            return Err(format!("Parameter '{}' specified multiple times", name));
                                        }
                                        local_env.insert(name.clone(), (value.clone(), false));
                                        used_params[pos] = true;
                                    } else {
                                        return Err(format!("Unknown parameter name '{}'", name));
                                    }
                                }
                            }

                            let mut pos = 0;
                            for (arg_name, value) in evaluated_args.iter() {
                                if arg_name.is_none() {
                                    while pos < params.len() && used_params[pos] {
                                        pos += 1;
                                    }
                                    if pos >= params.len() {
                                        return Err("Too many arguments".to_string());
                                    }
                                    local_env.insert(params[pos].0.clone(), (value.clone(), false));
                                    used_params[pos] = true;
                                    pos += 1;
                                }
                            }

                            for (i, (param_name, default_value)) in params.iter().enumerate() {
                                if !used_params[i] {
                                    if let Some(default_expr) = default_value {
                                        let default_val = match self.eval(default_expr)? {
                                            EvalResult::Value(v) => v,
                                            EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                                            EvalResult::Break => return Ok(EvalResult::Break),
                                            EvalResult::Continue => return Ok(EvalResult::Continue),
                                        };
                                        local_env.insert(param_name.clone(), (default_val, false));
                                    } else {
                                        return Err(format!("Missing argument for parameter '{}'", param_name));
                                    }
                                }
                            }

                            new_env.push(local_env);
                            let mut evaluator = Evaluator {
                                env_stack: new_env,
                            };
                            let result = evaluator.eval(&body)?;
                            match result {
                                EvalResult::Return(val) => Ok(EvalResult::Value(val)),
                                EvalResult::Value(val) => Ok(EvalResult::Value(val)),
                                EvalResult::Break => return Ok(EvalResult::Break),
                                EvalResult::Continue => return Ok(EvalResult::Continue),
                            }
                        }
                        _ => Err(format!("'{}' is not a function", name)),
                    }
                } else {
                    Err(format!("Unknown function '{}'", name))
                }
            }

            ASTNode::MethodCall(obj, method, args) => {
                let obj_val = match self.eval(obj)? {
                    EvalResult::Value(v) => v,
                    EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                    EvalResult::Break => return Ok(EvalResult::Break),
                    EvalResult::Continue => return Ok(EvalResult::Continue),
                };
                let mut arg_vals = Vec::new();
                for (_, a) in args {
                    let v = match self.eval(a)? {
                        EvalResult::Value(v) => v,
                        EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                        EvalResult::Break => return Ok(EvalResult::Break),
                        EvalResult::Continue => return Ok(EvalResult::Continue),
                    };
                    arg_vals.push(v);
                }
                match obj_val.call_method(method, arg_vals) {
                    Ok(result) => Ok(EvalResult::Value(result)),
                    Err(e) => Err(format!(
                        "No such method '{}' for '{}': {}",
                        method,
                        obj.to_string(),
                        e
                    )),
                }
            }

            ASTNode::MemberAccess(object, member) => {
                let obj_val = match self.eval(object)? {
                    EvalResult::Value(v) => v,
                    EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                    EvalResult::Break => return Ok(EvalResult::Break),
                    EvalResult::Continue => return Ok(EvalResult::Continue),
                };
                if let Value::Module(ref map) = obj_val {
                    if let Some(val) = map.get(member) {
                        return Ok(EvalResult::Value(val.clone()));
                    }
                }
                Err(format!(
                    "No such member '{}' for '{}'",
                    member,
                    object.to_string()
                ))
            }

            ASTNode::BooleanLiteral(b) => Ok(EvalResult::Value(Value::Bool(*b))),

            ASTNode::TernaryExpression(condition, then_branch, else_branch) => {
                let cond_val = match self.eval(condition)? {
                    EvalResult::Value(v) => v,
                    EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                    EvalResult::Break => return Ok(EvalResult::Break),
                    EvalResult::Continue => return Ok(EvalResult::Continue),
                };
                match cond_val {
                    Value::Bool(true) => self.eval(then_branch),
                    Value::Bool(false) => self.eval(else_branch),
                    _ => Err("Condition in ternary expression must be a boolean".to_string()),
                }
            }

            ASTNode::IfStatement(condition, then_branch, else_branch) => {
                let cond_val = match self.eval(condition)? {
                    EvalResult::Value(v) => v,
                    EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                    EvalResult::Break => return Ok(EvalResult::Break),
                    EvalResult::Continue => return Ok(EvalResult::Continue),
                };
                if self.is_truthy(&cond_val) {
                    self.eval(then_branch)
                } else {
                    if let Some(else_branch) = else_branch {
                        self.eval(else_branch)
                    } else {
                        Ok(EvalResult::Value(Value::Null))
                    }
                }
            }

            ASTNode::UnaryExpression(op, expr) => {
                let val = match self.eval(expr)? {
                    EvalResult::Value(v) => v,
                    EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                    EvalResult::Break => return Ok(EvalResult::Break),
                    EvalResult::Continue => return Ok(EvalResult::Continue),
                };
                match (op.as_str(), val) {
                    ("!", Value::Bool(b)) => Ok(EvalResult::Value(Value::Bool(!b))),
                    _ => Err("Unsupported unary operation".to_string()),
                }
            }

            ASTNode::ArrayLiteral(elements) => {
                let mut vals = Vec::new();
                for el in elements {
                    let v = match self.eval(el)? {
                        EvalResult::Value(v) => v,
                        EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                        EvalResult::Break => return Ok(EvalResult::Break),
                        EvalResult::Continue => return Ok(EvalResult::Continue),
                    };
                    vals.push(v);
                }
                Ok(EvalResult::Value(Value::Array(vals)))
            }

            ASTNode::IndexAccess(array_expr, index_expr) => {
                let array_val = match self.eval(array_expr)? {
                    EvalResult::Value(v) => v,
                    EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                    EvalResult::Break => return Ok(EvalResult::Break),
                    EvalResult::Continue => return Ok(EvalResult::Continue),
                };
                let index_val = match self.eval(index_expr)? {
                    EvalResult::Value(v) => v,
                    EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                    EvalResult::Break => return Ok(EvalResult::Break),
                    EvalResult::Continue => return Ok(EvalResult::Continue),
                };
                match (array_val, index_val) {
                    (Value::Array(arr), Value::Number(idx)) => {
                        let idx = idx as usize;
                        arr.get(idx)
                            .cloned()
                            .map(EvalResult::Value)
                            .ok_or_else(|| "Array index out of bounds".to_string())
                    }
                    (Value::Array(arr), Value::Float(idx)) => {
                        let idx = idx as usize;
                        arr.get(idx)
                            .cloned()
                            .map(EvalResult::Value)
                            .ok_or_else(|| "Array index out of bounds".to_string())
                    }
                    (Value::HashMapV(hashm), Value::String(key)) => {
                        hashm.get(&key)
                            .cloned()
                            .map(EvalResult::Value)
                            .ok_or_else(|| format!("Key '{}' not found in hash map", key))
                    }
                    _ => Err("Indexing only supported for arrays with integer indices".to_string()),
                }
            }

            ASTNode::AssignmentIndex(array_expr, index_expr, value_expr) => {
                if let ASTNode::Identifier(var_name) = &**array_expr {
                    let index_val = match self.eval(index_expr)? {
                        EvalResult::Value(v) => v,
                        EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                        EvalResult::Break => return Ok(EvalResult::Break),
                        EvalResult::Continue => return Ok(EvalResult::Continue),
                    };
                    let value_val = match self.eval(value_expr)? {
                        EvalResult::Value(v) => v,
                        EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                        EvalResult::Break => return Ok(EvalResult::Break),
                        EvalResult::Continue => return Ok(EvalResult::Continue),
                    };
                    if let Some((val, _)) = self.current_env_mut().get_mut(var_name) {
                        if let Value::Array(arr) = val {
                            if let Value::Number(idx) = index_val {
                                let idx = idx as usize;
                                if idx < arr.len() {
                                    arr[idx] = value_val.clone();
                                    return Ok(EvalResult::Value(Value::Array(arr.clone())));
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
                let mut array_val = match self.eval(array_expr)? {
                    EvalResult::Value(v) => v,
                    EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                    EvalResult::Break => return Ok(EvalResult::Break),
                    EvalResult::Continue => return Ok(EvalResult::Continue),
                };
                let index_val = match self.eval(index_expr)? {
                    EvalResult::Value(v) => v,
                    EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                    EvalResult::Break => return Ok(EvalResult::Break),
                    EvalResult::Continue => return Ok(EvalResult::Continue),
                };
                let value_val = match self.eval(value_expr)? {
                    EvalResult::Value(v) => v,
                    EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                    EvalResult::Break => return Ok(EvalResult::Break),
                    EvalResult::Continue => return Ok(EvalResult::Continue),
                };
                if let Value::Array(ref mut arr) = array_val {
                    if let Value::Number(idx) = index_val {
                        let idx = idx as usize;
                        if idx < arr.len() {
                            arr[idx] = value_val.clone();
                            return Ok(EvalResult::Value(Value::Array(arr.clone())));
                        } else {
                            return Err("Array index out of bounds".to_string());
                        }
                    } else {
                        return Err("Assignment only supported for arrays with integer indices".to_string());
                    }
                } else if let Value::HashMapV(ref mut hashm) = array_val {
                    if let Value::String(key) = index_val {
                        hashm.insert(key, value_val.clone());
                        return Ok(EvalResult::Value(Value::HashMapV(hashm.clone())));
                    } else {
                        return Err("Assignment only supported for hash maps with string keys".to_string());
                    }
                } else {
                    return Err("Assignment only supported for arrays".to_string());
                }
            }

            ASTNode::FunctionDeclaration(name, params, body) => {
                let func = Value::UserFunction {
                    params: params.clone(),
                    body: Box::new((**body).clone()),
                    env: self.env_stack.clone(),
                };
                self.current_env_mut().insert(name.clone(), (func, false));
                Ok(EvalResult::Value(Value::Null))
            }

            ASTNode::WhileStatement(condition, body) => {
                let mut last = Value::Null;
                loop {
                    let cond_val = match self.eval(condition)? {
                        EvalResult::Value(v) => v,
                        EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                        EvalResult::Break => break,
                        EvalResult::Continue => continue,
                    };
                    if let Value::Bool(false) = cond_val {
                        break;
                    }
                    match self.eval(body)? {
                        EvalResult::Value(val) => last = val,
                        EvalResult::Return(val) => return Ok(EvalResult::Return(val)),
                        EvalResult::Break => break,
                        EvalResult::Continue => continue,
                    }
                }
                Ok(EvalResult::Value(last))
            }
            
            ASTNode::ReturnStatement(value) => {
                let val = if let Some(val) = value {
                    match self.eval(val)? {
                        EvalResult::Value(v) => v,
                        EvalResult::Return(v) => v,
                        EvalResult::Break => return Ok(EvalResult::Break),
                        EvalResult::Continue => return Ok(EvalResult::Continue),
                    }
                } else {
                    Value::Null
                };
                Ok(EvalResult::Return(val))
            }

            ASTNode::ForStatement(init, cond, incr, body) => {
                self.env_stack.push(HashMap::new());
                if let Some(init) = init {
                    self.eval(&*init)?;
                }
                loop {
                    if let Some(cond) = cond {
                        let cond_val = self.eval(&*cond)?;
                        match cond_val {
                            EvalResult::Value(Value::Bool(false)) => break,
                            EvalResult::Value(Value::Bool(true)) => {},
                            EvalResult::Return(val) => return Ok(EvalResult::Return(val)),
                            EvalResult::Break => break,
                            EvalResult::Continue => continue,
                            _ => return Err("Condition in 'for' must be a boolean".to_string()),
                        }
                    }
                    let res = self.eval(&*body)?;
                    match res {
                        EvalResult::Break => break,
                        EvalResult::Continue => {},
                        EvalResult::Return(val) => return Ok(EvalResult::Return(val)),
                        _ => {}
                    }
                    if let Some(incr) = incr {
                        self.eval(&*incr)?;
                    }
                }
                self.env_stack.pop();
                Ok(EvalResult::Value(Value::Bool(true)))
            }

            ASTNode::PostfixUnaryExpression(op, expr) => {
                if let ASTNode::Identifier(ref name) = **expr {
                    let mut val = self.lookup(name).ok_or(format!("Undefined variable '{}'", name))?;
                    match (&op[..], &mut val) {
                        ("++", Value::Number(n)) => {
                            let old = *n;
                            *n += 1;
                            for env in self.env_stack.iter_mut().rev() {
                                if let Some((Value::Number(v), _)) = env.get_mut(name) {
                                    *v = *n;
                                    break;
                            }
                            }
                            Ok(EvalResult::Value(Value::Number(old)))
                        }
                        ("++", Value::Float(n)) => {
                            let old = *n;
                            *n += 1.0;
                            for env in self.env_stack.iter_mut().rev() {
                                if let Some((Value::Float(v), _)) = env.get_mut(name) {
                                    *v = *n;
                                    break;
                                }
                            }
                            Ok(EvalResult::Value(Value::Float(old)))
                        }
                        ("--", Value::Number(n)) => {
                            let old = *n;
                            *n -= 1;
                            for env in self.env_stack.iter_mut().rev() {
                                if let Some((Value::Number(v), _)) = env.get_mut(name) {
                                    *v = *n;
                                    break;
                            }
                            }
                            Ok(EvalResult::Value(Value::Number(old)))
                        }
                        ("--", Value::Float(n)) => {
                            let old = *n;
                            *n -= 1.0;
                            for env in self.env_stack.iter_mut().rev() {
                                if let Some((Value::Float(v), _)) = env.get_mut(name) {
                                    *v = *n;
                                    break;
                                }
                            }
                            Ok(EvalResult::Value(Value::Float(old)))
                        }
                        _ => Err("Unsupported postfix operation".to_string()),
                    }
                } else {
                    Err("Postfix unary operation only supported on variables".to_string())
                }
            }

            ASTNode::AssignmentOp(op, left, right) => {
                if let ASTNode::Identifier(ref name) = **left {
                    let right_val = match self.eval(right)? {
                        EvalResult::Value(v) => v,
                        _ => return Err("Invalid right value".to_string()),
                    };
                    let mut val = self.lookup(name).ok_or(format!("Undefined variable '{}'", name))?;
                    let new_val = match (op.as_str(), &mut val, right_val) {
                        ("+=", Value::Number(n), Value::Number(r)) => {
                            *n += r;
                            Value::Number(*n)
                        }
                        ("-=", Value::Number(n), Value::Number(r)) => {
                            *n -= r;
                            Value::Number(*n)
                        }
                        ("*=", Value::Number(n), Value::Number(r)) => {
                            *n *= r;
                            Value::Number(*n)
                        }
                        ("/=", Value::Number(n), Value::Number(r)) => {
                            *n /= r;
                            Value::Number(*n)
                        }
                        ("+=", Value::Number(n), Value::Float(r)) => {
                            *n = (*n as f64 + r) as i64;
                            Value::Number(*n)
                        }
                        ("-=", Value::Number(n), Value::Float(r)) => {
                            *n = (*n as f64 - r) as i64;
                            Value::Number(*n)
                        }
                        ("*=", Value::Number(n), Value::Float(r)) => {
                            *n = (*n as f64 * r) as i64;
                            Value::Number(*n)
                        }
                        ("/=", Value::Number(n), Value::Float(r)) => {
                            *n = (*n as f64 / r) as i64;
                            Value::Number(*n)
                        }
                        ("+=", Value::Float(n), Value::Float(r)) => {
                            *n += r;
                            Value::Float(*n)
                        }
                        ("-=", Value::Float(n), Value::Float(r)) => {
                            *n -= r;
                            Value::Float(*n)
                        }
                        ("*=", Value::Float(n), Value::Float(r)) => {
                            *n *= r;
                            Value::Float(*n)
                        }
                        ("/=", Value::Float(n), Value::Float(r)) => {
                            *n /= r;
                            Value::Float(*n)
                        }
                        ("+=", Value::Float(n), Value::Number(r)) => {
                            *n += r as f64;
                            Value::Float(*n)
                        }
                        ("-=", Value::Float(n), Value::Number(r)) => {
                            *n -= r as f64;
                            Value::Float(*n)
                        }
                        ("*=", Value::Float(n), Value::Number(r)) => {
                            *n *= r as f64;
                            Value::Float(*n)
                        }
                        ("/=", Value::Float(n), Value::Number(r)) => {
                            *n /= r as f64;
                            Value::Float(*n)
                        }
                        ("+=", Value::String(s), Value::String(r)) => {
                            s.push_str(&r);
                            Value::String(s.clone())
                        }
                        _ => return Err("Unsupported assignment operator or type".to_string()),
                    };
                    for env in self.env_stack.iter_mut().rev() {
                        if let Some((_, is_const)) = env.get_mut(name) {
                            *env.get_mut(name).unwrap() = (new_val.clone(), *is_const);
                            break;
                        }
                    }
                    Ok(EvalResult::Value(new_val))
                } else {
                    Err("Assignment operator only supported on variables".to_string())
                }
            }

            ASTNode::HashMapLiteral(pairs) => {
                let mut map = std::collections::HashMap::new();
                for (k, v_expr) in pairs {
                    let v = match self.eval(v_expr)? {
                        EvalResult::Value(val) => val,
                        _ => return Err("Invalid value in hash map literal".to_string()),
                    };
                    map.insert(k.clone(), v);
                }
                Ok(EvalResult::Value(Value::HashMapV(map)))
            }

            ASTNode::Break => Ok(EvalResult::Break),
            
            ASTNode::Continue => Ok(EvalResult::Continue),

            ASTNode::MatchExpression(expr, arms) => {
                let value = match self.eval(expr)? {
                    EvalResult::Value(v) => v,
                    result => return Ok(result),
                };

                let mut default_arm = None;
                if let Some((pattern, result)) = arms.last() {
                    if let ASTNode::Identifier(name) = &**pattern {
                        if name == "_" {
                            default_arm = Some(result);
                        }
                    }
                }

                for (pattern, result) in arms.iter().take(arms.len() - default_arm.is_some() as usize) {
                    match &**pattern {
                        ASTNode::NumberLiteral(n) => {
                            if let Value::Number(val) = value {
                                if val == *n {
                                    return self.eval(result);
                                }
                            }
                            continue;
                        }
                        ASTNode::StringLiteral(s) => {
                            if let Value::String(val) = &value {
                                if val == s {
                                    return self.eval(result);
                                }
                            }
                            continue;
                        }
                        ASTNode::BooleanLiteral(b) => {
                            if let Value::Bool(val) = value {
                                if val == *b {
                                    return self.eval(result);
                                }
                            }
                            continue;
                        }
                        ASTNode::NullLiteral => {
                            if let Value::Null = value {
                                return self.eval(result);
                            }
                            continue;
                        }
                        _ => {}
                    }

                    let pattern_val = match self.eval(pattern)? {
                        EvalResult::Value(v) => v,
                        result => return Ok(result),
                    };

                    match self.eval_binary(value.clone(), "==", pattern_val)? {
                        Value::Bool(true) => return self.eval(result),
                        _ => {}
                    }
                }

                if let Some(default_result) = default_arm {
                    self.eval(default_result)
                } else {
                    Ok(EvalResult::Value(Value::Null))
                }
            }

            ASTNode::Range(start, end, inclusive) => {
                let start_val = match self.eval(start)? {
                    EvalResult::Value(v) => v,
                    EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                    EvalResult::Break => return Ok(EvalResult::Break),
                    EvalResult::Continue => return Ok(EvalResult::Continue),
                };
                let end_val = match self.eval(end)? {
                    EvalResult::Value(v) => v,
                    EvalResult::Return(v) => return Ok(EvalResult::Return(v)),
                    EvalResult::Break => return Ok(EvalResult::Break),
                    EvalResult::Continue => return Ok(EvalResult::Continue),
                };

                match (start_val, end_val) {
                    (Value::Number(start), Value::Number(end)) => {
                        let range: Vec<Value> = if *inclusive {
                            (start..=end).map(Value::Number).collect()
                        } else {
                            (start..end).map(Value::Number).collect()
                        };
                        Ok(EvalResult::Value(Value::Array(range)))
                    }
                    _ => Err("Range bounds must be numbers".to_string())
                }
            }
        }
    }

    fn eval_binary(&self, left: Value, op: &str, right: Value) -> Result<Value, String> {
        if op == "?:" {
            return Ok(if self.is_truthy(&left) { left } else { right });
        }
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
                "*" => Ok(Value::String(a.repeat(b as usize))),
                "+" => Ok(Value::String(a + &b.to_string())),
                "==" => Ok(Value::Bool(false)),
                _ => Err(format!("Unknown string-number operator: {}", op)),
            },
            (Value::Number(a), Value::String(b)) => match op {
                "+" => Ok(Value::String(a.to_string() + &b)),
                "==" => Ok(Value::Bool(false)),
                _ => Err(format!("Unknown number-string operator: {}", op)),
            },
            (Value::String(a), Value::Float(b)) => match op {
                "*" => Ok(Value::String(a.repeat(b as usize))),
                "+" => Ok(Value::String(a + &b.to_string())),
                "==" => Ok(Value::Bool(false)),
                _ => Err(format!("Unknown string-float operator: {}", op)),
            },
            (Value::Float(a), Value::String(b)) => match op {
                "+" => Ok(Value::String(a.to_string() + &b)),
                "==" => Ok(Value::Bool(false)),
                _ => Err(format!("Unknown float-string operator: {}", op)),
            },
            (Value::Bool(a), Value::Bool(b)) => match op {
                "&&" => Ok(Value::Bool(a && b)),
                "||" => Ok(Value::Bool(a || b)),
                "==" => Ok(Value::Bool(a == b)),
                "!=" => Ok(Value::Bool(a != b)),
                _ => Err(format!("Unknown boolean operator: {}", op)),
            },
            (Value::Array(a), Value::Array(b)) => match op {
                "+" => Ok(Value::Array(a.into_iter().chain(b.into_iter()).collect())),
                _ => Err(format!("Unknown array operator: {}", op)),
            },
            _ => Err("Type error: incompatible types for binary operation".to_string()),
        }
    }
}