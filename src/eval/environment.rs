use super::Object;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Environment {
    pub storage: HashMap<String, Object>,
    pub outer: Option<Box<Environment>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn get(&self, k: &str) -> Option<Object> {
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
