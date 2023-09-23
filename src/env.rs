use crate::object::Object;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Env {
    parent: Option<Rc<Env>>,
    vars: HashMap<String, Object>,
}

pub const KEYWORDS: [&str; 8]  = ["if", "define", "true", "false", "lambda", "print", "equal", "load"];

/// TODO: Document this thing
impl Env {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.vars.get(name) {
            Some(value) => Some(value.clone()),
            None => self.parent.as_ref().and_then(|o| o.get(name)),
        }
    }

    pub fn set(&mut self, name: &str, val: Object) {
        self.vars.insert(name.to_string(), val);
    }
}
