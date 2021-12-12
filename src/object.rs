#[derive(PartialEq, Eq)]
pub enum Object {
    Integer { value: i64 },
    Boolean { value: bool },
    Null,
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer { value } => format!("{}", value),
            Object::Boolean { value } => format!("{}", value),
            Object::Null => String::from("null"),
        }
    }
}
