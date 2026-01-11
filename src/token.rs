#[derive(Debug)]
pub(crate) enum Token {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,

    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AndAnd,
    OrOr,

    Identifier(String),
    Int(i64),

    Fn,
    Let,
    If,
    Else,
    While,
    Return,
    True,
    False,

    UnexpectedCharacter(char),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TokenKind {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,

    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AndAnd,
    OrOr,

    Identifier,
    Int,

    Fn,
    Let,
    If,
    Else,
    While,
    Return,
    True,
    False,

    UnexpectedCharacter,
}

impl Token {
    pub fn kind(&self) -> TokenKind {
        match self {
            Token::LParen => TokenKind::LParen,
            Token::RParen => TokenKind::RParen,
            Token::LBrace => TokenKind::LBrace,
            Token::RBrace => TokenKind::RBrace,
            Token::Comma => TokenKind::Comma,
            Token::Semicolon => TokenKind::Semicolon,
            Token::Plus => TokenKind::Plus,
            Token::Minus => TokenKind::Minus,
            Token::Asterisk => TokenKind::Asterisk,
            Token::Slash => TokenKind::Slash,
            Token::Bang => TokenKind::Bang,
            Token::Equal => TokenKind::Equal,
            Token::EqualEqual => TokenKind::EqualEqual,
            Token::BangEqual => TokenKind::BangEqual,
            Token::Less => TokenKind::Less,
            Token::LessEqual => TokenKind::LessEqual,
            Token::Greater => TokenKind::Greater,
            Token::GreaterEqual => TokenKind::GreaterEqual,
            Token::AndAnd => TokenKind::AndAnd,
            Token::OrOr => TokenKind::OrOr,
            Token::Identifier(_) => TokenKind::Identifier,
            Token::Int(_) => TokenKind::Int,
            Token::Fn => TokenKind::Fn,
            Token::Let => TokenKind::Let,
            Token::If => TokenKind::If,
            Token::Else => TokenKind::Else,
            Token::While => TokenKind::While,
            Token::Return => TokenKind::Return,
            Token::True => TokenKind::True,
            Token::False => TokenKind::False,
            Token::UnexpectedCharacter(_) => TokenKind::UnexpectedCharacter,
        }
    }

    pub fn as_identifier_unchecked(self) -> String {
        match self {
            Token::Identifier(identifier) => identifier,
            _ => unreachable!(),
        }
    }
}
