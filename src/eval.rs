use crate::ast::Node;
use crate::object::Object;

pub fn eval(node: Node) -> Object {
    match node {
        Node::Program { statements } => eval_statements(statements),
        Node::IntegerLiteral { value: v } => Object::Integer { value: v },
        Node::ExpressionStatement { expression } => {
            if let Some(e) = expression {
                eval(*e)
            } else {
                Object::Null
            }
        }
        _ => panic!("Unsupported object"),
    }
}

fn eval_statements(statements: Vec<Node>) -> Object {
    statements.into_iter().fold(Object::Null, |_, s| eval(s))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_eval_integer_expression() {
        let table = vec![("5;".to_string(), 5), ("10;".to_string(), 10)];

        table.iter().for_each(|(input, output)| {
            let object = test_eval(input.to_string());
            match object {
                Object::Integer { value } => assert_eq!(value, *output),
                _ => panic!("Unexpected object"),
            }
        });
    }

    fn test_eval(input: String) -> Object {
        let l = Lexer::new(&input);
        let mut p = Parser::new(l);
        eval(p.parse_program())
    }
}
