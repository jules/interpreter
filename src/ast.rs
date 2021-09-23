use crate::tokens::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn as_string(&self) -> String;
}

pub trait Statement: Node {
    fn name(&self) -> String;
    fn value(&self) -> Option<String>;
}

pub trait Expression: Node {}

#[derive(Default)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        match self.statements.len() {
            0 => String::from(""),
            _ => self.statements[0].token_literal(),
        }
    }

    fn as_string(&self) -> String {
        let mut s = String::new();
        self.statements.iter().for_each(|st| {
            s.push_str(&st.as_string());
        });

        s
    }
}

#[derive(Default)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Option<Box<dyn Expression>>) -> Self {
        Self { token, name, value }
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.v.clone()
    }

    fn as_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.token_literal());
        s.push(' ');
        s.push_str(&self.name());
        s.push_str(&" = ");
        if let Some(v) = self.value() {
            s.push_str(&v);
        }

        s.push(';');
        s
    }
}

impl Statement for LetStatement {
    fn name(&self) -> String {
        self.name.token_literal()
    }

    fn value(&self) -> Option<String> {
        match &self.value {
            Some(v) => Some(v.token_literal()),
            None => None,
        }
    }
}

#[derive(Default)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Option<Box<dyn Expression>>,
}

impl ReturnStatement {
    pub fn new(token: Token, value: Option<Box<dyn Expression>>) -> Self {
        Self { token, value }
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.v.clone()
    }

    fn as_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.token_literal());
        s.push(' ');
        if let Some(v) = self.value() {
            s.push_str(&v);
        }

        s.push(';');
        s
    }
}

impl Statement for ReturnStatement {
    fn name(&self) -> String {
        "".to_owned()
    }

    fn value(&self) -> Option<String> {
        match &self.value {
            Some(v) => Some(v.token_literal()),
            None => None,
        }
    }
}

pub struct ExpressionStatement {
    pub token: Token,
    pub value: Option<Box<Expression>>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.v.clone()
    }

    fn as_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.token_literal());
        if let Some(v) = self.value() {
            s.push_str(&v);
        }

        s.push(';');
        s
    }
}

impl Statement for ExpressionStatement {
    fn name(&self) -> String {
        "".to_owned()
    }

    fn value(&self) -> Option<String> {
        match &self.value {
            Some(v) => Some(v.token_literal()),
            None => None,
        }
    }
}

#[derive(Default)]
pub struct Identifier {
    token: Token,
    v: String,
}

impl Identifier {
    pub fn new(token: Token, v: String) -> Self {
        Self { token, v }
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.v.clone()
    }

    fn as_string(&self) -> String {
        self.v.clone()
    }
}

impl Expression for Identifier {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::TokenType;

    #[test]
    fn test_string() {
        let value = Identifier::new(
            Token::new(TokenType::Ident, "another_var".to_string()),
            "another_var".to_string(),
        );
        let program = Program {
            statements: vec![Box::new(LetStatement {
                token: Token::new(TokenType::Let, "let".to_string()),
                name: Identifier::new(
                    Token::new(TokenType::Ident, "my_var".to_string()),
                    "my_var".to_string(),
                ),
                value: Some(Box::new(value)),
            })],
        };

        assert_eq!(program.as_string(), "let my_var = another_var;".to_owned());
    }
}
