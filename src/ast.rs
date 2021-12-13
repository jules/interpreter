use crate::tokens::Token;

/// All types of AST nodes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Node {
    Program {
        statements: Vec<Node>,
    },
    Identifier {
        value: Token,
    },
    IntegerLiteral {
        value: i64,
    },
    Boolean {
        value: bool,
    },
    FunctionLiteral {
        parameters: Vec<Node>,
        body: Box<Node>,
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
    IfExpression {
        condition: Box<Node>,
        consequence: Box<Node>,
        alternative: Option<Box<Node>>,
    },
    CallExpression {
        function: Box<Node>,
        arguments: Vec<Node>,
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
    BlockStatement {
        statements: Vec<Node>,
    },
}

impl Node {
    /// Returns the token literal for an AST node.
    pub fn token_literal(&self) -> String {
        match &self {
            Node::Program { .. } => "program".to_string(),
            Node::Identifier { value } => value.v.clone(),
            Node::IntegerLiteral { value } => value.to_string(),
            Node::Boolean { value } => value.to_string(),
            Node::FunctionLiteral { .. } => "fn".to_string(),
            Node::PrefixExpression { operator, .. } => operator.clone(),
            Node::InfixExpression {
                left: _, operator, ..
            } => operator.clone(),
            Node::IfExpression { .. } => "if".to_string(),
            Node::CallExpression { function, .. } => function.as_string(),
            Node::LetStatement { .. } => "let".to_string(),
            Node::ReturnStatement { .. } => "return".to_string(),
            Node::ExpressionStatement { expression } => {
                if let Some(expr) = expression {
                    expr.token_literal()
                } else {
                    "".to_string()
                }
            }
            Node::BlockStatement { .. } => "{".to_string(),
        }
    }

    /// Returns the string representation of an AST node.
    pub fn as_string(&self) -> String {
        match &self {
            Node::Program { statements } => statements
                .iter()
                .map(|st| st.as_string())
                .collect::<Vec<String>>()
                .join(""),
            Node::Identifier { value } => value.v.clone(),
            Node::IntegerLiteral { value } => value.to_string(),
            Node::Boolean { value } => value.to_string(),
            Node::FunctionLiteral { parameters, body } => {
                format!(
                    "fn({}) {}",
                    &parameters
                        .iter()
                        .map(|p| p.as_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    &body.as_string()
                )
            }
            Node::PrefixExpression { operator, right } => {
                format!("({}{})", operator, &*right.as_string())
            }
            Node::InfixExpression {
                left,
                operator,
                right,
            } => {
                format!(
                    "({} {} {})",
                    &*left.as_string(),
                    operator,
                    &*right.as_string()
                )
            }
            Node::IfExpression {
                condition,
                consequence,
                alternative,
            } => {
                let mut s = format!(
                    "if {} {}",
                    &*condition.as_string(),
                    &*consequence.as_string()
                );

                if let Some(a) = alternative {
                    s.push_str("else ");
                    s.push_str(&a.as_string());
                }

                s
            }
            Node::CallExpression {
                function,
                arguments,
            } => {
                format!(
                    "{}({})",
                    &function.as_string(),
                    &arguments
                        .iter()
                        .map(|p| p.as_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Node::LetStatement { name, value } => {
                let mut s = format!("let {}", &name.as_string());
                if let Some(v) = value {
                    s.push_str(" = ");
                    s.push_str(&v.as_string());
                }

                s.push(';');
                s
            }
            Node::ReturnStatement { value } => {
                let mut s = String::from("return");
                if let Some(v) = value {
                    s.push(' ');
                    s.push_str(&v.as_string());
                }

                s.push(';');
                s
            }
            Node::ExpressionStatement { expression } => {
                let mut s = String::new();
                if let Some(v) = expression {
                    s.push_str(&v.as_string());
                }

                s.push(';');
                s
            }
            Node::BlockStatement { statements } => statements
                .iter()
                .map(|statement| statement.as_string())
                .collect::<Vec<String>>()
                .join(""),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::TokenType;

    #[test]
    fn test_string() {
        let program = Node::Program {
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
