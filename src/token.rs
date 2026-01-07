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
