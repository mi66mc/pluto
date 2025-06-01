use crate::constants::token::{Token, TokenKind, TokenKindTrait};
use crate::parser::ast::ASTNode;

pub struct Parser {
    tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<ASTNode, String> {
        let mut statements = Vec::new();

        while self.current < self.tokens.len()
            && self.tokens[self.current].kind != TokenKind::EOF
        {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        Ok(ASTNode::Program(statements))
    }

    // -----------------------------------------------------
    // ------------------   STATEMENTS   -------------------
    // -----------------------------------------------------

    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        if self.match_kind(TokenKind::Let) {
            self.parse_variable_declaration()
        } else if self.match_kind(TokenKind::Const) {
            self.parse_const_declaration()
        } else if self.match_kind(TokenKind::While) {
            self.parse_while_statement()
        } else if self.match_kind(TokenKind::Return) {
            self.parse_return_statement()
        } else if self.match_kind(TokenKind::Fn) {
            self.parse_function_declaration()
        } else if self.match_kind(TokenKind::If) {
            self.parse_if_statement()
        } else if self.match_kind(TokenKind::Break) {
            self.consume(TokenKind::Semicolon, "Expected ';' after 'break'")?;
            Ok(ASTNode::Break)
        } else if self.match_kind(TokenKind::Continue) {
            self.consume(TokenKind::Semicolon, "Expected ';' after 'continue'")?;
            Ok(ASTNode::Continue)
        } else if self.match_kind(TokenKind::For) {
            self.parse_for_statement()
        } else {
            let expr = self.parse_expression(0)?;
            self.consume(TokenKind::Semicolon, "Expected ';' after expression")?;
            Ok(expr)
        }
    }

    fn parse_while_statement(&mut self) -> Result<ASTNode, String> {
        self.consume(TokenKind::LParen, "Expected '(' after 'while'")?;
        let condition = self.parse_expression(0)?;
        self.consume(TokenKind::RParen, "Expected ')' after while condition")?;
        let body = self.parse_block_or_single_statement()?;
        Ok(ASTNode::WhileStatement(Box::new(condition), Box::new(body)))
    }

    fn parse_return_statement(&mut self) -> Result<ASTNode, String> {
        let expr = if self.peek_kind() == Some(&TokenKind::Semicolon) {
            None
        } else {
            Some(Box::new(self.parse_expression(0)?))
        };
        self.consume(TokenKind::Semicolon, "Expected ';' after return statement")?;
        Ok(ASTNode::ReturnStatement(expr))
    }

    fn parse_function_declaration(&mut self) -> Result<ASTNode, String> {
        let name = if let Some(TokenKind::Identifier(id)) = self.peek_kind().cloned() {
            self.advance();
            id
        } else {
            return Err("Expected identifier after 'fn'".into());
        };

        self.consume(TokenKind::LParen, "Expected '(' after function name")?;

        let mut params = Vec::new();
        if self.peek_kind() != Some(&TokenKind::RParen) {
            loop {
                if let Some(TokenKind::Identifier(param)) = self.peek_kind().cloned() {
                    self.advance();
                    let default_value = if self.peek_kind() == Some(&TokenKind::Equal) {
                        self.advance(); // consume '='
                        Some(Box::new(self.parse_expression(0)?))
                    } else {
                        None
                    };
                    params.push((param, default_value));
                } else {
                    return Err("Expected identifier in function parameters".into());
                }
                if self.peek_kind() == Some(&TokenKind::RParen) {
                    break;
                }
                self.consume(TokenKind::Comma, "Expected ',' or ')' in function parameters")?;
            }
        }

        self.consume(TokenKind::RParen, "Expected ')' after function parameters")?;
        let body = self.parse_block_or_single_statement()?;
        Ok(ASTNode::FunctionDeclaration(name, params, Box::new(body)))
    }

    fn parse_function_params(&mut self) -> Result<Vec<(String, Option<Box<ASTNode>>)>, String> {
        let mut params = Vec::new();
        if self.peek_kind() != Some(&TokenKind::RParen) {
            loop {
                if let Some(TokenKind::Identifier(param)) = self.peek_kind().cloned() {
                    self.advance();
                    let default_value = if self.peek_kind() == Some(&TokenKind::Equal) {
                        self.advance(); // consume '='
                        Some(Box::new(self.parse_expression(0)?))
                    } else {
                        None
                    };
                    params.push((param, default_value));
                } else {
                    return Err("Expected identifier in function parameters".into());
                }
                if self.peek_kind() == Some(&TokenKind::RParen) {
                    break;
                }
                self.consume(TokenKind::Comma, "Expected ',' or ')' in function parameters")?;
            }
        }
        Ok(params)
    }

