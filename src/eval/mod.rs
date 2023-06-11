use crate::{
    parser::{ast::{Ast, Expression, Statement}, program::{self, Program}},
    token::Token,
};

use self::value::{Truth, Value};

pub mod value;

#[cfg(test)]
mod tests;

pub fn eval(ast: &Ast) -> Value {
    match ast {
        Ast::Program(prog) => eval_program(prog),
        Ast::Statement(stmt) => eval_statement(stmt),
        Ast::Expression(expr) => eval_expression(expr),
    }
}

fn eval_statement(stmt: &Statement) -> Value {
    match stmt {
        Statement::LetStatement { ident, value } => todo!(),
        Statement::BlockStatement(statements) => eval_block(statements),
        Statement::ReturnStatement(expr) => {
            let value = eval_expression(expr);
            Value::Return(Box::new(value))
        },
        Statement::ExpressionStatement(expr) => eval_expression(expr),
    }
}

fn eval_expression(expr: &Expression) -> Value {
    match expr {
        Expression::IntExpression(i) => Value::Integer(*i),
        Expression::BooleanExpression(b) => Value::Boolean(*b),
        Expression::IdentifierExpression(s) => todo!(),
        Expression::PrefixExpression { op_token, right } => {
            let right = eval_expression(&right);
            eval_prefix_expression(op_token, right)
        }
        Expression::InfixExpression {
            left,
            op_token,
            right,
        } => {
            let left = eval_expression(&left);
            let right = eval_expression(&right);
            eval_infix_expression(left, op_token, right)
        }
        Expression::IfExpression {
            condition,
            consequence,
            alternative,
        } => {
            let condition = eval_expression(&condition);
            eval_if_expression(condition, consequence, alternative.as_deref())
        }
        Expression::FunctionExpression { parameters, body } => todo!(),
        Expression::CallExpression {
            function,
            arguments,
        } => todo!(),
    }
}

fn eval_program(program: &Program) -> Value {
    let mut stmt_value = Value::Null;

    for stmt in &program.statments {
        stmt_value = eval_statement(stmt);

        // In evaluating the program we want to return the value of the return statement
        if let Value::Return(val) = stmt_value {
            return *val;
        }
    }

    return stmt_value;
}

fn eval_block(block: &Vec<Statement>) -> Value {
    let mut stmt_value = Value::Null;

    for stmt in block {
        stmt_value = eval_statement(stmt);

        // In evaluating the block statement we only want to return the Value::Return object
        if let Value::Return(_) = stmt_value {
            return stmt_value;
        }
    }

    return stmt_value;
}

fn eval_prefix_expression(operator: &Token, right: Value) -> Value {
    match (operator, right) {
        // Negate the truth value
        (token![!], right) => (!right.truth()).into(),
        // Only apply the negative operator when its an integer
        (token![-], Value::Integer(i)) => Value::Integer(-i),
        _ => Value::Null,
    }
}

fn eval_infix_expression(left: Value, operator: &Token, right: Value) -> Value {
    match (left, operator, right) {
        (Value::Integer(l), _, Value::Integer(r)) => eval_integer_infix_expression(l, operator, r),
        (Value::Boolean(l), token![==], Value::Boolean(r)) => (l == r).into(),
        (Value::Boolean(l), token![!=], Value::Boolean(r)) => (l != r).into(),
        _ => Value::Null,
    }
}

fn eval_integer_infix_expression(left: i64, operator: &Token, right: i64) -> Value {
    match operator {
        // Returns a Value::Integer
        token![+] => (left + right).into(),
        token![-] => (left - right).into(),
        token![*] => (left * right).into(),
        token![/] => (left / right).into(),
        // Returns a Value::Boolean
        token![<] => (left < right).into(),
        token![>] => (left > right).into(),
        token![==] => (left == right).into(),
        token![!=] => (left != right).into(),
        _ => return Value::Null,
    }
}

fn eval_if_expression(
    condition: Value,
    consequence: &Statement,
    alternative: Option<&Statement>,
) -> Value {
    // If the condition is true then we evaluate the first block
    if condition.truth() {
        return eval_statement(consequence);
        // if the condition is not true and the alternative is defined then we evalue that
    } else if let Some(alt) = alternative {
        return eval_statement(alt);
    }

    Value::Null
}
