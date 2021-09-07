#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    Illegal,
    EOF,
    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub t: TokenType,
    pub v: String,
}

impl Token {
    pub fn new(t: TokenType, v: String) -> Self {
        Self { t, v }
    }
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        match value {
            "function" => Token {
                t: TokenType::Function,
                v: value.into(),
            },
            "let" => Token {
                t: TokenType::Let,
                v: value.into(),
            },
            _ => Token {
                t: TokenType::Ident,
                v: value.into(),
            },
        }
    }
}
