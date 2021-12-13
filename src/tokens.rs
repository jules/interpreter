/// All the possible tokens that can be created by the lexer.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TokenType {
    Illegal,
    EOF,
    Ident,
    Int,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LessThan,
    GreaterThan,

    Equal,
    NotEqual,

    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::Illegal
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Token {
    pub t: TokenType,
    pub v: String,
}

impl Token {
    /// Creates a new Token with the given type and string value.
    pub fn new(t: TokenType, v: String) -> Self {
        Self { t, v }
    }
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        match value {
            "fn" => Token {
                t: TokenType::Function,
                v: value.into(),
            },
            "let" => Token {
                t: TokenType::Let,
                v: value.into(),
            },
            "true" => Token {
                t: TokenType::True,
                v: value.into(),
            },
            "false" => Token {
                t: TokenType::False,
                v: value.into(),
            },
            "if" => Token {
                t: TokenType::If,
                v: value.into(),
            },
            "else" => Token {
                t: TokenType::Else,
                v: value.into(),
            },
            "return" => Token {
                t: TokenType::Return,
                v: value.into(),
            },
            _ => Token {
                t: TokenType::Ident,
                v: value.into(),
            },
        }
    }
}
