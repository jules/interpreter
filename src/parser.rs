use crate::ast::{Identifier, LetStatement, Program, Statement};
use crate::lexer::Lexer;
use crate::tokens::{Token, TokenType};

#[derive(Debug)]
pub enum ParserError {
    TokenUnrecognized,
    IdentExpected,
    AssignExpected,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        let mut parser = Self {
            lexer,
            curr_token: Token::default(),
            peek_token: Token::default(),
        };

        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut program = Program::default();

        while !self.finished() {
            let stmt = self.parse_statement()?;
            program.statements.push(stmt);
            self.next_token();
        }

        Ok(program)
    }

    fn finished(&self) -> bool {
        self.curr_token.t == TokenType::EOF
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, ParserError> {
        match self.curr_token.t {
            TokenType::Let => Ok(self.parse_let_statement()?),
            _ => Err(ParserError::TokenUnrecognized),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Box<dyn Statement>, ParserError> {
        let let_token = self.curr_token.clone();

        if !self.expect_peek(TokenType::Ident) {
            return Err(ParserError::IdentExpected);
        }

        let ident = Identifier::new(self.curr_token.clone(), self.curr_token.v.clone());

        if !self.expect_peek(TokenType::Assign) {
            return Err(ParserError::AssignExpected);
        }

        loop {
            self.next_token();
            if self.curr_token.t == TokenType::Semicolon {
                break;
            }
        }

        let stmt = LetStatement::new(let_token, ident, None);
        Ok(Box::new(stmt))
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token.t == token_type {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;";

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser
            .parse_program()
            .expect("should have parsed program correctly");

        assert_eq!(3, program.statements.len());

        let mut iter = program.statements.iter();
        let first_statement = iter.next().expect("should contain a statement");

        assert_eq!(String::from("let"), first_statement.token_literal());
        // assert_eq!(String::from("x"), first_statement.name.token_literal());
    }
}
