use super::Object;
use std::collections::HashMap;

/// The language environment which keeps hold of variable bindings. The evaluator
/// may use this to store or fetch any bindings it needs. Additionally, an
/// Environment can embed another Environment, which gives us the capability to
/// add closures to our programming language.
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
    /// Creates a new, blank Environment.
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
            outer: None,
        }
    }

    /// Creates a new Environment, which embeds another Environment.
    /// This function is used to initialize environments for closures.
    pub fn new_enclosed(env: Environment) -> Self {
        Self {
            storage: HashMap::new(),
            outer: Some(Box::new(env)),
        }
    }

    /// Fetch an Object from the Environment.
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

    /// Insert an Object into the Environment.
    pub fn set(&mut self, k: String, v: Object) -> Option<Object> {
        self.storage.insert(k, v)
    }
}
