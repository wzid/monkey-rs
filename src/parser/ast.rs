use std::fmt::Display;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    LetStatement { ident: Token, value: Option<Expression> },
    ReturnStatement(Option<Expression>),
    ExpressionStatement(Expression),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::LetStatement { ident, value } => {
                write!(f, "let {} = {};", ident, value.as_ref().unwrap())
            }
            Statement::ReturnStatement(value) => write!(f, "return {};", value.as_ref().unwrap()),
            Statement::ExpressionStatement(value) => write!(f, "{}", value),
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
}


impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::IntExpression(value) => write!(f, "{}", value),
            Expression::IdentifierExpression(name) => write!(f, "{}", name),
            Expression::PrefixExpression { op_token, right } => {
                write!(f, "({}{})", op_token, right)
            }
            Expression::InfixExpression { left, op_token, right } => {
                write!(f, "({} {} {})", left, op_token, right)
            },
            Expression::BooleanExpression(value) => write!(f, "{}", value),
            Expression::IfExpression { condition, consequence, alternative } => {
                write!(f, "if {} {}", condition, consequence)?;
                if let Some(alt) = alternative {
                    write!(f, " else {}", alt)?;
                }
                Ok(())
            }
        }
    }
}