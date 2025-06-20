#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Block(Vec<ASTNode>),
    VariableDeclaration(String, Option<Box<ASTNode>>),
    ConstDeclaration(String, Option<Box<ASTNode>>),
    Assignment(String, Box<ASTNode>),
    BinaryExpression(Box<ASTNode>, String, Box<ASTNode>),
    UnaryExpression(String, Box<ASTNode>),
    PostfixUnaryExpression(String, Box<ASTNode>),
    AssignmentOp(String, Box<ASTNode>, Box<ASTNode>),
    NumberLiteral(i64),
    FloatLiteral(f64),
    NullLiteral,
    StringLiteral(String),
    ArrayLiteral(Vec<Box<ASTNode>>),
    HashMapLiteral(Vec<(String, Box<ASTNode>)>),
    Identifier(String),
    FunctionDeclaration(String, Vec<(String, Option<Box<ASTNode>>)>, Box<ASTNode>),
    AnonymousFunction(Vec<(String, Option<Box<ASTNode>>)>, Box<ASTNode>),
    FunctionCall(String, Vec<(Option<String>, Box<ASTNode>)>),
    ImmediateInvocation(Box<ASTNode>, Vec<(Option<String>, Box<ASTNode>)>),
    IfStatement(Box<ASTNode>, Box<ASTNode>, Option<Box<ASTNode>>),
    TernaryExpression(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>),
    WhileStatement(Box<ASTNode>, Box<ASTNode>),
    ReturnStatement(Option<Box<ASTNode>>),
    MemberAccess(Box<ASTNode>, String), // Math.pi
    MethodCall(Box<ASTNode>, String, Vec<(Option<String>, Box<ASTNode>)>),
    BooleanLiteral(bool),
    IndexAccess(Box<ASTNode>, Box<ASTNode>),
    AssignmentIndex(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>), // array, index, value
    Break,
    Continue,
    ForStatement(
        Option<Box<ASTNode>>,
        Option<Box<ASTNode>>,
        Option<Box<ASTNode>>,
        Box<ASTNode>,
    ),
    MatchExpression(Box<ASTNode>, Vec<(Box<ASTNode>, Box<ASTNode>)>),
    Range(Box<ASTNode>, Box<ASTNode>, bool),
}

pub trait ASTNodeTrait {
    fn to_string(&self) -> String;
}

