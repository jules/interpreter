use crate::tokens::{Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    pub input: Peekable<Chars<'a>>,
    pub ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input_string: &'a str) -> Self {
        let mut input = input_string.chars().peekable();
        let initial = input.next().unwrap();
        Self {
            input: input,
            ch: initial,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.eat_whitespace();

        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::Equal, "==".to_string())
                } else {
                    Token::new(TokenType::Assign, self.ch.into())
                }
            }
            '+' => Token::new(TokenType::Plus, self.ch.into()),
            '-' => Token::new(TokenType::Minus, self.ch.into()),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::NotEqual, "!=".to_string())
                } else {
                    Token::new(TokenType::Bang, self.ch.into())
                }
            }
            '*' => Token::new(TokenType::Asterisk, self.ch.into()),
            '/' => Token::new(TokenType::Slash, self.ch.into()),
            '<' => Token::new(TokenType::LessThan, self.ch.into()),
            '>' => Token::new(TokenType::GreaterThan, self.ch.into()),
            ',' => Token::new(TokenType::Comma, self.ch.into()),
            ';' => Token::new(TokenType::Semicolon, self.ch.into()),
            '(' => Token::new(TokenType::LParen, self.ch.into()),
            ')' => Token::new(TokenType::RParen, self.ch.into()),
            '{' => Token::new(TokenType::LBrace, self.ch.into()),
            '}' => Token::new(TokenType::RBrace, self.ch.into()),
            '0' => Token::new(TokenType::EOF, "".into()),
            _ => {
                if is_letter(self.ch) {
                    let v = self.read_ident(is_letter);
                    return Token::from(v.as_str());
                } else if is_digit(self.ch) {
                    let v = self.read_ident(is_digit);
                    return Token::new(TokenType::Int, v);
                }

                Token::new(TokenType::Illegal, self.ch.into())
            }
        };

        self.read_char();
        token
    }

    fn read_char(&mut self) {
        match self.input.next() {
            Some(ch) => self.ch = ch,
            None => self.ch = '0',
        }
    }

    fn peek_char(&mut self) -> char {
        match self.input.peek() {
            Some(ch) => *ch,
            None => '0',
        }
    }

    fn eat_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_ident(&mut self, conditional: fn(char) -> bool) -> String {
        let mut ident = String::new();

        while conditional(self.ch) {
            ident.push(self.ch);
            self.read_char();
        }

        ident
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::tokens::{Token, TokenType};

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;";

        let mut lexer = Lexer::new(input);

        let tokens = {
            let mut tokens = vec![];

            loop {
                let token = lexer.next_token();
                if token.t == TokenType::EOF {
                    break;
                }

                tokens.push(token);
            }

            tokens
        };

        let expected_tokens = vec![
            Token::new(TokenType::Let, String::from("let")),
            Token::new(TokenType::Ident, String::from("five")),
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Int, String::from("5")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Let, String::from("let")),
            Token::new(TokenType::Ident, String::from("ten")),
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Int, String::from("10")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Let, String::from("let")),
            Token::new(TokenType::Ident, String::from("add")),
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Function, String::from("fn")),
            Token::new(TokenType::LParen, String::from("(")),
            Token::new(TokenType::Ident, String::from("x")),
            Token::new(TokenType::Comma, String::from(",")),
            Token::new(TokenType::Ident, String::from("y")),
            Token::new(TokenType::RParen, String::from(")")),
            Token::new(TokenType::LBrace, String::from("{")),
            Token::new(TokenType::Ident, String::from("x")),
            Token::new(TokenType::Plus, String::from("+")),
            Token::new(TokenType::Ident, String::from("y")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::RBrace, String::from("}")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Let, String::from("let")),
            Token::new(TokenType::Ident, String::from("result")),
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Ident, String::from("add")),
            Token::new(TokenType::LParen, String::from("(")),
            Token::new(TokenType::Ident, String::from("five")),
            Token::new(TokenType::Comma, String::from(",")),
            Token::new(TokenType::Ident, String::from("ten")),
            Token::new(TokenType::RParen, String::from(")")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Bang, String::from("!")),
            Token::new(TokenType::Minus, String::from("-")),
            Token::new(TokenType::Slash, String::from("/")),
            Token::new(TokenType::Asterisk, String::from("*")),
            Token::new(TokenType::Int, String::from("5")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Int, String::from("5")),
            Token::new(TokenType::LessThan, String::from("<")),
            Token::new(TokenType::Int, String::from("10")),
            Token::new(TokenType::GreaterThan, String::from(">")),
            Token::new(TokenType::Int, String::from("5")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::If, String::from("if")),
            Token::new(TokenType::LParen, String::from("(")),
            Token::new(TokenType::Int, String::from("5")),
            Token::new(TokenType::LessThan, String::from("<")),
            Token::new(TokenType::Int, String::from("10")),
            Token::new(TokenType::RParen, String::from(")")),
            Token::new(TokenType::LBrace, String::from("{")),
            Token::new(TokenType::Return, String::from("return")),
            Token::new(TokenType::True, String::from("true")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::RBrace, String::from("}")),
            Token::new(TokenType::Else, String::from("else")),
            Token::new(TokenType::LBrace, String::from("{")),
            Token::new(TokenType::Return, String::from("return")),
            Token::new(TokenType::False, String::from("false")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::RBrace, String::from("}")),
            Token::new(TokenType::Int, String::from("10")),
            Token::new(TokenType::Equal, String::from("==")),
            Token::new(TokenType::Int, String::from("10")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Int, String::from("10")),
            Token::new(TokenType::NotEqual, String::from("!=")),
            Token::new(TokenType::Int, String::from("9")),
            Token::new(TokenType::Semicolon, String::from(";")),
        ];

        assert!(tokens
            .iter()
            .zip(expected_tokens.iter())
            .all(|(t, e)| *t == *e));
    }
}
