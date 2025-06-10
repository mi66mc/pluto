#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier(String),
    Boolean(bool),
    Number(i64),
    Float(f64),
    StringLiteral(String),
    Null,
    Dot,
    DotDot,
    DotDotEqual,
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
    LBracket,
    RBracket,
    EqualsEqual,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    PlusPlus,
    MinusMinus,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    ArrowFunc,
    And,
    Or,
    Not,
    Semicolon,
    Comma,
    Let,
    Const,
    Fn,
    While,
    For,
    Break,
    Continue,
    Return,
    If,
    Else,
    Match,
    QuestionMark,
    EOF,
    Unknown(char),
    Colon,
    Underscore,
}

pub trait TokenKindTrait {
    fn to_string(&self) -> String;
}

impl TokenKindTrait for TokenKind {
    fn to_string(&self) -> String {
        match self {
            TokenKind::Match => "match".to_string(),
            TokenKind::QuestionMark => "?".to_string(),
            TokenKind::Else => "else".to_string(),
            TokenKind::Const => "const".to_string(),
            TokenKind::Null => "null".to_string(),
            TokenKind::ArrowFunc => "->".to_string(),
            TokenKind::PlusPlus => "++".to_string(),
            TokenKind::MinusMinus => "--".to_string(),
            TokenKind::PlusEqual => "+=".to_string(),
            TokenKind::MinusEqual => "-=".to_string(),
            TokenKind::StarEqual => "*=".to_string(),
            TokenKind::SlashEqual => "/=".to_string(),
            TokenKind::For => "for".to_string(),
            TokenKind::Break => "break".to_string(),
            TokenKind::Continue => "continue".to_string(),
            TokenKind::While => "while".to_string(),
            TokenKind::Return => "return".to_string(),
            TokenKind::Fn => "fn".to_string(),
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
            TokenKind::DotDot => "..".to_string(),
            TokenKind::DotDotEqual => "..=".to_string(),
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
            TokenKind::LBracket => "[".to_string(),
            TokenKind::RBracket => "]".to_string(),
            TokenKind::Semicolon => ";".to_string(),
            TokenKind::Comma => ",".to_string(),
            TokenKind::Let => "let".to_string(),
            TokenKind::If => "if".to_string(),
            TokenKind::StringLiteral(value) => format!("\"{}\"", value),
            TokenKind::EOF => "EOF".to_string(),
            TokenKind::Unknown(c) => c.to_string(),
            TokenKind::Colon => ":".to_string(),
            TokenKind::Underscore => "_".to_string(),
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