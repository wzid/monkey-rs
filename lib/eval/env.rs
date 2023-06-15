use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{value::Value, EvaluatorErr};

#[derive(Debug, Default, Clone)]
pub struct Environment {
    map: HashMap<String, Value>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new_enclosed(outer: Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            outer: Some(outer),
            map: HashMap::new(),
        }))
    }

    pub fn get(&self, name: &String) -> Result<Value, EvaluatorErr> {
        match self.map.get(name) {
            Some(v) => Ok(v.clone()),
            None => match &self.outer {
                Some(outer) => outer.borrow().get(name),
                None => Err(format!("identifier not found: {name}")),
            },
        }
    }

    pub fn set(&mut self, name: String, value: Value) -> Value {
        self.map.insert(name, value.clone());
        value
    }
}
