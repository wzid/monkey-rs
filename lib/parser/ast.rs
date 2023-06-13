use std::fmt::Display;

use crate::token::Token;

use super::program::Program;

#[allow(dead_code)]
pub enum Ast {
    Program(Program),
    Statement(Statement),
    Expression(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    LetStatement {
        ident: Token,
        value: Expression,
    },
    ReturnStatement(Expression),
    ExpressionStatement(Expression),
    BlockStatement(Vec<Statement>),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::LetStatement { ident, value } => {
                write!(f, "let {} = {};", ident, value)
            }
            Statement::ReturnStatement(value) => write!(f, "return {};", value),
            Statement::ExpressionStatement(value) => write!(f, "{}", value),
            Statement::BlockStatement(statements) => {
                statements.iter().for_each(|stmt| write!(f, "{}", stmt).unwrap());
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    IntExpression(i64),
    IdentifierExpression(String),
    PrefixExpression {
        op_token: Token,
        right: Box<Expression>,
    },
    InfixExpression {
        left: Box<Expression>,
        op_token: Token,
        right: Box<Expression>,
    },
    BooleanExpression(bool),
    IfExpression {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
    FunctionExpression {
        parameters: Vec<String>,
        body: Box<Statement>,
    },
    CallExpression {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::IntExpression(value) => write!(f, "{}", value),
            Expression::IdentifierExpression(name) => write!(f, "{}", name),
            Expression::PrefixExpression { op_token, right } => {
                write!(f, "({}{})", op_token, right)
            }
            Expression::InfixExpression {
                left,
                op_token,
                right,
            } => {
                write!(f, "({} {} {})", left, op_token, right)
            }
            Expression::BooleanExpression(value) => write!(f, "{}", value),
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
            } => {
                write!(f, "if {} {{{}}}", condition, consequence)?;
                if let Some(alt) = alternative {
                    write!(f, " else {{{}}}", alt)?;
                }
                Ok(())
            }
            Expression::FunctionExpression { parameters, body } => {
                let params = parameters.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ");
                write!(f, "fn ({params}) {{{body}}}")
            }
            Expression::CallExpression { function, arguments } => {
                let args = arguments.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ");
                write!(f, "{function}({args})")
            }
        }
    }
}
