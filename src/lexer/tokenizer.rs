use crate::constants::token::TokenKind;
use crate::constants::token::Token;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut position = 0;

    while position < input.len() {
        let current_char = input.chars().nth(position).unwrap();

        if current_char.is_whitespace() {
            position += 1;
            continue;
        }

        if current_char.is_digit(10) {
            let start = position;
            while position < input.len() && input.chars().nth(position).unwrap().is_digit(10) {
                position += 1;
            }
            let number: i64 = input[start..position].parse().unwrap();
            tokens.push(Token::new(TokenKind::Number(number), start));
            continue;
        }

        match current_char {
            '+' => tokens.push(Token::new(TokenKind::Plus, position)),
            '-' => tokens.push(Token::new(TokenKind::Minus, position)),
            '*' => tokens.push(Token::new(TokenKind::Star, position)),
            '/' => tokens.push(Token::new(TokenKind::Slash, position)),
            '=' => tokens.push(Token::new(TokenKind::Equal, position)),
            '(' => tokens.push(Token::new(TokenKind::LParen, position)),
            ')' => tokens.push(Token::new(TokenKind::RParen, position)),
            ';' => tokens.push(Token::new(TokenKind::Semicolon, position)),
            'a'..='z' | 'A'..='Z' => {
                let start = position;
                while position < input.len() && input.chars().nth(position).unwrap().is_alphanumeric() {
                    position += 1;
                }
                let identifier = &input[start..position];
                if identifier == "let" {
                    tokens.push(Token::new(TokenKind::Let, start));
                } else {
                    tokens.push(Token::new(TokenKind::Identifier(identifier.to_string()), start));
                }
            }
            _ => tokens.push(Token::new(TokenKind::Unknown(current_char), position)),
        }

        position += 1;
    }

    tokens.push(Token::new(TokenKind::EOF, input.len()));
    tokens
}