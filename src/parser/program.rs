use std::fmt::Display;

use crate::parser::ast::Statement;

#[derive(Default)]
pub struct Program {
    pub statments: Vec<Statement>
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in &self.statments {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}