impl ASTNodeTrait for ASTNode {
    fn to_string(&self) -> String {
        match self {
            ASTNode::TernaryExpression(condition, then_branch, else_branch) => {
                format!("? {} -> {} : {}", condition.to_string(), then_branch.to_string(), else_branch.to_string())
            }
            ASTNode::ConstDeclaration(name, initializer) => {
                let init_str = if let Some(init) = initializer {
                    format!(" = {}", init.to_string())
                } else {
                    String::new()
                };
                format!("const {}{}", name, init_str)
            }
            ASTNode::NullLiteral => "null".to_string(),
            ASTNode::HashMapLiteral(pairs) => {
                let pairs_str: Vec<String> = pairs
                    .iter()
                    .map(|(key, value)| format!("{}: {}", key, value.to_string()))
                    .collect();
                format!("{{{}}}", pairs_str.join(", "))
            }
            ASTNode::AnonymousFunction(params, body) => {
                let params_str = params.iter().map(|(param, _)| param.clone()).collect::<Vec<String>>().join(", ");
                format!("({}) {}", params_str, body.to_string())
            }
            ASTNode::PostfixUnaryExpression(operator, expression) => {
                format!("{}{}", expression.to_string(), operator)
            }
            ASTNode::AssignmentOp(operator, left, right) => {
                format!("{} {} {}", left.to_string(), operator, right.to_string())
            }
            ASTNode::Block(statements) => {
                let mut result = String::new();
                for statement in statements {
                    result.push_str(&statement.to_string());
                    result.push('\n');
                }
                result
            }
            ASTNode::ArrayLiteral(elements) => {
                let elements_str: Vec<String> = elements.iter().map(|e| e.to_string()).collect();
                format!("[{}]", elements_str.join(", "))
            }
            ASTNode::UnaryExpression(operator, expression) => {
                format!("{}{}", operator, expression.to_string())
            }
            ASTNode::Program(statements) => {
                let mut result = String::new();
                for statement in statements {
                    result.push_str(&statement.to_string());
                    result.push('\n');
                }
                result
            }
            ASTNode::VariableDeclaration(name, initializer) => {
                let init_str = if let Some(init) = initializer {
                    format!(" = {}", init.to_string())
                } else {
                    String::new()
                };
                format!("let {}{}", name, init_str)
            }
            ASTNode::Assignment(name, value) => format!("{} = {}", name, value.to_string()),
            ASTNode::BinaryExpression(left, operator, right) => format!("{} {} {}", left.to_string(), operator, right.to_string()),
            ASTNode::NumberLiteral(value) => value.to_string(),
            ASTNode::FloatLiteral(value) => value.to_string(),
            ASTNode::StringLiteral(value) => format!("\"{}\"", value),
            ASTNode::Identifier(name) => name.clone(),
            ASTNode::FunctionDeclaration(name, params, body) => {
                let params_str = params.iter().map(|(param, _)| param.clone()).collect::<Vec<String>>().join(", ");
                format!("fn {}({}) {}", name, params_str, body.to_string())
            }
            ASTNode::FunctionCall(name, args) => {
                let args_str: Vec<String> = args.iter().map(|(arg, _)| arg.clone().unwrap_or_default()).collect();
                format!("{}({})", name, args_str.join(", "))
            }
            ASTNode::IfStatement(condition, then_branch, else_branch) => {
                let else_str = if let Some(else_branch) = else_branch {
                    format!(" else {}", else_branch.to_string())
                } else {
                    String::new()
                };
                format!("if {} {}{}", condition.to_string(), then_branch.to_string(), else_str)
            }
            ASTNode::WhileStatement(condition, body) => format!("while {} {}", condition.to_string(), body.to_string()),
            ASTNode::ReturnStatement(value) => {
                if let Some(value) = value {
                    format!("return {}", value.to_string())
                } else {
                    "return".to_string()
                }
            }
            ASTNode::MemberAccess(object, property) => format!("{}.{}", object.to_string(), property),
            ASTNode::MethodCall(object, method, args) => {
                let args_str: Vec<String> = args.iter().map(|(arg, _)| arg.clone().unwrap_or_default()).collect();
                format!("{}.{}/{}", object.to_string(), method, args_str.join(", "))
            }
            ASTNode::BooleanLiteral(value) => value.to_string(),
            ASTNode::IndexAccess(array, index) => format!("{}[{}]", array.to_string(), index.to_string()),
            ASTNode::AssignmentIndex(array, index, value) => format!("{}[{}] = {}", array.to_string(), index.to_string(), value.to_string()),
            ASTNode::Break => "break".to_string(),
            ASTNode::Continue => "continue".to_string(),
            ASTNode::ForStatement(initializer, condition, increment, body) => {
                let init_str = if let Some(init) = initializer {
                    format!("{}; ", init.to_string())
                } else {
                    String::new()
                };
                let cond_str = if let Some(cond) = condition {
                    format!("{}; ", cond.to_string())
                } else {
                    String::new()
                };
                let inc_str = if let Some(inc) = increment {
                    format!("{}", inc.to_string())
                } else {
                    String::new()
                };
                format!("for {}{}{}{}", init_str, cond_str, inc_str, body.to_string())
            }
            ASTNode::ImmediateInvocation(func, args) => {
                let args_str: Vec<String> = args.iter().map(|(arg, _)| arg.clone().unwrap_or_default()).collect();
                format!("{}({})", func.to_string(), args_str.join(", "))
            }
            ASTNode::MatchExpression(expr, arms) => {
                let arms_str: Vec<String> = arms
                    .iter()
                    .map(|(pattern, result)| format!("{} => {}", pattern.to_string(), result.to_string()))
                    .collect();
                format!("match {} {{ {} }}", expr.to_string(), arms_str.join(", "))
            }
            ASTNode::Range(start, end, inclusive) => {
                format!("{}{}{}",
                    start.to_string(),
                    if *inclusive { "..=" } else { ".." },
                    end.to_string()
                )
            },
        }
    }
}

// impl ASTNode {
//     pub fn new_program(statements: Vec<ASTNode>) -> Self {
//         ASTNode::Program(statements)
//     }

//     pub fn new_variable_declaration(name: String, initializer: Option<Box<ASTNode>>) -> Self {
//         ASTNode::VariableDeclaration(name, initializer)
//     }

//     pub fn new_assignment(name: String, value: Box<ASTNode>) -> Self {
//         ASTNode::Assignment(name, value)
//     }

//     pub fn new_binary_expression(left: Box<ASTNode>, operator: String, right: Box<ASTNode>) -> Self {
//         ASTNode::BinaryExpression(left, operator, right)
//     }

//     pub fn new_number_literal(value: i64) -> Self {
//         ASTNode::NumberLiteral(value)
//     }

//     pub fn new_identifier(name: String) -> Self {
//         ASTNode::Identifier(name)
//     }

//     pub fn new_function_declaration(name: String, params: Vec<String>, body: Box<ASTNode>) -> Self {
//         ASTNode::FunctionDeclaration(name, params, body)
//     }

//     pub fn new_function_call(name: String, args: Vec<Box<ASTNode>>) -> Self {
//         ASTNode::FunctionCall(name, args)
//     }
//     pub fn new_if_statement(condition: Box<ASTNode>, then_branch: Box<ASTNode>, else_branch: Option<Box<ASTNode>>) -> Self {
//         ASTNode::IfStatement(condition, then_branch, else_branch)
//     }
//     pub fn new_while_statement(condition: Box<ASTNode>, body: Box<ASTNode>) -> Self {
//         ASTNode::WhileStatement(condition, body)
//     }
//     pub fn new_return_statement(value: Option<Box<ASTNode>>) -> Self {
//         ASTNode::ReturnStatement(value)
//     }
//     pub fn new_member_access(object: Box<ASTNode>, property: String) -> Self {
//         ASTNode::MemberAccess(object, property)
//     }
//     pub fn new_method_call(object: Box<ASTNode>, method: String, args: Vec<Box<ASTNode>>) -> Self {
//         ASTNode::MethodCall(object, method, args)
//     }
// }