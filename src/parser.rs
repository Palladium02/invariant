use crate::ast::{Item, Program, Statement};
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

    fn expect_item(&mut self) -> Result<Item, ParseError> {
        match self.input.peek() {
            Some((Token::Fn, _)) => self.expect_function(),
            Some(_) => todo!(),
            None => todo!(),
        }
    }

    fn expect_function(&mut self) -> Result<Item, ParseError> {
        let _ = self.expect_token(TokenKind::Fn);
        let name = self.expect_identifier()?;
        let _ = self.expect_token(TokenKind::LParen);
        let arguments = self.expect_arguments();
        let _ = self.expect_token(TokenKind::RParen);
        let body = self.expect_block()?;
        todo!()
    }

    fn expect_arguments(&mut self) -> Result<Vec<String>, ParseError> {
        let mut arguments = Vec::new();

        if let Some((Token::RParen, _)) = self.input.peek() {
            return Ok(arguments);
        }

        let (arg, _) = self.expect_identifier()?;
        arguments.push(arg);

        while let Some((Token::Comma, _)) = self.input.peek() {
            self.input.next();
            let (arg, _) = self.expect_identifier()?;
            arguments.push(arg);
        }

        Ok(arguments)
    }

    fn expect_block(&mut self) -> Result<Statement, ParseError> {
        let (_, start) = self.expect_token(TokenKind::LBrace)?;

        if let Some((Token::RBrace, end)) = self.input.peek() {
            return Ok(Statement::Block {
                body: Vec::new(),
                span: Range::from(start.start..end.end),
            });
        }

        let mut statements = Vec::new();

        while let Some((token, _)) = self.input.peek()
            && token.kind() != TokenKind::RBrace
        {
            let statement = self.expect_statement()?;
            statements.push(statement);
        }

        // eat unconsumed Token::RBrace
        let end = match self.input.next() {
            Some((Token::RBrace, span)) => span,
            _ => return Err(ParseError::UndefinedFailure),
        };

        Ok(Statement::Block {
            body: statements,
            span: Range::from(start.start..end.end),
        })
    }

    fn expect_statement(&mut self) -> Result<Statement, ParseError> {
        match self.input.peek() {
            Some((Token::LBrace, _)) => self.expect_block(),
            Some((Token::Let, _)) => self.expect_binding(),
            Some((Token::Return, _)) => self.expect_return(),
            Some((Token::If, _)) => self.expect_branching(),
            Some((Token::While, _)) => self.expect_while(),
            _ => Err(ParseError::UndefinedFailure),
        }
    }

    fn expect_binding(&mut self) -> Result<Statement, ParseError> {
        todo!()
    }

    fn expect_return(&mut self) -> Result<Statement, ParseError> {
        todo!()
    }

    fn expect_branching(&mut self) -> Result<Statement, ParseError> {
        todo!()
    }

    fn expect_while(&mut self) -> Result<Statement, ParseError> {
        todo!()
    }

    fn expect_identifier(&mut self) -> Result<(String, Range<usize>), ParseError> {
        match self.input.next() {
            Some((Token::Identifier(identifier), span)) => Ok((identifier, span)),
            _ => Err(ParseError::UndefinedFailure),
        }
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
