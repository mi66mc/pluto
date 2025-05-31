use crate::constants::token::TokenKind;
use crate::constants::token::Token;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let bytes = input.as_bytes();
    let mut position = 0;

    while position < bytes.len() {
        let current_char = bytes[position] as char;

        // skip whitespace
        if current_char.is_whitespace() {
            position += 1;
            continue;
        }

        if current_char == '/' && position + 1 < bytes.len() && bytes[position + 1] as char == '*' {
            position += 2; // /*
            
            while position + 1 < bytes.len() {
                let curr = bytes[position] as char;
                let next = bytes[position + 1] as char;
                
                if curr == '*' && next == '/' {
                    position += 2; // */
                    break;
                }
                position += 1;
            }
            continue;
        }

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
            '[' => tokens.push(Token::new(TokenKind::LBracket, position)),
            ']' => tokens.push(Token::new(TokenKind::RBracket, position)),
            '\"' => {
                let start = position + 1;
                position += 1;
                let mut string = String::new();
                while position < bytes.len() {
                    let c = bytes[position] as char;
                    if c == '\\' && position + 1 < bytes.len() {
                        let next = bytes[position + 1] as char;
                        match next {
                            '"' => {
                                string.push('"');
                                position += 2;
                            }
                            'n' => {
                                string.push('\n');
                                position += 2;
                            }
                            't' => {
                                string.push('\t');
                                position += 2;
                            }
                            '\\' => {
                                string.push('\\');
                                position += 2;
                            }
                            _ => {
                                string.push(c);
                                position += 1;
                            }
                        }
                    } else if c == '"' {
                        position += 1;
                        break;
                    } else {
                        string.push(c);
                        position += 1;
                    }
                }
                tokens.push(Token::new(TokenKind::StringLiteral(string), start));
                continue;
            }
            // '+' => tokens.push(Token::new(TokenKind::Plus, position)),
            // '-' => tokens.push(Token::new(TokenKind::Minus, position)),
            // '*' => tokens.push(Token::new(TokenKind::Star, position)),
            // '/' => tokens.push(Token::new(TokenKind::Slash, position)),
            // '%' => tokens.push(Token::new(TokenKind::Percent, position)),
            '+' => {
                if position + 1 < bytes.len() && bytes[position + 1] as char == '+' {
                    tokens.push(Token::new(TokenKind::PlusPlus, position));
                    position += 2;
                    continue;
                } else if position + 1 < bytes.len() && bytes[position + 1] as char == '=' {
                    tokens.push(Token::new(TokenKind::PlusEqual, position));
                    position += 2;
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::Plus, position));
                }
            }
            '-' => {
                if position + 1 < bytes.len() && bytes[position + 1] as char == '-' {
                    tokens.push(Token::new(TokenKind::MinusMinus, position));
                    position += 2;
                    continue;
                } else if position + 1 < bytes.len() && bytes[position + 1] as char == '=' {
                    tokens.push(Token::new(TokenKind::MinusEqual, position));
                    position += 2;
                    continue;
                } else if position + 1 < bytes.len() && bytes[position + 1] as char == '>' {
                    tokens.push(Token::new(TokenKind::ArrowFunc, position));
                    position += 2;
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::Minus, position));
                }
            }
            '*' => {
                if position + 1 < bytes.len() && bytes[position + 1] as char == '=' {
                    tokens.push(Token::new(TokenKind::StarEqual, position));
                    position += 2;
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::Star, position));
                }
            }
            '/' => {
                if position + 1 < bytes.len() && bytes[position + 1] as char == '=' {
                    tokens.push(Token::new(TokenKind::SlashEqual, position));
                    position += 2;
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::Slash, position));
                }
            }
            '%' => {
                tokens.push(Token::new(TokenKind::Percent, position));
            }
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
            ':' => tokens.push(Token::new(TokenKind::Colon, position)),
            'a'..='z' | 'A'..='Z' => {
                let start = position;
                while position < bytes.len() && ((bytes[position] as char).is_alphanumeric() || (bytes[position] as char) == '_') {
                    position += 1;
                }
                let identifier = &input[start..position];
                let kind = match identifier {
                    "let" => TokenKind::Let,
                    "const" => TokenKind::Const,
                    "for" => TokenKind::For,
                    "while" => TokenKind::While,
                    "fn" => TokenKind::Fn,
                    "return" => TokenKind::Return,
                    "if" => TokenKind::If,
                    "true" => TokenKind::Boolean(true),
                    "false" => TokenKind::Boolean(false),
                    "break" => TokenKind::Break,
                    "continue" => TokenKind::Continue,
                    "null" => TokenKind::Null,
                    _ => TokenKind::Identifier(identifier.to_string()),
                };
                tokens.push(Token::new(kind, start));
                continue;
            }
            _ => tokens.push(Token::new(TokenKind::Unknown(current_char), position)),
        }

        position += 1;
    }

    tokens.push(Token::new(TokenKind::EOF, input.len()));
    tokens
}