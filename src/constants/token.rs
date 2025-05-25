#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier(String),
    Boolean(bool),
    Number(i64),
    Float(f64),
    StringLiteral(String),
    Dot,
    Plus,
    Minus,
    Percent,
    Star,
    Slash,
    Equal,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBacket,
    RBacket,
    EqualsEqual,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    And,
    Or,
    Not,
    Semicolon,
    Comma,
    Let,
    If,
    EOF,
    Unknown(char),
}

pub trait TokenKindTrait {
    fn to_string(&self) -> String;
}

impl TokenKindTrait for TokenKind {
    fn to_string(&self) -> String {
        match self {
            TokenKind::EqualsEqual => "==".to_string(),
            TokenKind::NotEqual => "!=".to_string(),
            TokenKind::LessThan => "<".to_string(),
            TokenKind::GreaterThan => ">".to_string(),
            TokenKind::LessThanEqual => "<=".to_string(),
            TokenKind::GreaterThanEqual => ">=".to_string(),
            TokenKind::And => "&&".to_string(),
            TokenKind::Or => "||".to_string(),
            TokenKind::Not => "!".to_string(),
            TokenKind::Identifier(name) => name.clone(),
            TokenKind::Number(value) => value.to_string(),
            TokenKind::Float(value) => value.to_string(),
            TokenKind::Boolean(value) => value.to_string(),
            TokenKind::Dot => ".".to_string(),
            TokenKind::Plus => "+".to_string(),
            TokenKind::Minus => "-".to_string(),
            TokenKind::Percent => "%".to_string(),
            TokenKind::Star => "*".to_string(),
            TokenKind::Slash => "/".to_string(),
            TokenKind::Equal => "=".to_string(),
            TokenKind::LParen => "(".to_string(),
            TokenKind::RParen => ")".to_string(),
            TokenKind::LBrace => "{".to_string(),
            TokenKind::RBrace => "}".to_string(),
            TokenKind::LBacket => "[".to_string(),
            TokenKind::RBacket => "]".to_string(),
            TokenKind::Semicolon => ";".to_string(),
            TokenKind::Comma => ",".to_string(),
            TokenKind::Let => "let".to_string(),
            TokenKind::If => "if".to_string(),
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

    // pub fn is_identifier(&self) -> bool {
    //     matches!(self.kind, TokenKind::Identifier(_))
    // }

    // pub fn is_number(&self) -> bool {
    //     matches!(self.kind, TokenKind::Number(_))
    // }

    // pub fn is_operator(&self) -> bool {
    //     matches!(
    //         self.kind,
    //         TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash
    //     )
    // }
}