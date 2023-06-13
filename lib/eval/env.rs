use std::{collections::HashMap, cell::RefCell, rc::Rc, fmt::Display};

use super::{value::Value, EvaluatorErr};

#[derive(Debug, Default, Clone)]
pub struct Enviorment {
    map: HashMap<String, Value>,
    outer: Option<Rc<RefCell<Enviorment>>>,
}

impl Enviorment {
    pub fn new_enclosed(outer: Rc<RefCell<Enviorment>>) -> Rc<RefCell<Self>> {
        let mut env = Enviorment::default();
        env.outer = Some(outer);
        Rc::new(RefCell::new(env))
    }

    pub fn get(&self, name: &String) -> Result<Value, EvaluatorErr> {
        match self.map.get(name) {
            Some(v) => Ok(v.clone()),
            None => match &self.outer {
                Some(outer) => outer.borrow().get(name),
                None => Err(format!("identifier not found: {}", name)),
            },
        }
    }

    pub fn set(&mut self, name: String, value: Value) -> Value {
        self.map.insert(name, value.clone());
        value
    }
}

impl Display for Enviorment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


