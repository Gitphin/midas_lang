use std::collections::HashMap;
use crate::literals::LiteralVal;

pub struct Enviro {
     vals:  HashMap<String, LiteralVal>,
}

impl Enviro {
    pub fn new() -> Self {
        Self {
            vals: HashMap::new()
        }
    }

    pub fn define(&mut self, name: String, val: LiteralVal) {
        self.vals.insert(name, val);
    }

    pub fn get(&self, name: &str) -> Option<&LiteralVal> {
        self.vals.get(name)
    }

}
