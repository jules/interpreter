use super::Environment;
use crate::ast::Node;

/// All types of language objects. These are resolved from AST nodes, and represent
/// any kind of evaluated input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Object {
    Integer {
        value: i64,
    },
    Boolean {
        value: bool,
    },
    ReturnValue {
        value: Box<Object>,
    },
    Error {
        value: String,
    },
    Function {
        parameters: Vec<Node>,
        body: Node,
        env: Environment,
    },
    Null,
}

impl Object {
    /// Returns the string representation of an Object.
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer { value } => format!("{}", value),
            Object::Boolean { value } => format!("{}", value),
            Object::ReturnValue { value } => (*value.inspect()).to_string(),
            Object::Error { value } => {
                format!("ERROR: {}", value)
            }
            Object::Function {
                parameters, body, ..
            } => {
                format!(
                    "fn({}) {{\n{}\n}}",
                    parameters
                        .iter()
                        .map(|p| p.as_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    &body.as_string()
                )
            }
            Object::Null => String::from("null"),
        }
    }

    /// Returns the proper name for an Object, as a string.
    pub fn name(&self) -> String {
        match self {
            Object::Integer { .. } => "INTEGER".to_string(),
            Object::Boolean { .. } => "BOOLEAN".to_string(),
            Object::ReturnValue { .. } => "RETURN_VALUE".to_string(),
            Object::Error { .. } => "ERROR".to_string(),
            Object::Function { .. } => "FUNCTION".to_string(),
            Object::Null { .. } => "NULL".to_string(),
        }
    }
}
