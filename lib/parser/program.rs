use std::fmt::Display;

use crate::parser::ast::Statement;

#[derive(Debug, Default)]
pub struct Program {
    pub statments: Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self
            .statments
            .iter()
            .map(|stmt| stmt.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{output}")
    }
}
