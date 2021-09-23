use crate::ast::{Node, Program};
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

    fn parse_statement(&mut self) -> Result<Node, ParserError> {
        match self.curr_token.t {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => Err(ParserError::TokenUnrecognized),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Node, ParserError> {
        if !self.expect_peek(TokenType::Ident) {
            return Err(ParserError::IdentExpected);
        }

        let ident = Node::Identifier {
            value: self.curr_token.clone(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return Err(ParserError::AssignExpected);
        }

        loop {
            self.next_token();
            if self.curr_token.t == TokenType::Semicolon {
                break;
            }
        }

        Ok(Node::LetStatement {
            name: Box::new(ident),
            value: None,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Node, ParserError> {
        loop {
            self.next_token();
            if self.curr_token.t == TokenType::Semicolon {
                break;
            }
        }

        Ok(Node::ReturnStatement { value: None })
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
        if let Node::LetStatement { name, value } = first_statement {
            assert_eq!(String::from("x"), name.token_literal());
            assert!(value.is_none());
        } else {
            panic!("expected let statement");
        }

        let second_statement = iter.next().expect("should contain a statement");
        assert_eq!(String::from("let"), second_statement.token_literal());
        if let Node::LetStatement { name, value } = second_statement {
            assert_eq!(String::from("y"), name.token_literal());
            assert!(value.is_none());
        } else {
            panic!("expected let statement");
        }

        let third_statement = iter.next().expect("should contain a statement");
        assert_eq!(String::from("let"), third_statement.token_literal());
        if let Node::LetStatement { name, value } = third_statement {
            assert_eq!(String::from("z"), name.token_literal());
            assert!(value.is_none());
        } else {
            panic!("expected let statement");
        }
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
        if let Node::ReturnStatement { value } = first_statement {
            assert!(value.is_none());
        } else {
            panic!("expected return statement");
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert!(!did_parser_fail(parser.errors));

        assert_eq!(1, program.statements.len());

        let stmt = program.statements[0].clone();
        let ident = Node::Identifier {
            value: Token::new(TokenType::Ident, "foobar".to_string()),
        };
        assert_eq!(stmt, ident);
        assert_eq!(stmt.token_literal(), "foobar".to_string());
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