    fn parse_variable_declaration(&mut self) -> Result<ASTNode, String> {
        let name = if let Some(TokenKind::Identifier(id)) = self.peek_kind().cloned() {
            self.advance();
            id
        } else {
            return Err("Expected identifier after 'let'".into());
        };

        if self.peek_kind() == Some(&TokenKind::Equal) {
            self.advance(); // '='
            let expr = self.parse_expression(0)?;
            self.consume(TokenKind::Semicolon, "Expected ';' after variable declaration")?;
            Ok(ASTNode::VariableDeclaration(name, Some(Box::new(expr))))
        } else {
            self.consume(TokenKind::Semicolon, "Expected ';' after variable declaration")?;
            Ok(ASTNode::VariableDeclaration(name, None))
        }
    }

    fn parse_const_declaration(&mut self) -> Result<ASTNode, String> {
        let name = if let Some(TokenKind::Identifier(id)) = self.peek_kind().cloned() {
            self.advance();
            id
        } else {
            return Err("Expected identifier after 'const'".into());
        };

        if self.peek_kind() == Some(&TokenKind::Equal) {
            self.advance(); // '='
            let expr = self.parse_expression(0)?;
            self.consume(TokenKind::Semicolon, "Expected ';' after constant declaration")?;
            Ok(ASTNode::ConstDeclaration(name, Some(Box::new(expr))))
        } else {
            self.consume(TokenKind::Semicolon, "Expected ';' after constant declaration")?;
            Ok(ASTNode::ConstDeclaration(name, None))
        }
    }

    fn parse_if_statement(&mut self) -> Result<ASTNode, String> {
        let condition = self.parse_expression(0)?;
        let then_branch = self.parse_block_or_single_statement()?;
        
        let mut else_branch = None;
        if self.match_kind(TokenKind::Else) {
            if self.match_kind(TokenKind::If) {
                else_branch = Some(Box::new(self.parse_if_statement()?));
            } else {
                else_branch = Some(Box::new(self.parse_block_or_single_statement()?));
            }
        }
        
        Ok(ASTNode::IfStatement(Box::new(condition), Box::new(then_branch), else_branch))
    }

    fn parse_block_or_single_statement(&mut self) -> Result<ASTNode, String> {
        if self.match_kind(TokenKind::LBrace) {
            let mut statements = Vec::new();
            while !self.match_kind(TokenKind::RBrace) && self.current < self.tokens.len() {
                statements.push(self.parse_statement()?);
            }
            Ok(ASTNode::Block(statements))
        } else {
            self.parse_statement()
        }
    }

    // -----------------------------------------------------
    // ------------------   EXPRESSIONS  -------------------
    // -----------------------------------------------------

    fn parse_expression(&mut self, min_prec: u8) -> Result<ASTNode, String> {
        let mut left = self.parse_primary()?;

        if self.peek_kind() == Some(&TokenKind::Equal) {
            if let ASTNode::Identifier(ref name) = left {
                self.advance(); // '='
                let right = self.parse_expression(0)?;
                return Ok(ASTNode::Assignment(name.clone(), Box::new(right)));
            }
            if let ASTNode::IndexAccess(array, index) = left {
                self.advance(); // '='
                let right = self.parse_expression(0)?;
                return Ok(ASTNode::AssignmentIndex(array, index, Box::new(right)));
            }
        }

        if let Some(kind) = self.peek_kind() {
            let op = match kind {
                TokenKind::PlusEqual => Some("+="),
                TokenKind::MinusEqual => Some("-="),
                TokenKind::StarEqual => Some("*="),
                TokenKind::SlashEqual => Some("/="),
                _ => None,
            };
            if let Some(op_str) = op {
                self.advance();
                let right = self.parse_expression(0)?;
                return Ok(ASTNode::AssignmentOp(op_str.to_string(), Box::new(left), Box::new(right)));
            }
        }

        while let Some(op_prec) = self.current_precedence() {
            if op_prec < min_prec {
                break;
            }
            let op_token = self.advance().clone();
            let right = self.parse_expression(op_prec + 1)?;
            left = ASTNode::BinaryExpression(
                Box::new(left),
                op_token.kind.to_string(),
                Box::new(right),
            );
        }

        Ok(left)
    }

    fn current_precedence(&self) -> Option<u8> {
        match self.peek_kind()? {
            TokenKind::Or => Some(1),
            TokenKind::And => Some(2),
            TokenKind::EqualsEqual | TokenKind::NotEqual => Some(3),
            TokenKind::LessThan | TokenKind::GreaterThan | TokenKind::LessThanEqual | TokenKind::GreaterThanEqual => Some(4),
            TokenKind::Plus | TokenKind::Minus => Some(5),
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Some(6),
            _ => None,
        }
    }

