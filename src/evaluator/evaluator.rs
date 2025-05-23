use std::collections::HashMap;

use crate::parser::ast::{ASTNode, ASTNodeTrait}; // se tiver trait, senão remova
use crate::parser::parser::Parser;
use crate::constants::token::Token;

pub struct Evaluator<'a> {
    parser: Parser<'a>,
    env: HashMap<String, i64>,
}

impl<'a> Evaluator<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Evaluator {
            parser: Parser::new(tokens),
            env: HashMap::new(),
        }
    }

    pub fn evaluate(&mut self) -> Result<i64, String> {
        let ast = self.parser.parse()?;
        self.eval(&ast)
    }

    // -----------------------------------------------------
    // ------------------   CORE EVAL    -------------------
    // -----------------------------------------------------

    fn eval(&mut self, node: &ASTNode) -> Result<i64, String> {
        match node {
            ASTNode::Program(statements) => {
                let mut last = 0;
                for stmt in statements {
                    last = self.eval(stmt)?;
                }
                Ok(last)
            }

            ASTNode::VariableDeclaration(name, Some(expr)) => {
                let val = self.eval(expr)?;
                self.env.insert(name.clone(), val);
                Ok(val)
            }

            ASTNode::VariableDeclaration(_, None) => Ok(0),

            ASTNode::BinaryExpression(left, op, right) => {
                let l = self.eval(left)?;
                let r = self.eval(right)?;
                match op.as_str() {
                    "+" => Ok(l + r),
                    "-" => Ok(l - r),
                    "*" => Ok(l * r),
                    "/" => Ok(l / r),
                    _   => Err(format!("Unknown operator: {}", op)),
                }
            }

            ASTNode::NumberLiteral(n) => Ok(*n),

            ASTNode::Identifier(name) => {
                self.env
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("Undefined variable '{}'", name))
            }

            _ => Err("Unsupported AST node".into()),
        }
    }
}
