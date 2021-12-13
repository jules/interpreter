use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Object {
    Integer { value: i64 },
    Boolean { value: bool },
    ReturnValue { value: Box<Object> },
    Error { value: String },
    Null,
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer { value } => format!("{}", value),
            Object::Boolean { value } => format!("{}", value),
            Object::ReturnValue { value } => (*value.inspect()).to_string(),
            Object::Error { value } => {
                let mut e = String::from("ERROR: ");
                e.push_str(&value);
                e
            }
            Object::Null => String::from("null"),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Object::Integer { .. } => "INTEGER".to_string(),
            Object::Boolean { .. } => "BOOLEAN".to_string(),
            Object::ReturnValue { .. } => "RETURN_VALUE".to_string(),
            Object::Error { .. } => "ERROR".to_string(),
            Object::Null { .. } => "NULL".to_string(),
        }
    }
}

pub struct Environment {
    pub storage: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
}
