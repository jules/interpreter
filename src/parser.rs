use crate::ast::{Identifier, LetStatement, Program, ReturnStatement, Statement};
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
    pub errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        let mut parser = Self {
            lexer,
            curr_token: Token::default(),
            peek_token: Token::default(),
            errors: vec![],
        };

        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::default();

        while !self.finished() {
            match self.parse_statement() {
                Ok(stmt) => program.statements.push(stmt),
                Err(e) => self.errors.push(e),
            };

            self.next_token();
        }

        program
    }

    fn finished(&self) -> bool {
        self.curr_token.t == TokenType::EOF
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, ParserError> {
        match self.curr_token.t {
            TokenType::Let => Ok(self.parse_let_statement()?),
            TokenType::Return => Ok(self.parse_return_statement()?),
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

    fn parse_return_statement(&mut self) -> Result<Box<dyn Statement>, ParserError> {
        let let_token = self.curr_token.clone();

        loop {
            self.next_token();
            if self.curr_token.t == TokenType::Semicolon {
                break;
            }
        }

        let stmt = ReturnStatement::new(let_token, None);
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
        let z = 838383;";

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert!(!did_parser_fail(parser.errors));

        assert_eq!(3, program.statements.len());

        let mut iter = program.statements.iter();

        let first_statement = iter.next().expect("should contain a statement");
        assert_eq!(String::from("let"), first_statement.token_literal());
        assert_eq!(String::from("x"), first_statement.name());

        let second_statement = iter.next().expect("should contain a statement");
        assert_eq!(String::from("let"), second_statement.token_literal());
        assert_eq!(String::from("y"), second_statement.name());

        let third_statement = iter.next().expect("should contain a statement");
        assert_eq!(String::from("let"), third_statement.token_literal());
        assert_eq!(String::from("z"), third_statement.name());
    }

    #[test]
    fn test_return_statements() {
        let input = "
        return 5;
        return 10;
        return 987235;";

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert!(!did_parser_fail(parser.errors));

        assert_eq!(3, program.statements.len());

        let mut iter = program.statements.iter();

        let first_statement = iter.next().expect("should contain a statement");
        assert_eq!(String::from("return"), first_statement.token_literal());
        assert_eq!(Some(String::from("5")), first_statement.value());
    }

    fn did_parser_fail(errors: Vec<ParserError>) -> bool {
        if errors.len() == 0 {
            false
        } else {
            errors.iter().for_each(|e| {
                println!("{:?}", e);
            });

            true
        }
    }
}
