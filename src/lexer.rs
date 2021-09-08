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
