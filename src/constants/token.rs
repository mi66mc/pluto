#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier(String),
    Number(i64),
    Float(f64),
    StringLiteral(String),
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    LParen,
    RParen,
    Semicolon,
    Comma,
    Let,
    EOF,
    Unknown(char),
}

pub trait TokenKindTrait {
    fn to_string(&self) -> String;
}

impl TokenKindTrait for TokenKind {
    fn to_string(&self) -> String {
        match self {
            TokenKind::Identifier(name) => name.clone(),
            TokenKind::Number(value) => value.to_string(),
            TokenKind::Float(value) => value.to_string(),
            TokenKind::Plus => "+".to_string(),
            TokenKind::Minus => "-".to_string(),
            TokenKind::Star => "*".to_string(),
            TokenKind::Slash => "/".to_string(),
            TokenKind::Equal => "=".to_string(),
            TokenKind::LParen => "(".to_string(),
            TokenKind::RParen => ")".to_string(),
            TokenKind::Semicolon => ";".to_string(),
            TokenKind::Comma => ",".to_string(),
            TokenKind::Let => "let".to_string(),
            TokenKind::StringLiteral(value) => format!("\"{}\"", value),
            TokenKind::EOF => "EOF".to_string(),
            TokenKind::Unknown(c) => c.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub position: usize,
}

impl Token {
    pub fn new(kind: TokenKind, position: usize) -> Self {
        Token { kind, position }
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self.kind, TokenKind::Identifier(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self.kind, TokenKind::Number(_))
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash
        )
    }
}