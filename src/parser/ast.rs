#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    VariableDeclaration(String, Option<Box<ASTNode>>),
    Assignment(String, Box<ASTNode>),
    BinaryExpression(Box<ASTNode>, String, Box<ASTNode>),
    NumberLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    Identifier(String),
    FunctionDeclaration(String, Vec<String>, Box<ASTNode>),
    FunctionCall(String, Vec<Box<ASTNode>>),
    IfStatement(Box<ASTNode>, Box<ASTNode>, Option<Box<ASTNode>>),
    WhileStatement(Box<ASTNode>, Box<ASTNode>),
    ReturnStatement(Option<Box<ASTNode>>),
    MemberAccess(Box<ASTNode>, String), // Math.pi
    MethodCall(Box<ASTNode>, String, Vec<Box<ASTNode>>), // Math.pow(x,y)
    BooleanLiteral(bool),
}

pub trait ASTNodeTrait {
    fn to_string(&self) -> String;
}

impl ASTNodeTrait for ASTNode {
    fn to_string(&self) -> String {
        match self {
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
                let params_str = params.join(", ");
                format!("fn {}({}) {}", name, params_str, body.to_string())
            }
            ASTNode::FunctionCall(name, args) => {
                let args_str: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
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
                let args_str: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
                format!("{}.{}/{}", object.to_string(), method, args_str.join(", "))
            }
            ASTNode::BooleanLiteral(value) => value.to_string(),
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