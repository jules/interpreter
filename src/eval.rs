use crate::ast::Node;
use crate::object::Object;

const NULL: Object = Object::Null;
const TRUE: Object = Object::Boolean { value: true };
const FALSE: Object = Object::Boolean { value: false };

pub fn eval(node: Node) -> Object {
    match node {
        Node::Program { statements } => eval_program(statements),
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
            if is_error(right.clone()) {
                return right;
            }

            eval_prefix_expression(operator, right)
        }
        Node::InfixExpression {
            left,
            operator,
            right,
        } => {
            let right = eval(*right);
            if is_error(right.clone()) {
                return right;
            }

            let left = eval(*left);
            if is_error(left.clone()) {
                return left;
            }

            eval_infix_expression(operator, left, right)
        }
        Node::BlockStatement { statements } => eval_block_statement(statements),
        Node::IfExpression {
            condition,
            consequence,
            alternative,
        } => eval_if_expression(*condition, *consequence, alternative),
        Node::ReturnStatement { value } => match value {
            Some(v) => {
                let evaluated = eval(*v.clone());
                if is_error(evaluated.clone()) {
                    return evaluated;
                }

                Object::ReturnValue {
                    value: Box::new(eval(*v)),
                }
            }
            None => Object::ReturnValue {
                value: Box::new(NULL),
            },
        },
        _ => panic!("Unsupported object"),
    }
}

fn eval_program(statements: Vec<Node>) -> Object {
    let mut s = Object::Null;
    for statement in statements {
        s = eval(statement);
        match s {
            Object::ReturnValue { value } => {
                return *value;
            }
            Object::Error { .. } => {
                break;
            }
            _ => {}
        }
    }

    s
}

fn eval_block_statement(statements: Vec<Node>) -> Object {
    let mut s = Object::Null;
    for statement in statements {
        s = eval(statement);
        if matches!(s, Object::ReturnValue { .. }) || matches!(s, Object::Error { .. }) {
            break;
        }
    }

    s
}

fn eval_prefix_expression(operator: String, right: Object) -> Object {
    match operator.as_str() {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_operator_expression(right),
        _ => Object::Error {
            value: format!("unknown operator: {}{}", operator, right.name()),
        },
    }
}

fn eval_infix_expression(operator: String, left: Object, right: Object) -> Object {
    match (left.clone(), operator.as_str(), right.clone()) {
        (Object::Integer { value: v1 }, _, Object::Integer { value: v2 }) => {
            eval_integer_infix_expression(operator, v1, v2)
        }
        (_, "==", _) => {
            return Object::Boolean {
                value: left == right,
            }
        }
        (_, "!=", _) => {
            return Object::Boolean {
                value: left != right,
            }
        }
        _ => {
            if left.name() != right.name() {
                return Object::Error {
                    value: format!(
                        "type mismatch: {} {} {}",
                        left.name(),
                        operator,
                        right.name()
                    ),
                };
            } else {
                return Object::Error {
                    value: format!(
                        "unknown operator: {} {} {}",
                        left.name(),
                        operator,
                        right.name()
                    ),
                };
            }
        }
    }
}

fn eval_if_expression(
    condition: Node,
    consequence: Node,
    alternative: Option<Box<Node>>,
) -> Object {
    let condition = eval(condition);
    if is_error(condition.clone()) {
        return condition;
    }

    if is_truthy(condition) {
        eval(consequence)
    } else if alternative.is_some() {
        eval(*alternative.unwrap())
    } else {
        NULL
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
        _ => {
            return Object::Error {
                value: format!("unknown operator: -{}", right.name()),
            }
        }
    }
}

fn eval_integer_infix_expression(operator: String, left: i64, right: i64) -> Object {
    match operator.as_str() {
        "+" => Object::Integer {
            value: left + right,
        },
        "-" => Object::Integer {
            value: left - right,
        },
        "*" => Object::Integer {
            value: left * right,
        },
        "/" => Object::Integer {
            value: left / right,
        },
        "<" => Object::Boolean {
            value: left < right,
        },
        ">" => Object::Boolean {
            value: left > right,
        },
        "==" => Object::Boolean {
            value: left == right,
        },
        "!=" => Object::Boolean {
            value: left != right,
        },
        _ => {
            return Object::Error {
                value: format!("unknown operator: INTEGER {} INTEGER", operator),
            }
        }
    }
}

