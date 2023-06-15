use eval::{Evaluator, value::Value, EvaluatorErr};
use parser::{ast::Ast, program::Program};

#[macro_use]
pub mod token;

pub mod lexer;
pub mod parser;
pub mod eval;

#[derive(Default)]
pub struct Monkey {
    eval: Evaluator
}

impl Monkey {

    pub fn eval(&mut self, program: Program) -> Result<Value, EvaluatorErr> {
        self.eval.eval_self(&Ast::Program(program))
    }
}