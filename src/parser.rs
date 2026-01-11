use crate::ast::{Item, Program};
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};
use std::iter::Peekable;
use std::range::Range;

pub enum ParseError {
    UndefinedFailure,
}

pub(crate) struct Parser<'t> {
    input: Peekable<Lexer<'t>>,
}

impl<'t> Parser<'t> {
    pub fn new(input: Lexer<'t>) -> Self {
        Self {
            input: input.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut items = Vec::new();

        while !self.is_eof() {
            items.push(
                self.expect_item()
                    .map_err(|_| ParseError::UndefinedFailure)?,
            );
        }

        Ok(Program::new(items))
    }

    fn expect_item(&mut self) -> Result<Item, ()> {
        match self.input.peek() {
            Some((Token::Fn, _)) => self.expect_function(),
            Some(_) => todo!(),
            None => todo!(),
        }
    }

    fn expect_function(&mut self) -> Result<Item, ()> {
        todo!()
    }

    fn expect_token(&mut self, kind: TokenKind) -> Result<(Token, Range<usize>), ParseError> {
        match self.input.next() {
            Some((token, span)) if token.kind() == kind => Ok((token, span)),
            Some((token, span)) => Err(ParseError::UndefinedFailure),
            None => Err(ParseError::UndefinedFailure),
        }
    }

    fn is_eof(&mut self) -> bool {
        if let None = self.input.peek() {
            true
        } else {
            false
        }
    }
}
