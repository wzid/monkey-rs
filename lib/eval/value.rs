use std::{fmt::Display, rc::Rc, cell::RefCell};

use crate::parser::ast::Statement;

use super::env::Environment;

pub trait Truth {
    fn truth(&self) -> bool;
}

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Boolean(bool),
    Return(Box<Value>), 
    Function {
        params: Vec<String>,
        body: Box<Statement>, // Statement::BlockStatement
        env: Rc<RefCell<Environment>>,
    },
    Null,
}

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
}

impl Truth for Value {
    fn truth(&self) -> bool {
        match self {
            Value::Integer(i) => i > &0,
            Value::Boolean(b) => *b,
            _ => false
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Integer(value)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{i}"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Null => write!(f, "null"),
            Value::Return(v) => write!(f, "{v}"),
            Value::Function { params, body, .. } => {
                write!(f, "fn({}) {{\n{body}\n}}", params.join(", "))
            },
        }
    }
}
