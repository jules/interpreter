use crate::ast::{Node, Program};
use crate::lexer::Lexer;
use crate::tokens::{Token, TokenType};

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

#[derive(Debug)]
pub enum ParserError {
    TokenUnrecognized,
    IdentExpected,
    AssignExpected,
    IntegerParsingFailed,
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
            _ => self.parse_expression_statement(),
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

        self.peek_until_semicolon();

        Ok(Node::LetStatement {
            name: Box::new(ident),
            value: None,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Node, ParserError> {
        self.peek_until_semicolon();

        Ok(Node::ReturnStatement { value: None })
    }

    fn parse_expression_statement(&mut self) -> Result<Node, ParserError> {
        let expr = Node::ExpressionStatement {
            token: self.curr_token.clone(),
            expression: Some(Box::new(self.parse_expression(Precedence::Lowest)?)),
        };

        self.peek_until_semicolon();
        Ok(expr)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Node, ParserError> {
        let mut left_exp = match self.curr_token.t {
            TokenType::Ident => Ok(Node::Identifier {
                value: self.curr_token.clone(),
            }),
            TokenType::Int => self.parse_integer_literal(),
            TokenType::Minus | TokenType::Bang => self.parse_prefix_expression(),
            _ => Err(ParserError::TokenUnrecognized),
        }?;

        while self.peek_token.t != TokenType::Semicolon && precedence < self.check_peek_precedence()
        {
            if !self.should_keep_parsing() {
                return Ok(left_exp);
            }

            self.next_token();
            left_exp = self.parse_infix_expression(left_exp)?;
        }

        Ok(left_exp)
    }

    fn parse_integer_literal(&mut self) -> Result<Node, ParserError> {
        Ok(Node::IntegerLiteral {
            value: self
                .curr_token
                .v
                .parse()
                .map_err(|_| ParserError::IntegerParsingFailed)?,
        })
    }

    fn parse_prefix_expression(&mut self) -> Result<Node, ParserError> {
        let prefix_token = self.curr_token.clone();

        self.next_token();
        Ok(Node::PrefixExpression {
            operator: prefix_token.v.clone(),
            right: Box::new(self.parse_expression(Precedence::Prefix)?),
        })
    }

    fn parse_infix_expression(&mut self, left: Node) -> Result<Node, ParserError> {
        let operator = self.curr_token.clone();

        let precedence = self.check_curr_precedence();
        self.next_token();
        Ok(Node::InfixExpression {
            left: Box::new(left),
            operator: operator.v,
            right: Box::new(self.parse_expression(precedence)?),
        })
    }

    fn check_curr_precedence(&mut self) -> Precedence {
        match self.curr_token.t {
            TokenType::Equal | TokenType::NotEqual => Precedence::Equals,
            TokenType::LessThan | TokenType::GreaterThan => Precedence::LessGreater,
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Slash | TokenType::Asterisk => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }

    fn check_peek_precedence(&mut self) -> Precedence {
        match self.peek_token.t {
            TokenType::Equal | TokenType::NotEqual => Precedence::Equals,
            TokenType::LessThan | TokenType::GreaterThan => Precedence::LessGreater,
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Slash | TokenType::Asterisk => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }

    fn peek_until_semicolon(&mut self) {
        loop {
            self.next_token();
            if self.curr_token.t == TokenType::Semicolon {
                break;
            }
        }
    }

    fn should_keep_parsing(&mut self) -> bool {
        match self.peek_token.t {
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Slash
            | TokenType::Asterisk
            | TokenType::Equal
            | TokenType::NotEqual
            | TokenType::LessThan
            | TokenType::GreaterThan => true,
            _ => false,
        }
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
        let ident = Node::ExpressionStatement {
            token: Token::new(TokenType::Ident, "foobar".to_string()),
            expression: Some(Box::new(Node::Identifier {
                value: Token::new(TokenType::Ident, "foobar".to_string()),
            })),
        };
        assert_eq!(stmt, ident);
        assert_eq!(stmt.token_literal(), "foobar".to_string());
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert!(!did_parser_fail(parser.errors));

        assert_eq!(1, program.statements.len());

        let stmt = program.statements[0].clone();
        let ident = Node::ExpressionStatement {
            token: Token::new(TokenType::Int, "5".to_string()),
            expression: Some(Box::new(Node::IntegerLiteral { value: 5 })),
        };
        assert_eq!(stmt, ident);
        assert_eq!(stmt.token_literal(), "5".to_string());
    }

    #[test]
    fn test_prefix_expression() {
        let input = "
            !5;
            -15;";

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert!(!did_parser_fail(parser.errors));

        assert_eq!(2, program.statements.len());

        let mut iter = program.statements.into_iter();
        let stmt = iter.next().unwrap();
        let ident = Node::ExpressionStatement {
            token: Token::new(TokenType::Bang, "!".to_string()),
            expression: Some(Box::new(Node::PrefixExpression {
                operator: "!".to_string(),
                right: Box::new(Node::IntegerLiteral { value: 5 }),
            })),
        };
        assert_eq!(stmt, ident);
        assert_eq!(stmt.token_literal(), "!".to_string());

        let stmt = iter.next().unwrap();
        let ident = Node::ExpressionStatement {
            token: Token::new(TokenType::Minus, "-".to_string()),
            expression: Some(Box::new(Node::PrefixExpression {
                operator: "-".to_string(),
                right: Box::new(Node::IntegerLiteral { value: 15 }),
            })),
        };
        assert_eq!(stmt, ident);
        assert_eq!(stmt.token_literal(), "-".to_string());
    }

    #[test]
    fn test_infix_expression() {
        let input = "
            5 + 5;
            5 - 5;
            5 * 5;
            5 / 5;
            5 > 5;
            5 < 5;
            5 == 5;
            5 != 5;";

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert!(!did_parser_fail(parser.errors));

        assert_eq!(8, program.statements.len());

        let mut iter = program.statements.into_iter();
        let stmt = iter.next().unwrap();
        let ident = Node::ExpressionStatement {
            token: Token::new(TokenType::Int, "5".to_string()),
            expression: Some(Box::new(Node::InfixExpression {
                left: Box::new(Node::IntegerLiteral { value: 5 }),
                operator: "+".to_string(),
                right: Box::new(Node::IntegerLiteral { value: 5 }),
            })),
        };
        assert_eq!(stmt, ident);
        assert_eq!(stmt.token_literal(), "5".to_string());

        let stmt = iter.next().unwrap();
        let ident = Node::ExpressionStatement {
            token: Token::new(TokenType::Int, "5".to_string()),
            expression: Some(Box::new(Node::InfixExpression {
                left: Box::new(Node::IntegerLiteral { value: 5 }),
                operator: "-".to_string(),
                right: Box::new(Node::IntegerLiteral { value: 5 }),
            })),
        };
        assert_eq!(stmt, ident);
        assert_eq!(stmt.token_literal(), "5".to_string());

        let stmt = iter.next().unwrap();
        let ident = Node::ExpressionStatement {
            token: Token::new(TokenType::Int, "5".to_string()),
            expression: Some(Box::new(Node::InfixExpression {
                left: Box::new(Node::IntegerLiteral { value: 5 }),
                operator: "*".to_string(),
                right: Box::new(Node::IntegerLiteral { value: 5 }),
            })),
        };
        assert_eq!(stmt, ident);
        assert_eq!(stmt.token_literal(), "5".to_string());

        let stmt = iter.next().unwrap();
        let ident = Node::ExpressionStatement {
            token: Token::new(TokenType::Int, "5".to_string()),
            expression: Some(Box::new(Node::InfixExpression {
                left: Box::new(Node::IntegerLiteral { value: 5 }),
                operator: "/".to_string(),
                right: Box::new(Node::IntegerLiteral { value: 5 }),
            })),
        };
        assert_eq!(stmt, ident);
        assert_eq!(stmt.token_literal(), "5".to_string());

        let stmt = iter.next().unwrap();
        let ident = Node::ExpressionStatement {
            token: Token::new(TokenType::Int, "5".to_string()),
            expression: Some(Box::new(Node::InfixExpression {
                left: Box::new(Node::IntegerLiteral { value: 5 }),
                operator: ">".to_string(),
                right: Box::new(Node::IntegerLiteral { value: 5 }),
            })),
        };
        assert_eq!(stmt, ident);
        assert_eq!(stmt.token_literal(), "5".to_string());

        let stmt = iter.next().unwrap();
        let ident = Node::ExpressionStatement {
            token: Token::new(TokenType::Int, "5".to_string()),
            expression: Some(Box::new(Node::InfixExpression {
                left: Box::new(Node::IntegerLiteral { value: 5 }),
                operator: "<".to_string(),
                right: Box::new(Node::IntegerLiteral { value: 5 }),
            })),
        };
        assert_eq!(stmt, ident);
        assert_eq!(stmt.token_literal(), "5".to_string());

        let stmt = iter.next().unwrap();
        let ident = Node::ExpressionStatement {
            token: Token::new(TokenType::Int, "5".to_string()),
            expression: Some(Box::new(Node::InfixExpression {
                left: Box::new(Node::IntegerLiteral { value: 5 }),
                operator: "==".to_string(),
                right: Box::new(Node::IntegerLiteral { value: 5 }),
            })),
        };
        assert_eq!(stmt, ident);
        assert_eq!(stmt.token_literal(), "5".to_string());

        let stmt = iter.next().unwrap();
        let ident = Node::ExpressionStatement {
            token: Token::new(TokenType::Int, "5".to_string()),
            expression: Some(Box::new(Node::InfixExpression {
                left: Box::new(Node::IntegerLiteral { value: 5 }),
                operator: "!=".to_string(),
                right: Box::new(Node::IntegerLiteral { value: 5 }),
            })),
        };
        assert_eq!(stmt, ident);
        assert_eq!(stmt.token_literal(), "5".to_string());
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
