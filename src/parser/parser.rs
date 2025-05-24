use crate::constants::token::{Token, TokenKind, TokenKindTrait};
use crate::parser::ast::ASTNode;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Programa completo (múltiplas declarações/expressões)
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
        } else {
            let expr = self.parse_expression(0)?;
            self.consume(TokenKind::Semicolon, "Expected ';' after expression")?;
            Ok(expr)
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<ASTNode, String> {
        // já consumiu 'let'
        let name = if let Some(TokenKind::Identifier(id)) = self.peek_kind().cloned() {
            self.advance();
            id
        } else {
            return Err("Expected identifier after 'let'".into());
        };

        self.consume(TokenKind::Equal, "Expected '=' after identifier")?;
        let initializer = self.parse_expression(0)?;
        self.consume(TokenKind::Semicolon, "Expected ';' after variable declaration")?;

        Ok(ASTNode::VariableDeclaration(name, Some(Box::new(initializer))))
    }

    // -----------------------------------------------------
    // ------------------   EXPRESSIONS  -------------------
    // -----------------------------------------------------

    fn parse_expression(&mut self, min_prec: u8) -> Result<ASTNode, String> {
        let mut left = self.parse_primary()?;

        while let Some(op_prec) = self.current_precedence() {
            if op_prec < min_prec {
                break;
            }
            let op_token = self.advance().clone();       // consome operador
            let right = self.parse_expression(op_prec + 1)?;
            left = ASTNode::BinaryExpression(
                Box::new(left),
                op_token.kind.to_string(),
                Box::new(right),
            );
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<ASTNode, String> {
        match self.advance().kind.clone() {
            TokenKind::Number(n)     => Ok(ASTNode::NumberLiteral(n)),
            TokenKind::Identifier(s) => {
                // Check for function call: identifier followed by '('
                if self.peek_kind() == Some(&TokenKind::LParen) {
                    self.advance(); // consume '('
                    let mut args = Vec::new();
                    if self.peek_kind() != Some(&TokenKind::RParen) {
                        loop {
                            let arg = self.parse_expression(0)?;
                            args.push(Box::new(arg));
                            if self.peek_kind() == Some(&TokenKind::RParen) {
                                break;
                            }
                            self.consume(TokenKind::Comma, "Expected ',' or ')' in argument list")?;
                        }
                    }
                    self.consume(TokenKind::RParen, "Expected ')' after arguments")?;
                    Ok(ASTNode::FunctionCall(s, args))
                } else {
                    Ok(ASTNode::Identifier(s))
                }
            }
            TokenKind::LParen        => {
                let expr = self.parse_expression(0)?;
                self.consume(TokenKind::RParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            other => Err(format!("Unexpected token: {:?}", other)),
        }
    }

    // -----------------------------------------------------
    // ------------------   HELPERS      -------------------
    // -----------------------------------------------------

    fn current_precedence(&self) -> Option<u8> {
        match self.peek_kind()? {
            TokenKind::Plus | TokenKind::Minus => Some(1),
            TokenKind::Star | TokenKind::Slash => Some(2),
            _ => None,
        }
    }

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
}
