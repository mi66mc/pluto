use std::collections::HashMap;
use crate::{evaluator::evaluator::Value};
use std::io::Write;

pub type MethodFn = fn(&Value, Vec<Value>) -> Result<Value, String>;

fn string_len(v: &Value, _: Vec<Value>) -> Result<Value, String> {
    if let Value::String(s) = v {
        Ok(Value::Number(s.len() as i64))
    } else {
        Err("Not a string".into())
    }
}

fn string_to_number(v: &Value, _: Vec<Value>) -> Result<Value, String> {
    if let Value::String(s) = v {
        if let Ok(n) = s.parse::<i64>() {
            Ok(Value::Number(n))
        } else {
            Err("Not a number".into())
        }
    } else {
        Err("Not a string".into())
    }
}

fn string_to_float(v: &Value, _: Vec<Value>) -> Result<Value, String> {
    if let Value::String(s) = v {
        if let Ok(n) = s.parse::<f64>() {
            Ok(Value::Float(n))
        } else {
            Err("Not a float".into())
        }
    } else {
        Err("Not a string".into())
    }
}

fn string_to_uppercase(v: &Value, _: Vec<Value>) -> Result<Value, String> {
    if let Value::String(s) = v {
        Ok(Value::String(s.to_uppercase()))
    } else {
        Err("Not a string".into())
    }
}

fn string_to_lowercase(v: &Value, _: Vec<Value>) -> Result<Value, String> {
    if let Value::String(s) = v {
        Ok(Value::String(s.to_lowercase()))
    } else {
        Err("Not a string".into())
    }
}

fn string_char_at(v: &Value, args: Vec<Value>) -> Result<Value, String> {
    if let Value::String(s) = v {
        if let Some(Value::Number(n)) = args.get(0) {
            if *n >= 0 && *n < s.len() as i64 {
                return Ok(Value::String(s.chars().nth(*n as usize).unwrap().to_string()));
            }
        }
        Err("Index out of bounds".into())
    } else {
        Err("Not a string".into())
    }
}

// ------------------------------------------------------

fn number_and_float_to_string(v: &Value, _: Vec<Value>) -> Result<Value, String> {
    match v {
        Value::Number(n) => Ok(Value::String(n.to_string())),
        Value::Float(f) => Ok(Value::String(f.to_string())),
        _ => Err("Not a number".into()),
    }
}

// ------------------------------------------------------

fn array_len(v: &Value, _: Vec<Value>) -> Result<Value, String> {
    if let Value::Array(arr) = v {
        Ok(Value::Number(arr.len() as i64))
    } else {
        Err("Not an array".into())
    }
}

fn array_push(v: &Value, args: Vec<Value>) -> Result<Value, String> {
    if let Value::Array(arr) = v {
        let mut new_arr = arr.clone();
        for arg in args {
            new_arr.push(arg);
        }
        Ok(Value::Array(new_arr))
    } else {
        Err("Not an array".into())
    }
}

fn array_pop(v: &Value, _: Vec<Value>) -> Result<Value, String> {
    if let Value::Array(arr) = v {
        if arr.is_empty() {
            return Err("Array is empty".into());
        }
        let mut new_arr = arr.clone();
        new_arr.pop();
        Ok(Value::Array(new_arr))
    } else {
        Err("Not an array".into())
    }
}

fn array_remove(v: &Value, args: Vec<Value>) -> Result<Value, String> {
    if let Value::Array(arr) = v {
        if let Some(Value::Number(n)) = args.get(0) {
            if *n >= 0 && *n < arr.len() as i64 {
                let mut new_arr = arr.clone();
                new_arr.remove(*n as usize);
                return Ok(Value::Array(new_arr));
            }
        }
        Err("Index out of bounds".into())
    } else {
        Err("Not an array".into())
    }
}

fn array_sum(v: &Value, _: Vec<Value>) -> Result<Value, String> {
    if let Value::Array(arr) = v {
        let sum: f64 = arr.iter().filter_map(|x| {
            if let Value::Number(n) = x {
                Some(*n as f64)
            } else if let Value::Float(f) = x {
                Some(*f)
            } else {
                None
            }
        }).sum();
        Ok(Value::Float(sum))
    } else {
        Err("Not an array".into())
    }
}

// ------------------------------------------------------

pub fn string_methods() -> HashMap<&'static str, MethodFn> {
    let mut map = HashMap::new();
    map.insert("len", string_len as MethodFn);
    map.insert("to_int", string_to_number as MethodFn);
    map.insert("to_float", string_to_float as MethodFn);
    map.insert("to_upper", string_to_uppercase as MethodFn);
    map.insert("to_lower", string_to_lowercase as MethodFn);
    map.insert("char_at", string_char_at as MethodFn);
    map
}

pub fn number_methods() -> HashMap<&'static str, MethodFn> {
    let mut map = HashMap::new();
    map.insert("to_string", number_and_float_to_string as MethodFn);
    map
}

pub fn float_methods() -> HashMap<&'static str, MethodFn> {
    let mut map = HashMap::new();
    map.insert("to_string", number_and_float_to_string as MethodFn);
    map
}

pub fn array_methods() -> HashMap<&'static str, MethodFn> {
    let mut map = HashMap::new();
    map.insert("len", array_len as MethodFn);
    map.insert("push", array_push as MethodFn);
    map.insert("pop", array_pop as MethodFn);
    map.insert("remove", array_remove as MethodFn);
    map.insert("sum", array_sum as MethodFn);
    map
}

pub fn default_env() -> HashMap<String, Value> {
    let mut env = HashMap::new();

    // -----------------------------------------------------
    // -------------------- MODULES ------------------------
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
        if let Some(Value::Float(f)) = args.get(2) {
            return Value::Float(a.powf(b) + f);
        }
        Value::Float(0.0)
    }));

    env.insert("Math".to_string(), Value::Module(math));

    // -----------------------------------------------------
    // -------------------- GENERAL ------------------------
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
                    Value::Array(_) => Value::String("Array".to_string()),
                    Value::Module(_) => Value::String("Module".to_string()),
                    Value::BuiltInFunction(_) => Value::String("BuiltInFunction".to_string()),
                    Value::UserFunction { .. } => Value::String("UserFunction".to_string()),
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

    env.insert(
        "exit".to_string(),
        Value::BuiltInFunction(|args| {
            if let Some(Value::Number(n)) = args.get(0) {
                std::process::exit(*n as i32);
            } else {
                std::process::exit(0);
            }
        }),
    );
    env.insert(
        "format".to_string(),
        Value::BuiltInFunction(|args| {
            if args.is_empty() {
                return Value::String("".to_string());
            }
            let template = match &args[0] {
                Value::String(s) => s,
                _ => return Value::String("".to_string()),
            };

            let mut result = String::new();
            let mut arg_iter = args.iter().skip(1);
            let mut chars = template.chars().peekable();

            while let Some(c) = chars.next() {
                if c == '{' {
                    if let Some(&next) = chars.peek() {
                        if next == '}' {
                            chars.next(); // '}'
                            if let Some(val) = arg_iter.next() {
                                result.push_str(&val.to_string());
                            } else {
                                result.push_str("{}");
                            }
                            continue;
                        }
                    }
                }
                result.push(c);
            }
            Value::String(result)
        }),
    );

    env
}