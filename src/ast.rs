use crate::tokens::Token;

/// All types of AST nodes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Node {
    Identifier {
        value: Token,
    },
    IntegerLiteral {
        value: i64,
    },
    PrefixExpression {
        operator: String,
        right: Box<Node>,
    },
    InfixExpression {
        left: Box<Node>,
        operator: String,
        right: Box<Node>,
    },
    LetStatement {
        name: Box<Node>,
        value: Option<Box<Node>>,
    },
    ReturnStatement {
        value: Option<Box<Node>>,
    },
    ExpressionStatement {
        expression: Option<Box<Node>>,
    },
}

impl Node {
    pub fn token_literal(&self) -> String {
        match &self {
            Node::Identifier { value } => value.v.clone(),
            Node::IntegerLiteral { value } => value.to_string(),
            Node::PrefixExpression { operator, .. } => operator.clone(),
            Node::InfixExpression { left, operator, .. } => operator.clone(),
            Node::LetStatement { .. } => "let".to_string(),
            Node::ReturnStatement { .. } => "return".to_string(),
            Node::ExpressionStatement { expression } => {
                if let Some(expr) = expression {
                    expr.token_literal()
                } else {
                    "".to_string()
                }
            }
        }
    }

    pub fn as_string(&self) -> String {
        let mut s = String::new();
        match &self {
            Node::Identifier { value } => s.push_str(&value.v),
            Node::IntegerLiteral { value } => s.push_str(&value.to_string()),
            Node::PrefixExpression { operator, right } => {
                s.push('(');
                s.push_str(&operator);
                s.push_str(&*right.as_string());
                s.push(')');
            }
            Node::InfixExpression {
                left,
                operator,
                right,
            } => {
                s.push('(');
                s.push_str(&*left.as_string());
                s.push(' ');
                s.push_str(&operator);
                s.push(' ');
                s.push_str(&*right.as_string());
                s.push(')');
            }
            Node::LetStatement { name, value } => {
                s.push_str(&"let ");
                s.push_str(&name.as_string());
                if let Some(v) = value {
                    s.push_str(&" = ");
                    s.push_str(&v.as_string());
                }

                s.push(';');
            }
            Node::ReturnStatement { value } => {
                s.push_str(&"return");
                if let Some(v) = value {
                    s.push(' ');
                    s.push_str(&v.as_string());
                }

                s.push(';');
            }
            Node::ExpressionStatement { expression } => {
                if let Some(v) = expression {
                    s.push_str(&v.as_string());
                }

                s.push(';');
            }
        };

        s
    }
}

#[derive(Default)]
pub struct Program {
    pub statements: Vec<Node>,
}

impl Program {
    pub fn as_string(&self) -> String {
        let mut s = String::new();
        self.statements.iter().for_each(|st| {
            s.push_str(&st.as_string());
        });

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::TokenType;

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Node::LetStatement {
                name: Box::new(Node::Identifier {
                    value: Token::new(TokenType::Ident, "my_var".to_string()),
                }),
                value: Some(Box::new(Node::Identifier {
                    value: Token::new(TokenType::Ident, "another_var".to_string()),
                })),
            }],
        };

        assert_eq!(program.as_string(), "let my_var = another_var;".to_owned());
    }
}
