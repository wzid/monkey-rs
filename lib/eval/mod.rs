use std::{cell::RefCell, rc::Rc};

use crate::{
    parser::{
        ast::{Ast, Expression, Statement},
        program::Program,
    },
    token::Token,
};

use self::value::{Truth, Value};
use env::Environment;

pub mod env;
pub mod value;

#[cfg(test)]
mod tests;

pub type EvaluatorErr = String;

#[derive(Default)]
pub struct Evaluator {
    env: Rc<RefCell<env::Environment>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            env: Rc::new(RefCell::new(Environment::default())),
        }
    }
    // Main entry point
    pub fn eval_with_environment(
        ast: &Ast,
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, EvaluatorErr> {
        let mut evaluator = Evaluator { env };
        match ast {
            Ast::Program(prog) => evaluator.eval_program(prog),
            Ast::Statement(stmt) => evaluator.eval_statement(stmt),
            Ast::Expression(expr) => evaluator.eval_expression(expr),
        }
    }

    // Entry point method with a new Environment (no local variables set)
    pub fn eval(ast: &Ast) -> Result<Value, EvaluatorErr> {
        Evaluator::eval_with_environment(ast, Rc::new(RefCell::new(Environment::default())))
    }

    pub fn eval_self(&mut self, ast: &Ast) -> Result<Value, EvaluatorErr> {
        match ast {
            Ast::Program(prog) => self.eval_program(prog),
            Ast::Statement(stmt) => self.eval_statement(stmt),
            Ast::Expression(expr) => self.eval_expression(expr),
        }
    }

    fn eval_statement(&mut self, stmt: &Statement) -> Result<Value, EvaluatorErr> {
        match stmt {
            Statement::LetStatement { ident, value } => {
                let value = self.eval_expression(value)?;

                // Set the value in the map and return the value
                Ok(self.env.borrow_mut().set(ident.to_string(), value))
            }
            Statement::BlockStatement(statements) => self.eval_block(statements),
            Statement::ReturnStatement(expr) => {
                let value = self.eval_expression(expr)?;
                Ok(Value::Return(Box::new(value)))
            }
            Statement::ExpressionStatement(expr) => self.eval_expression(expr),
        }
    }

    fn eval_expression(&mut self, expr: &Expression) -> Result<Value, EvaluatorErr> {
        match expr {
            Expression::IntExpression(i) => Ok(Value::Integer(*i)),
            Expression::BooleanExpression(b) => Ok(Value::Boolean(*b)),
            Expression::IdentifierExpression(s) => self.eval_identifier(s),
            Expression::PrefixExpression { op_token, right } => {
                let right = self.eval_expression(right)?;
                self.eval_prefix_expression(op_token, right)
            }
            Expression::InfixExpression {
                left,
                op_token,
                right,
            } => {
                let left = self.eval_expression(left)?;
                let right = self.eval_expression(right)?;
                self.eval_infix_expression(left, op_token, right)
            }
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
            } => {
                let condition = self.eval_expression(condition)?;
                self.eval_if_expression(condition, consequence, alternative.as_deref())
            }
            Expression::FunctionExpression { parameters, body } => Ok(Value::Function {
                params: parameters.clone(),
                body: body.clone(),
                env: Rc::clone(&self.env),
            }),
            Expression::CallExpression {
                function,
                arguments,
            } => {
                let function = self.eval_expression(function)?;

                // Evaluate every argument into a Vec<Value> or return an error if it happens
                let arguments = arguments
                    .iter()
                    .map(|arg| self.eval_expression(arg))
                    .collect::<Result<Vec<Value>, EvaluatorErr>>()?;

                let result = self.apply_function(function, arguments)?;

                Ok(match result {
                    Value::Return(v) => *v,
                    _ => result,
                })
            }
        }
    }

    fn eval_program(&mut self, program: &Program) -> Result<Value, EvaluatorErr> {
        let mut stmt_value = Value::Null;

        for stmt in &program.statments {
            stmt_value = self.eval_statement(stmt)?;

            // In evaluating the program we want to return the value of the return statement
            if let Value::Return(val) = stmt_value {
                return Ok(*val);
            }
        }

        Ok(stmt_value)
    }

    fn eval_block(&mut self, block: &Vec<Statement>) -> Result<Value, EvaluatorErr> {
        let mut stmt_value = Value::Null;

        for stmt in block {
            stmt_value = self.eval_statement(stmt)?;

            // In evaluating the block statement we only want to return the Value::Return object
            if let Value::Return(_) = stmt_value {
                return Ok(stmt_value);
            }
        }

        Ok(stmt_value)
    }

    fn eval_prefix_expression(
        &self,
        operator: &Token,
        right: Value,
    ) -> Result<Value, EvaluatorErr> {
        match (operator, &right) {
            // Negate the truth value
            (token![!], right) => Ok((!right.truth()).into()),
            // Only apply the negative operator when its an integer
            (token![-], Value::Integer(i)) => Ok(Value::Integer(-i)),
            _ => Err(format!("Invalid prefix expression!\n\t({operator}{right})")),
        }
    }

    fn eval_infix_expression(
        &self,
        left: Value,
        operator: &Token,
        right: Value,
    ) -> Result<Value, EvaluatorErr> {
        match (&left, operator, &right) {
            (Value::Integer(l), _, Value::Integer(r)) => {
                self.eval_integer_infix_expression(*l, operator, *r)
            }
            (Value::Boolean(l), token![==], Value::Boolean(r)) => Ok((l == r).into()),
            (Value::Boolean(l), token![!=], Value::Boolean(r)) => Ok((l != r).into()),
            _ => Err(format!(
                "Invalid infix expression!\n\t({left} {operator} {right})"
            )),
        }
    }

    fn eval_integer_infix_expression(
        &self,
        left: i64,
        operator: &Token,
        right: i64,
    ) -> Result<Value, EvaluatorErr> {
        match operator {
            // Returns a Value::Integer
            token![+] => Ok((left + right).into()),
            token![-] => Ok((left - right).into()),
            token![*] => Ok((left * right).into()),
            token![/] => Ok((left / right).into()),
            token![%] => Ok((left % right).into()),
            // Returns a Value::Boolean
            token![<] => Ok((left < right).into()),
            token![>] => Ok((left > right).into()),
            token![==] => Ok((left == right).into()),
            token![!=] => Ok((left != right).into()),
            _ => return Err(format!("Invalid integer infix operator!\n\t({left} {operator} {right}).\n {operator} is not a valid integer operator")),
        }
    }

    fn eval_if_expression(
        &mut self,
        condition: Value,
        consequence: &Statement,
        alternative: Option<&Statement>,
    ) -> Result<Value, EvaluatorErr> {
        // If the condition is true then we evaluate the first block
        if condition.truth() {
            return self.eval_statement(consequence);
            // if the condition is not true and the alternative is defined then we evalue that
        } else if let Some(alt) = alternative {
            return self.eval_statement(alt);
        }

        // Not an error. this means nothing was done from the if statement
        Ok(Value::Null)
    }

    fn eval_identifier(&self, name: &String) -> Result<Value, EvaluatorErr> {
        self.env.borrow().get(name)
    }

    fn apply_function(&self, func: Value, arguments: Vec<Value>) -> Result<Value, EvaluatorErr> {
        match func {
            Value::Function { params, body, env } => {
                let func_env = self.setup_function_env(env, params, arguments);

                let result =
                    Evaluator::eval_with_environment(&Ast::Statement(*body), func_env)?;

                Ok(result)
            }
            _ => Err(format!(
                "apply_function had an error. func is not of type Value::Function. got {func}"
            )),
        }
    }

    fn setup_function_env(
        &self,
        function_env: Rc<RefCell<Environment>>,
        params: Vec<String>,
        arguments: Vec<Value>,
    ) -> Rc<RefCell<Environment>> {
        let function_env = Environment::new_enclosed(function_env);

        for (i, param) in params.iter().enumerate() {
            function_env
                .borrow_mut()
                .set(param.to_string(), arguments[i].clone());
        }

        function_env
    }
}
