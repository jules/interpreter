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
    pub fn token_literal(&self) -> String {
        match &self {
            Node::Identifier { value } => value.v.clone(),
            Node::IntegerLiteral { value } => value.to_string(),
            Node::Boolean { value } => value.to_string(),
            Node::FunctionLiteral { .. } => "function".to_string(),
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

    pub fn as_string(&self) -> String {
        let mut s = String::new();
        match &self {
            Node::Identifier { value } => s.push_str(&value.v),
            Node::IntegerLiteral { value } => s.push_str(&value.to_string()),
            Node::Boolean { value } => s.push_str(&value.to_string()),
            Node::FunctionLiteral { parameters, body } => {
                s.push_str(&"fn(");
                s.push_str(
                    &parameters
                        .iter()
                        .map(|p| p.as_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                );
                s.push_str(&") ");
                s.push_str(&body.as_string());
            }
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
            Node::IfExpression {
                condition,
                consequence,
                alternative,
            } => {
                s.push_str(&"if");
                s.push_str(&*condition.as_string());
                s.push(' ');
                s.push_str(&*consequence.as_string());

                if let Some(a) = alternative {
                    s.push_str(&"else ");
                    s.push_str(&a.as_string());
                }
            }
            Node::CallExpression {
                function,
                arguments,
            } => {
                s.push_str(&function.as_string());
                s.push('(');
                s.push_str(
                    &arguments
                        .iter()
                        .map(|p| p.as_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                );
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
            Node::BlockStatement { statements } => {
                statements.iter().for_each(|statement| {
                    s.push_str(&*statement.as_string());
                });
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