fn is_truthy(condition: Object) -> bool {
    match condition {
        NULL => false,
        TRUE => true,
        FALSE => false,
        _ => true,
    }
}

fn is_error(object: Object) -> bool {
    if let Object::Error { .. } = object {
        true
    } else {
        false
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
            ("5 + 5 + 5 + 5 - 10;".to_string(), 10),
            ("2 * 2 * 2 * 2 * 2;".to_string(), 32),
            ("-50 + 100 + -50;".to_string(), 0),
            ("5 * 2 + 10;".to_string(), 20),
            ("5 + 2 * 10;".to_string(), 25),
            ("20 + 2 * -10;".to_string(), 0),
            ("50 / 2 * 2 + 10;".to_string(), 60),
            ("2 * (5 + 10);".to_string(), 30),
            ("3 * 3 * 3 + 10;".to_string(), 37),
            ("3 * (3 * 3) + 10;".to_string(), 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10;".to_string(), 50),
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
        let table = vec![
            ("true;".to_string(), true),
            ("false;".to_string(), false),
            ("1 < 2;".to_string(), true),
            ("1 > 2;".to_string(), false),
            ("1 < 1;".to_string(), false),
            ("1 > 1;".to_string(), false),
            ("1 == 1;".to_string(), true),
            ("1 != 1;".to_string(), false),
            ("1 == 2;".to_string(), false),
            ("1 != 2;".to_string(), true),
            ("true == true;".to_string(), true),
            ("false == false;".to_string(), true),
            ("true == false;".to_string(), false),
            ("true != false;".to_string(), true),
            ("false != true;".to_string(), true),
            ("(1 < 2) == true;".to_string(), true),
            ("(1 < 2) == false;".to_string(), false),
            ("(1 > 2) == true;".to_string(), false),
            ("(1 > 2) == false;".to_string(), true),
        ];

        table.iter().for_each(|(input, output)| {
            let object = test_eval(input.to_string());
            match object {
                Object::Boolean { value } => assert_eq!(value, *output),
                _ => panic!("Unexpected object, {:?}", object),
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

    #[test]
    fn test_if_else_expressions() {
        let table = vec![
            (
                "if (true) { 10 };".to_string(),
                Object::Integer { value: 10 },
            ),
            ("if (false) { 10 };".to_string(), Object::Null),
            ("if (1) { 10 };".to_string(), Object::Integer { value: 10 }),
            (
                "if (1 < 2) { 10 };".to_string(),
                Object::Integer { value: 10 },
            ),
            ("if (1 > 2) { 10 };".to_string(), Object::Null),
            (
                "if (1 > 2) { 10 } else { 20 };".to_string(),
                Object::Integer { value: 20 },
            ),
            (
                "if (1 < 2) { 10 } else { 20 };".to_string(),
                Object::Integer { value: 10 },
            ),
        ];

        table.iter().for_each(|(input, output)| {
            let object = test_eval(input.to_string());
            assert_eq!(object, *output);
        });
    }

    #[test]
    fn test_return_statements() {
        let table = vec![
            ("return 10;".to_string(), 10),
            ("return 10; 9;".to_string(), 10),
            ("return 2 * 5; 9;".to_string(), 10),
            ("9; return 2 * 5; 9;".to_string(), 10),
            (
                "if (10 > 1) { if (10 > 1) { return 10; } return 1; };".to_string(),
                10,
            ),
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
    fn test_error_handling() {
        let table = vec![
            ("5 + true;".to_string(), "type mismatch: INTEGER + BOOLEAN"),
            (
                "5 + true; 5;".to_string(),
                "type mismatch: INTEGER + BOOLEAN",
            ),
            ("-true;".to_string(), "unknown operator: -BOOLEAN"),
            (
                "true + false;".to_string(),
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                "5; true + false; 5;".to_string(),
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                "if (10 > 1) { true + false; };".to_string(),
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                "if (10 > 1) { if (10 > 1) { return true + false; } return 1; };".to_string(),
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
        ];

        table.iter().for_each(|(input, output)| {
            let object = test_eval(input.to_string());
            match object {
                Object::Error { value } => assert_eq!(value, *output),
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