    fn parse_primary(&mut self) -> Result<ASTNode, String> {
        let mut node = match self.advance().kind.clone() {
            TokenKind::Number(n)            => ASTNode::NumberLiteral(n),
            TokenKind::Float(f)             => ASTNode::FloatLiteral(f),
            TokenKind::Boolean(b)          => ASTNode::BooleanLiteral(b),
            TokenKind::StringLiteral(s)  => ASTNode::StringLiteral(s),
            TokenKind::LBracket => {
                let mut elements = Vec::new();
                if self.peek_kind() != Some(&TokenKind::RBracket) {
                    loop {
                        let expr = self.parse_expression(0)?;
                        elements.push(Box::new(expr));
                        if self.peek_kind() == Some(&TokenKind::RBracket) {
                            break;
                        }
                        self.consume(TokenKind::Comma, "Expected ',' or ']' in array literal")?;
                    }
                }
                self.consume(TokenKind::RBracket, "Expected ']' after array literal")?;
                ASTNode::ArrayLiteral(elements)
            }
            TokenKind::Identifier(s)     => {
                if self.peek_kind() == Some(&TokenKind::LParen) {
                    self.advance();
                    let mut args = Vec::new();
                    if self.peek_kind() != Some(&TokenKind::RParen) {
                        loop {
                            let mut arg_name = None;
                            if let Some(&TokenKind::Identifier(ref name)) = self.peek_kind() {
                                let next_token = self.tokens.get(self.current + 1).map(|t| &t.kind);
                                if next_token == Some(&TokenKind::Equal) {
                                    let name = name.clone();
                                    self.advance(); // consume identifier
                                    self.advance(); // consume equals
                                    arg_name = Some(name);
                                }
                            }
                            let arg = self.parse_expression(0)?;
                            args.push((arg_name, Box::new(arg)));
                            if self.peek_kind() == Some(&TokenKind::RParen) {
                                break;
                            }
                            self.consume(TokenKind::Comma, "Expected ',' or ')' in function arguments")?;
                        }
                    }
                    self.consume(TokenKind::RParen, "Expected ')' after arguments")?;
                    ASTNode::FunctionCall(s, args)
                } else {
                    ASTNode::Identifier(s)
                }
            }
            TokenKind::LParen => {
                let start_pos = self.current;
                let mut is_param_list = true;
                let mut temp_pos = self.current;

                if self.peek_kind() != Some(&TokenKind::RParen) {
                    loop {
                        if let Some(TokenKind::Identifier(_)) = self.tokens.get(temp_pos).map(|t| &t.kind) {
                            temp_pos += 1;
                            if self.tokens.get(temp_pos).map(|t| &t.kind) == Some(&TokenKind::Equal) {
                                temp_pos += 1;
                                while temp_pos < self.tokens.len() {
                                    let token = &self.tokens[temp_pos].kind;
                                    if matches!(token, TokenKind::Comma | TokenKind::RParen) {
                                        break;
                                    }
                                    temp_pos += 1;
                                }
                            }
                        } else {
                            is_param_list = false;
                            break;
                        }
                        if self.tokens.get(temp_pos).map(|t| &t.kind) == Some(&TokenKind::RParen) {
                            temp_pos += 1;
                            break;
                        }
                        if self.tokens.get(temp_pos).map(|t| &t.kind) == Some(&TokenKind::Comma) {
                            temp_pos += 1;
                        } else {
                            is_param_list = false;
                            break;
                        }
                    }
                } else {
                    temp_pos += 1;
                }

                // check '->'
                if is_param_list && self.tokens.get(temp_pos).map(|t| &t.kind) == Some(&TokenKind::ArrowFunc) {
                    let params = self.parse_function_params()?;
                    self.consume(TokenKind::RParen, "Expected ')' after function parameters")?;
                    self.consume(TokenKind::ArrowFunc, "Expected '->' after function parameters")?;
                    let body = if self.peek_kind() == Some(&TokenKind::LBrace) {
                        self.parse_block_or_single_statement()?
                    } else {
                        self.parse_expression(0)?
                    };
                    return Ok(ASTNode::AnonymousFunction(params, Box::new(body)));
                } else {
                    self.current = start_pos;
                    let expr = self.parse_expression(0)?;
                    self.consume(TokenKind::RParen, "Expected ')' after expression")?;
                    return Ok(expr);
                }
            }
            TokenKind::Not => {
                let expr = self.parse_primary()?;
                ASTNode::UnaryExpression("!".to_string(), Box::new(expr))
            }
            TokenKind::Null => ASTNode::NullLiteral,
            TokenKind::LBrace => {
                // hashmap
                let mut pairs = Vec::new();
                if self.peek_kind() != Some(&TokenKind::RBrace) {
                    loop {
                        let key = match self.advance().kind.clone() {
                            TokenKind::StringLiteral(s) => s,
                            TokenKind::Identifier(s) => s,
                            other => return Err(format!("Expected string or identifier as key, got {:?}", other)),
                        };
                        self.consume(TokenKind::Colon, "Expected ':' after key in hash map literal")?;
                        let value = self.parse_expression(0)?;
                        pairs.push((key, Box::new(value)));
                        if self.peek_kind() == Some(&TokenKind::RBrace) {
                            break;
                        }
                        self.consume(TokenKind::Comma, "Expected ',' or '}' in hash map literal")?;
                    }
                }
                self.consume(TokenKind::RBrace, "Expected '}' after hash map literal")?;
                ASTNode::HashMapLiteral(pairs)
            }
            TokenKind::While => {
                return self.parse_while_statement();
            }
            TokenKind::For => {
                return self.parse_for_statement();
            }
            other => return Err(format!("Unexpected token: {:?}", other)),
        };

        loop {
            if self.peek_kind() == Some(&TokenKind::Dot) {
                self.advance(); // '.'
                let member_token = self.advance();
                let member_name = if let TokenKind::Identifier(ref s) = member_token.kind {
                    s.clone()
                } else {
                    return Err("Expected identifier after '.'".into());
                };
                if self.peek_kind() == Some(&TokenKind::LParen) {
                    self.advance(); // '('
                    let mut args = Vec::new();
                    if self.peek_kind() != Some(&TokenKind::RParen) {
                        loop {
                            let mut arg_name = None;
                            let next_token = self.peek_kind().cloned();
                            let next_next_token = self.tokens.get(self.current + 1).map(|t| &t.kind);
                            if let Some(TokenKind::Identifier(name)) = next_token {
                                if next_next_token == Some(&TokenKind::Equal) {
                                    self.advance(); // consume identifier
                                    self.advance(); // consume equals
                                    arg_name = Some(name);
                                }
                            }
                            let arg = self.parse_expression(0)?;
                            args.push((arg_name, Box::new(arg)));
                            if self.peek_kind() == Some(&TokenKind::RParen) {
                                break;
                            }
                            self.consume(TokenKind::Comma, "Expected ',' or ')' in argument list")?;
                        }
                    }
                    self.consume(TokenKind::RParen, "Expected ')' after arguments")?;
                    node = ASTNode::MethodCall(Box::new(node), member_name, args);
                } else {
                    node = ASTNode::MemberAccess(Box::new(node), member_name);
                }
            } else if self.peek_kind() == Some(&TokenKind::LBracket) {
                self.advance(); // '['
                let index_expr = self.parse_expression(0)?;
                self.consume(TokenKind::RBracket, "Expected ']' after index")?;
                node = ASTNode::IndexAccess(Box::new(node), Box::new(index_expr));
            } else if self.peek_kind() == Some(&TokenKind::PlusPlus) {
                self.advance();
                node = ASTNode::PostfixUnaryExpression("++".to_string(), Box::new(node));
                continue;
            } else if self.peek_kind() == Some(&TokenKind::MinusMinus) {
                self.advance();
                node = ASTNode::PostfixUnaryExpression("--".to_string(), Box::new(node));
                continue;
            } else {
                break;
            }
        }

        Ok(node)
    }

