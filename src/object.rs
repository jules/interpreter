use crate::ast::Node;
use std::collections::HashMap;

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
            Object::Function {
                parameters, body, ..
            } => {
                let mut s = String::new();

                let mut params = vec![];
                parameters.iter().for_each(|p| {
                    params.push(p.as_string());
                });

                s.push_str("fn(");
                s.push_str(&params.join(", "));
                s.push_str(") {\n");
                s.push_str(&body.as_string());
                s.push_str("\n}");

                s
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
            Object::Function { .. } => "FUNCTION".to_string(),
            Object::Null { .. } => "NULL".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Environment {
    pub storage: HashMap<String, Object>,
    pub outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(env: Environment) -> Self {
        Self {
            storage: HashMap::new(),
            outer: Some(Box::new(env)),
        }
    }

    pub fn get(&self, k: &String) -> Option<Object> {
        match self.storage.get(k) {
            Some(v) => Some(v.clone()),
            None => {
                if self.outer.is_some() {
                    return self.outer.as_ref().unwrap().get(k);
                }

                None
            }
        }
    }
}
