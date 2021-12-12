#[derive(Debug, PartialEq, Eq)]
pub enum Object {
    Integer { value: i64 },
    Boolean { value: bool },
    ReturnValue { value: Box<Object> },
    Null,
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer { value } => format!("{}", value),
            Object::Boolean { value } => format!("{}", value),
            Object::ReturnValue { value } => (*value.inspect()).to_string(),
            Object::Null => String::from("null"),
        }
    }
}
