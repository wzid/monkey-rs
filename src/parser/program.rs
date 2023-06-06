use crate::parser::ast::Statement;

#[derive(Default)]
pub struct Program {
    pub statments: Vec<Statement>
}