    // -----------------------------------------------------
    // ------------------   HELPERS      -------------------
    // -----------------------------------------------------

    fn match_kind(&mut self, kind: TokenKind) -> bool {
        if self.peek_kind() == Some(&kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, kind: TokenKind, msg: &str) -> Result<(), String> {
        if self.match_kind(kind.clone()) {
            Ok(())
        } else {
            Err(msg.into())
        }
    }

    fn peek_kind(&self) -> Option<&TokenKind> {
        self.tokens.get(self.current).map(|t| &t.kind)
    }

    fn advance(&mut self) -> &Token {
        let tok = &self.tokens[self.current];
        self.current += 1;
        tok
    }

    fn parse_for_statement(&mut self) -> Result<ASTNode, String> {
        self.consume(TokenKind::LParen, "Expected '(' after 'for'")?;
        
        let initializer = if self.peek_kind() != Some(&TokenKind::Semicolon) {
            Some(Box::new(self.parse_statement()?))
        } else {
            self.advance(); // ';'
            None
        };

        let condition = if self.peek_kind() != Some(&TokenKind::Semicolon) {
            Some(Box::new(self.parse_expression(0)?))
        } else {
            None
        };
        self.consume(TokenKind::Semicolon, "Expected ';' after for-loop condition")?;

        let increment = if self.peek_kind() != Some(&TokenKind::RParen) {
            Some(Box::new(self.parse_expression(0)?))
        } else {
            None
        };
        
        self.consume(TokenKind::RParen, "Expected ')' after for-loop increment")?;

        let body = self.parse_block_or_single_statement()?;
        Ok(ASTNode::ForStatement(initializer, condition, increment, Box::new(body)))
    }
}
