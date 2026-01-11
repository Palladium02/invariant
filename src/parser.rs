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
        let _ = self.expect_token(TokenKind::Fn);
        let name = self.expect_token(TokenKind::Identifier);
        let _ = self.expect_token(TokenKind::LParen);
        let arguments = self.expect_arguments();
        let _ = self.expect_token(TokenKind::RParen);
        todo!()
    }

    fn expect_arguments(&mut self) -> Result<Vec<String>, ParseError> {
        let mut arguments = Vec::new();

        if let Some((Token::RParen, _)) = self.input.peek() {
            return Ok(arguments);
        }

        let (arg, _) = self.expect_token(TokenKind::Identifier)?;
        arguments.push(arg.as_identifier_unchecked());

        while let Some((Token::Comma, _)) = self.input.peek() {
            self.input.next();
            let (arg, _) = self.expect_token(TokenKind::Identifier)?;
            arguments.push(arg.as_identifier_unchecked())
        }

        Ok(arguments)
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
