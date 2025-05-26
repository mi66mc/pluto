use crate::constants::token::TokenKind;
use crate::constants::token::Token;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let bytes = input.as_bytes();
    let mut position = 0;

    while position < bytes.len() {
        let current_char = bytes[position] as char;

        // Skip whitespace
        if current_char.is_whitespace() {
            position += 1;
            continue;
        }
        // "/* */" 
        // "*/"
        if current_char == '/' && position + 1 < bytes.len() && bytes[position + 1] as char == '*' {
            position += 2;
            while position + 1 < bytes.len() && !(bytes[position] as char == '*' && bytes[position + 1] as char == '/') {
                position += 1;
            }
            // "*/"
            if position + 1 < bytes.len() {
                position += 2;
            }
            continue;
        }
        // --- End block ---

        if current_char.is_digit(10) {
            let start = position;
            let mut has_dot = false;
            while position < bytes.len() && ((bytes[position] as char).is_digit(10) || (bytes[position] as char) == '.') {
                if (bytes[position] as char) == '.' {
                    if has_dot {
                        break;
                    }
                    has_dot = true;
                }
                position += 1;
            }
            let number_str = &input[start..position];
            if has_dot {
                let number: f64 = number_str.parse().unwrap();
                tokens.push(Token::new(TokenKind::Float(number), start));
            } else {
                let number: i64 = number_str.parse().unwrap();
                tokens.push(Token::new(TokenKind::Number(number), start));
            }
            continue;
        }

        match current_char {
            '[' => tokens.push(Token::new(TokenKind::LBacket, position)),
            ']' => tokens.push(Token::new(TokenKind::RBacket, position)),
            '\"' => {
                let start = position + 1;
                position += 1;
                while position < bytes.len() && bytes[position] as char != '\"' {
                    position += 1;
                }
                if position < bytes.len() {
                    let string_literal = &input[start..position];
                    tokens.push(Token::new(TokenKind::StringLiteral(string_literal.to_string()), start));
                    position += 1;
                } else {
                    tokens.push(Token::new(TokenKind::Unknown('\"'), start));
                }
                continue;
            }
            '+' => tokens.push(Token::new(TokenKind::Plus, position)),
            '-' => tokens.push(Token::new(TokenKind::Minus, position)),
            '*' => tokens.push(Token::new(TokenKind::Star, position)),
            '/' => tokens.push(Token::new(TokenKind::Slash, position)),
            '%' => tokens.push(Token::new(TokenKind::Percent, position)),
            '=' => {
                if position + 1 < bytes.len() && bytes[position + 1] as char == '=' {
                    tokens.push(Token::new(TokenKind::EqualsEqual, position));
                    position += 2;
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::Equal, position));
                }
            }
            '!' => {
                if position + 1 < bytes.len() && bytes[position + 1] as char == '=' {
                    tokens.push(Token::new(TokenKind::NotEqual, position));
                    position += 2;
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::Not, position));
                }
            }
            '<' => {
                if position + 1 < bytes.len() && bytes[position + 1] as char == '=' {
                    tokens.push(Token::new(TokenKind::LessThanEqual, position));
                    position += 2;
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::LessThan, position));
                }
            }
            '>' => {
                if position + 1 < bytes.len() && bytes[position + 1] as char == '=' {
                    tokens.push(Token::new(TokenKind::GreaterThanEqual, position));
                    position += 2;
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::GreaterThan, position));
                }
            }
            '&' => {
                if position + 1 < bytes.len() && bytes[position + 1] as char == '&' {
                    tokens.push(Token::new(TokenKind::And, position));
                    position += 2;
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::Unknown('&'), position));
                }
            }
            '|' => {
                if position + 1 < bytes.len() && bytes[position + 1] as char == '|' {
                    tokens.push(Token::new(TokenKind::Or, position));
                    position += 2;
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::Unknown('|'), position));
                }
            }
            '(' => tokens.push(Token::new(TokenKind::LParen, position)),
            ')' => tokens.push(Token::new(TokenKind::RParen, position)),
            '{' => tokens.push(Token::new(TokenKind::LBrace, position)),
            '}' => tokens.push(Token::new(TokenKind::RBrace, position)),
            ';' => tokens.push(Token::new(TokenKind::Semicolon, position)),
            ',' => tokens.push(Token::new(TokenKind::Comma, position)),
            '.' => tokens.push(Token::new(TokenKind::Dot, position)),
            'a'..='z' | 'A'..='Z' => {
                let start = position;
                while position < bytes.len() && ((bytes[position] as char).is_alphanumeric() || (bytes[position] as char) == '_') {
                    position += 1;
                }
                let identifier = &input[start..position];
                if identifier == "let" {
                    tokens.push(Token::new(TokenKind::Let, start));
                } else if identifier == "while" {
                    tokens.push(Token::new(TokenKind::While, start));
                } else if identifier == "fn" {
                    tokens.push(Token::new(TokenKind::Fn, start));
                } else if identifier == "return" {
                    tokens.push(Token::new(TokenKind::Return, start));
                } else if identifier == "if" {
                    tokens.push(Token::new(TokenKind::If, start));
                } else if identifier == "true" {
                    tokens.push(Token::new(TokenKind::Boolean(true), start));
                } else if identifier == "false" {
                    tokens.push(Token::new(TokenKind::Boolean(false), start));
                } else if identifier == "break" {
                    tokens.push(Token::new(TokenKind::Break, start));
                } else if identifier == "continue" {
                    tokens.push(Token::new(TokenKind::Continue, start));
                } else {
                    tokens.push(Token::new(TokenKind::Identifier(identifier.to_string()), start));
                }
                continue;
            }
            _ => tokens.push(Token::new(TokenKind::Unknown(current_char), position)),
        }

        position += 1;
    }

    tokens.push(Token::new(TokenKind::EOF, input.len()));
    tokens
}