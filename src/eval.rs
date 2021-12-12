use crate::ast::Node;
use crate::object::Object;

const NULL: Object = Object::Null;
const TRUE: Object = Object::Boolean { value: true };
const FALSE: Object = Object::Boolean { value: false };

pub fn eval(node: Node) -> Object {
    match node {
        Node::Program { statements } => eval_statements(statements),
        Node::IntegerLiteral { value: v } => Object::Integer { value: v },
        Node::Boolean { value: v } => {
            if v {
                TRUE
            } else {
                FALSE
            }
        }
        Node::ExpressionStatement { expression } => {
            if let Some(e) = expression {
                eval(*e)
            } else {
                NULL
            }
        }
        Node::PrefixExpression { operator, right } => {
            let right = eval(*right);
            eval_prefix_expression(operator, right)
        }
        _ => panic!("Unsupported object"),
    }
}

fn eval_statements(statements: Vec<Node>) -> Object {
    statements.into_iter().fold(Object::Null, |_, s| eval(s))
}

fn eval_prefix_expression(operator: String, right: Object) -> Object {
    match operator.as_str() {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_operator_expression(right),
        _ => NULL,
    }
}

fn eval_bang_operator_expression(right: Object) -> Object {
    match right {
        TRUE => FALSE,
        FALSE => TRUE,
        NULL => TRUE,
        _ => FALSE,
    }
}

fn eval_minus_operator_expression(right: Object) -> Object {
    match right {
        Object::Integer { value } => Object::Integer { value: -value },
        _ => NULL,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_eval_integer_expression() {
        let table = vec![
            ("5;".to_string(), 5),
            ("10;".to_string(), 10),
            ("-5;".to_string(), -5),
            ("-10;".to_string(), -10),
        ];

        table.iter().for_each(|(input, output)| {
            let object = test_eval(input.to_string());
            match object {
                Object::Integer { value } => assert_eq!(value, *output),
                _ => panic!("Unexpected object"),
            }
        });
    }

    #[test]
    fn test_eval_boolean_expression() {
        let table = vec![("true;".to_string(), true), ("false;".to_string(), false)];

        table.iter().for_each(|(input, output)| {
            let object = test_eval(input.to_string());
            match object {
                Object::Boolean { value } => assert_eq!(value, *output),
                _ => panic!("Unexpected object"),
            }
        });
    }

    #[test]
    fn test_bang_operator() {
        let table = vec![
            ("!true;".to_string(), false),
            ("!false;".to_string(), true),
            ("!5;".to_string(), false),
            ("!!true;".to_string(), true),
            ("!!false;".to_string(), false),
            ("!!5;".to_string(), true),
        ];

        table.iter().for_each(|(input, output)| {
            let object = test_eval(input.to_string());
            match object {
                Object::Boolean { value } => assert_eq!(value, *output),
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
