use std::{fmt::Display, rc::Rc, cell::RefCell};

use crate::parser::ast::Statement;

use super::{env::Environment, funcs::BuiltInFunctionType};

pub trait Truth {
    fn truth(&self) -> bool;
}

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Boolean(bool),
    String(String),
    Return(Box<Value>), 
    Function {
        params: Vec<String>,
        body: Box<Statement>, // Statement::BlockStatement
        env: Rc<RefCell<Environment>>,
    },
    BuiltInFunction {
        func: BuiltInFunctionType,
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

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{i}"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Null => write!(f, "null"),
            Value::Return(v) => write!(f, "{v}"),
            Value::Function { params, body, .. } => {
                write!(f, "fn({}) {{\n{body}\n}}", params.join(", "))
            },
            Value::BuiltInFunction { .. } => write!(f, "builtin function"),
        }
    }
}
