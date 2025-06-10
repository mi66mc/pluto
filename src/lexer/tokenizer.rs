use crate::constants::token::TokenKind;
use crate::constants::token::Token;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut position = 0;

    while position < chars.len() {
        let current_char = chars[position];

        // skip whitespace
        if current_char.is_whitespace() {
            position += 1;
            continue;
        }

        if current_char == '/' && position + 1 < chars.len() && chars[position + 1] == '*' {
            position += 2; // /*
            
            while position + 1 < chars.len() {
                let curr = chars[position];
                let next = chars[position + 1];
                
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
            let mut dot_position = 0;
            
            while position < chars.len() && (chars[position].is_digit(10) || chars[position] == '.') {
                if chars[position] == '.' {
                    if position + 1 < chars.len() && chars[position + 1] == '.' {
                        break;
                    }
                    if has_dot {
                        break;
                    }
                    has_dot = true;
                    dot_position = position;
                }
                position += 1;
            }

            let number_str = chars[start..position].iter().collect::<String>();
            if has_dot && dot_position + 1 < position {
                let number: f64 = number_str.parse().unwrap();
                tokens.push(Token::new(TokenKind::Float(number), start));
            } else {
                let end = if has_dot { dot_position } else { position };
                let number_str = chars[start..end].iter().collect::<String>();
                let number: i64 = number_str.parse().unwrap();
                tokens.push(Token::new(TokenKind::Number(number), start));
                
                if has_dot {
                    position = dot_position + 1;
                    tokens.push(Token::new(TokenKind::Dot, dot_position));
                }
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
                while position < chars.len() {
                    let c = chars[position];
                    if c == '\\' && position + 1 < chars.len() {
                        let next = chars[position + 1];
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
            '+' => {
                if position + 1 < chars.len() && chars[position + 1] == '+' {
                    tokens.push(Token::new(TokenKind::PlusPlus, position));
                    position += 2;
                    continue;
                } else if position + 1 < chars.len() && chars[position + 1] == '=' {
                    tokens.push(Token::new(TokenKind::PlusEqual, position));
                    position += 2;
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::Plus, position));
                }
            }
            '-' => {
                if position + 1 < chars.len() && chars[position + 1] == '-' {
                    tokens.push(Token::new(TokenKind::MinusMinus, position));
                    position += 2;
                    continue;
                } else if position + 1 < chars.len() && chars[position + 1] == '=' {
                    tokens.push(Token::new(TokenKind::MinusEqual, position));
                    position += 2;
                    continue;
                } else if position + 1 < chars.len() && chars[position + 1] == '>' {
                    tokens.push(Token::new(TokenKind::ArrowFunc, position));
                    position += 2;
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::Minus, position));
                }
            }
            '*' => {
                if position + 1 < chars.len() && chars[position + 1] == '=' {
                    tokens.push(Token::new(TokenKind::StarEqual, position));
                    position += 2;
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::Star, position));
                }
            }
            '/' => {
                if position + 1 < chars.len() && chars[position + 1] == '=' {
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
                if position + 1 < chars.len() && chars[position + 1] == '=' {
                    tokens.push(Token::new(TokenKind::EqualsEqual, position));
                    position += 2;
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::Equal, position));
                }
            }
            '!' => {
                if position + 1 < chars.len() && chars[position + 1] == '=' {
                    tokens.push(Token::new(TokenKind::NotEqual, position));
                    position += 2;
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::Not, position));
                }
            }
            '<' => {
                if position + 1 < chars.len() && chars[position + 1] == '=' {
                    tokens.push(Token::new(TokenKind::LessThanEqual, position));
                    position += 2;
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::LessThan, position));
                }
            }
            '>' => {
                if position + 1 < chars.len() && chars[position + 1] == '=' {
                    tokens.push(Token::new(TokenKind::GreaterThanEqual, position));
                    position += 2;
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::GreaterThan, position));
                }
            }
            '&' => {
                if position + 1 < chars.len() && chars[position + 1] == '&' {
                    tokens.push(Token::new(TokenKind::And, position));
                    position += 2;
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::Unknown('&'), position));
                }
            }
            '|' => {
                if position + 1 < chars.len() && chars[position + 1] == '|' {
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
            '.' => {
                if position + 1 < chars.len() {
                    if chars[position + 1] == '.' {
                        if position + 2 < chars.len() && chars[position + 2] == '=' {
                            tokens.push(Token::new(TokenKind::DotDotEqual, position));
                            position += 3;
                        } else {
                            tokens.push(Token::new(TokenKind::DotDot, position));
                            position += 2;
                        }
                    } else {
                        tokens.push(Token::new(TokenKind::Dot, position));
                        position += 1;
                    }
                } else {
                    tokens.push(Token::new(TokenKind::Dot, position));
                    position += 1;
                }
                continue;
            }
            ':' => tokens.push(Token::new(TokenKind::Colon, position)),
            '?' => tokens.push(Token::new(TokenKind::QuestionMark, position)),
            '_' => {
                if position + 1 < chars.len() && 
                   (chars[position + 1].is_alphanumeric() || chars[position + 1] == '_') {
                    let start = position;
                    while position < chars.len() && 
                          (chars[position].is_alphanumeric() || chars[position] == '_') {
                        position += 1;
                    }
                    let identifier = chars[start..position].iter().collect::<String>();
                    tokens.push(Token::new(TokenKind::Identifier(identifier), start));
                    continue;
                } else {
                    tokens.push(Token::new(TokenKind::Underscore, position));
                }
            },
            'a'..='z' | 'A'..='Z' => {
                let start = position;
                while position < chars.len() && (chars[position].is_alphanumeric() || chars[position] == '_') {
                    position += 1;
                }
                let identifier = chars[start..position].iter().collect::<String>();
                let kind = match identifier.as_str() {
                    "let" => TokenKind::Let,
                    "const" => TokenKind::Const,
                    "for" => TokenKind::For,
                    "while" => TokenKind::While,
                    "fn" => TokenKind::Fn,
                    "return" => TokenKind::Return,
                    "if" => TokenKind::If,
                    "else" => TokenKind::Else,
                    "true" => TokenKind::Boolean(true),
                    "false" => TokenKind::Boolean(false),
                    "break" => TokenKind::Break,
                    "continue" => TokenKind::Continue,
                    "null" => TokenKind::Null,
                    "match" => TokenKind::Match,
                    _ => TokenKind::Identifier(identifier),
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