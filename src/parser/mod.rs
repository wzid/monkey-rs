pub mod ast;
pub mod precedence;
pub mod program;

#[cfg(test)]
mod tests;

use crate::lexer::Lexer;
use crate::token::Token;
use program::Program;

use ast::{Expression, Statement};

use precedence::Precedence;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token,
    next_token: Token,
    pub errors: Vec<String>,
}

#[allow(dead_code)]
impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut p = Parser {
            lexer,
            curr_token: token![ILLEGAL],
            next_token: token![ILLEGAL],
            errors: Vec::new(),
        };

        // Read 2 tokens so that curr_token and next_token are both set
        p.advance_tokens();
        p.advance_tokens();

        p
    }

    fn advance_tokens(&mut self) {
        self.curr_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::default();

        while self.curr_token != token![EOF] {
            let stmt = self.parse_statement();

            if let Some(statement) = stmt {
                program.statments.push(statement);
            }

            self.advance_tokens();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.curr_token {
            token![LET] => self.parse_let_statement(),
            token![RETURN] => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        if let Some(expr) = self.parse_expression(Precedence::Lowest) {
            if self.is_next_token(Token::Semicolon) {
                self.advance_tokens();
            }

            return Some(Statement::ExpressionStatement(expr));
        }
        None
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        // Get the first expression
        let prefix = self.parse_prefix();

        // Something has messed up with other logic if this code runs
        if prefix.is_none() {
            self.report_error(format!("no prefix parse function for {}", self.curr_token));
            return None;
        }

        // We loop through and update the expression as it grow in size
        let mut left = prefix.unwrap();

        // Loop until we hit a semicolon or a token with a lower precedence
        while !self.is_next_token(token![;]) && precedence < Precedence::from(&self.next_token) {
            // Get the infix expression
            if let Some(infix) = self.parse_infix(&left) {
                left = infix;
            } else {
                return Some(left);
            }
        }

        Some(left)
    }

    fn parse_prefix(&mut self) -> Option<Expression> {
        match &self.curr_token {
            Token::Ident(name) => Some(self.parse_identifier(name.clone())),
            Token::Int(i) => Some(self.parse_integer(*i)),
            token![TRUE] | token![FALSE] => Some(self.parse_boolean_expression()),
            token![!] | token![-] => self.parse_prefix_expression(),
            token!['('] => self.parse_grouped_expression(),
            token![IF] => self.parse_if_expression(),
            token![FN] => self.parse_function_expression(),
            _ => None,
        }
    }

    fn parse_infix(&mut self, left: &Expression) -> Option<Expression> {
        // Advance the tokens only if we have a valid infix operator
        match &self.next_token {
            token![+]
            | token![-]
            | token![*]
            | token![/]
            | token![==]
            | token![!=]
            | token![<]
            | token![>] => {
                self.advance_tokens();
                self.parse_infix_expression(left.clone())
            }
            token!['('] => {
                self.advance_tokens();
                self.parse_call_expression(left.clone())
            }
            _ => None,
        }
    }

    fn is_curr_token(&self, token: Token) -> bool {
        self.curr_token == token
    }

    fn is_next_token(&self, token: Token) -> bool {
        self.next_token == token
    }

    fn expect_next_token(&mut self, token: Token) -> bool {
        if self.is_next_token(token.clone()) {
            return true;
        }

        self.report_error(format!(
            "expected next token to be {}, got {} instead",
            token, self.next_token
        ));
        false
    }

    fn report_error(&mut self, msg: String) {
        self.errors.push(msg);
    }

    fn advance_if_expected(&mut self, token: Token) -> bool {
        if self.expect_next_token(token) {
            self.advance_tokens();
            return true;
        }
        false
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        // Make sure we have an identifier after the let keyword
        let identifier = match &self.next_token {
            Token::Ident(_s) => {
                self.advance_tokens();
                self.curr_token.clone()
            }
            _ => return None,
        };

        if !self.advance_if_expected(token![=]) {
            return None;
        }

        // Advance past the '='
        self.advance_tokens();

        // Parse the expression
        if let Some(value) = self.parse_expression(Precedence::Lowest) {
            if self.is_next_token(token![;]) {
                self.advance_tokens();
            }

            return Some(Statement::LetStatement {
                ident: identifier,
                value,
            });
        } else {
            return None;
        }
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        // Advance past the return keyword
        self.advance_tokens();

        // Parse the expression
        if let Some(value) = self.parse_expression(Precedence::Lowest) {
            if self.is_next_token(token![;]) {
                self.advance_tokens();
            }
            return Some(Statement::ReturnStatement(value));
        }
        None
    }

    fn parse_identifier(&mut self, name: String) -> Expression {
        Expression::IdentifierExpression(name)
    }

    fn parse_integer(&mut self, value: i64) -> Expression {
        Expression::IntExpression(value)
    }

    // This function is called when we have an operator and an expression after it
    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let op_token = self.curr_token.clone();

        self.advance_tokens();

        if let Some(right) = self.parse_expression(Precedence::Prefix) {
            return Some(Expression::PrefixExpression {
                op_token,
                right: Box::new(right),
            });
        }
        None
    }

    // This function is called when we have a left expression and a right expression and an operator in between them
    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let op_token = self.curr_token.clone();
        let curr_precedence = Precedence::from(&op_token);

        self.advance_tokens();

        if let Some(right) = self.parse_expression(curr_precedence) {
            return Some(Expression::InfixExpression {
                left: Box::new(left),
                op_token,
                right: Box::new(right),
            });
        } else {
            return None;
        }
    }

    fn parse_boolean_expression(&self) -> Expression {
        Expression::BooleanExpression(self.is_curr_token(token![TRUE]))
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.advance_tokens();

        let expr = self.parse_expression(Precedence::Lowest);

        // If it does not end with a ')', then we have an error
        if !self.advance_if_expected(token![')']) {
            return None;
        }

        expr
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        self.advance_tokens();

        let condition = self.parse_expression(Precedence::Lowest);

        if !self.advance_if_expected(token!['{']) {
            return None;
        }

        let consequence = self.parse_block_statement();

        if self.is_next_token(token![ELSE]) {
            // ELSE is now curr_token
            self.advance_tokens();

            // '{' is now curr_token
            if !self.advance_if_expected(token!['{']) {
                return None;
            }

            println!("curr_token: {:?}", self.curr_token);
            println!("next_token: {:?}", self.next_token);
            let alternative = self.parse_block_statement();

            // // Advance past this if it was what ended the block statement
            // if self.is_curr_token(token!['}']) {
            //     self.advance_tokens();
            // }

            return Some(Expression::IfExpression {
                condition: Box::new(condition.unwrap()),
                consequence: Box::new(consequence),
                alternative: Some(Box::new(alternative)),
            });
        }

        Some(Expression::IfExpression {
            condition: Box::new(condition.unwrap()),
            consequence: Box::new(consequence),
            alternative: None,
        })
    }

    fn parse_block_statement(&mut self) -> Statement {
        self.advance_tokens();
        let mut statements = Vec::new();

        while !self.is_curr_token(token!['}']) && !self.is_curr_token(token![EOF]) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }

            self.advance_tokens();
        }

        Statement::BlockStatement(statements)
    }

    fn parse_function_expression(&mut self) -> Option<Expression> {
        if !self.advance_if_expected(token!['(']) {
            return None;
        }

        let parameters = self.parse_function_parameters();

        if !self.advance_if_expected(token!['{']) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Expression::FunctionExpression {
            parameters: parameters.unwrap(),
            body: Box::new(body),
        })
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Token>> {
        let mut identifiers = Vec::new();

        if self.is_next_token(token![')']) {
            self.advance_tokens();
            return Some(identifiers);
        }

        // Move the first parameter into curr_token
        self.advance_tokens();

        // Add the first parameter to the list
        identifiers.push(self.curr_token.clone());

        while self.is_next_token(token![,]) {
            self.advance_tokens();
            self.advance_tokens();
            identifiers.push(self.curr_token.clone());
        }

        if !self.advance_if_expected(token![')']) {
            return None;
        }

        Some(identifiers)
    }

    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        let arguments = self.parse_call_arguments();

        Some(Expression::CallExpression {
            function: Box::new(function),
            arguments: arguments.unwrap(),
        })
    }

    fn parse_call_arguments(&mut self) -> Option<Vec<Expression>> {
        let mut arguments = Vec::new();

        if self.is_next_token(token![')']) {
            self.advance_tokens();
            return Some(arguments);
        }

        // Move the first argument into curr_token
        self.advance_tokens();

        println!("curr_token: {:?}", self.curr_token);
        println!("next_token: {:?}", self.next_token);
        // Add the first argument to the list
        arguments.push(self.parse_expression(Precedence::Lowest).unwrap());

        while self.is_next_token(token![,]) {
            self.advance_tokens();
            self.advance_tokens();
            arguments.push(self.parse_expression(Precedence::Lowest).unwrap());
        }

        if !self.advance_if_expected(token![')']) {
            return None;
        }

        Some(arguments)
    }
}
