use crate::tokens::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {}

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
}

#[derive(Default)]
pub struct LetStatement<'a> {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<&'a dyn Expression>,
}

impl<'a> LetStatement<'a> {
    pub fn new(token: Token, name: Identifier, value: Option<&'a dyn Expression>) -> Self {
        Self { token, name, value }
    }
}

impl Node for LetStatement<'_> {
    fn token_literal(&self) -> String {
        self.token.v.clone()
    }
}

impl Statement for LetStatement<'_> {}

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
}

impl Expression for Identifier {}
