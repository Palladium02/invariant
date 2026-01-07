use crate::token::Token;
use std::iter::Peekable;
use std::range::Range;
use std::str::Chars;

pub(crate) struct Lexer<'c> {
    input: Peekable<Chars<'c>>,
    position: usize,
}

impl<'c> Lexer<'c> {
    pub fn new(input: &'c str) -> Self {
        Self {
            input: input.chars().peekable(),
            position: 0,
        }
    }

    fn next_token(&mut self) -> Option<(Token, Range<usize>)> {
        let current_position = self.position;
        match self.next_char()? {
            '(' => self.emit_token(current_position, Token::LParen),
            ')' => self.emit_token(current_position, Token::RParen),
            '{' => self.emit_token(current_position, Token::LBrace),
            '}' => self.emit_token(current_position, Token::RBrace),
            ',' => self.emit_token(current_position, Token::Comma),
            ';' => self.emit_token(current_position, Token::Semicolon),
            '+' => self.emit_token(current_position, Token::Plus),
            '-' => self.emit_token(current_position, Token::Minus),
            '*' => self.emit_token(current_position, Token::Asterisk),
            '/' => self.emit_token(current_position, Token::Slash),
            '!' => {
                if let Some(_) = self.next_char_if(|c| c == '=') {
                    return self.emit_token(current_position, Token::BangEqual);
                }

                self.emit_token(current_position, Token::Bang)
            }
            '=' => {
                if let Some(_) = self.next_char_if(|c| c == '=') {
                    return self.emit_token(current_position, Token::EqualEqual);
                }

                self.emit_token(current_position, Token::Equal)
            }
            '<' => {
                if let Some(_) = self.next_char_if(|c| c == '=') {
                    return self.emit_token(current_position, Token::LessEqual);
                }

                self.emit_token(current_position, Token::Less)
            }
            '>' => {
                if let Some(_) = self.next_char_if(|c| c == '=') {
                    return self.emit_token(current_position, Token::GreaterEqual);
                }

                self.emit_token(current_position, Token::Greater)
            }
            '&' => {
                if let Some(_) = self.next_char_if(|c| c == '&') {
                    return self.emit_token(current_position, Token::AndAnd);
                }

                self.emit_token(current_position, Token::UnexpectedCharacter('&'))
            }
            '|' => {
                if let Some(_) = self.next_char_if(|c| c == '|') {
                    return self.emit_token(current_position, Token::OrOr);
                }

                self.emit_token(current_position, Token::UnexpectedCharacter('|'))
            }
            c if c.is_alphabetic() => {
                let mut identifier = String::from(c);
                while let Some(c) = self.next_char_if(|c| c.is_alphanumeric() || c == '_') {
                    identifier.push(c);
                }

                match identifier.as_str() {
                    "fn" => self.emit_token(current_position, Token::Fn),
                    "let" => self.emit_token(current_position, Token::Let),
                    "if" => self.emit_token(current_position, Token::If),
                    "else" => self.emit_token(current_position, Token::Else),
                    "while" => self.emit_token(current_position, Token::While),
                    "return" => self.emit_token(current_position, Token::Return),
                    "true" => self.emit_token(current_position, Token::True),
                    "false" => self.emit_token(current_position, Token::False),
                    _ => self.emit_token(current_position, Token::Identifier(identifier)),
                }
            }
            c if c.is_numeric() => {
                let mut number = String::from(c);
                while let Some(c) = self.next_char_if(|c| c.is_numeric()) {
                    number.push(c);
                }

                self.emit_token(
                    current_position,
                    Token::Int(
                        number
                            .parse()
                            .expect("This cannot fail due to how we constructed `number`"),
                    ),
                )
            }
            c if c.is_whitespace() => {
                while self.next_char_if(|c| c.is_whitespace()).is_some() {}
                self.next_token()
            }
            _ => todo!(),
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.next_char_if(|_| true)
    }

    fn next_char_if(&mut self, f: impl FnOnce(char) -> bool) -> Option<char> {
        self.input.next_if(|&c| f(c)).inspect(|c| {
            self.position += c.len_utf8();
        })
    }

    fn emit_token(&self, start: usize, token: Token) -> Option<(Token, Range<usize>)> {
        Some((token, Range::from(start..self.position)))
    }
}

impl<'c> Iterator for Lexer<'c> {
    type Item = (Token, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
