pub mod program;
pub mod ast;

use program::Program;
use crate::lexer::Lexer;
use crate::token::Token;

use self::ast::{Statement, StatementType, Identifier};


struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token,
    next_token: Token,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        let mut p = Parser { lexer, curr_token: Token::Illegal, next_token: Token::Illegal };

        // Read 2 tokens so that curr_token and next_token are both set
        p.advance_tokens();
        p.advance_tokens();

        p
    }

    fn advance_tokens(&mut self) {
        self.curr_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program::default();

        while self.curr_token != Token::Eof {
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
            Token::Let => Statement::from(self.parse_let_statement()),
            _ => None
        }
    }

    fn parse_let_statement(&mut self) -> Option<StatementType> {

        let peek_token = self.next_token.clone();

        let identifier = match peek_token {
            Token::Ident(_s) => {
                self.advance_tokens();
                Identifier::new(self.curr_token.clone())
            },
            _ => return None,
        };

        if !self.advance_if_expected(Token::Assign) {
            return None;
        } 
        
        // TODO:
        while !self.curr_token_is(Token::Semicolon) {
            self.advance_tokens();
        }

        Some(StatementType::LetStatment(Token::Let, identifier, None))
    }

    fn curr_token_is(&self, token: Token) -> bool {
        self.curr_token == token
    }

    fn next_token_is(&self, token: Token) -> bool {
        self.next_token == token
    }

    fn advance_if_expected(&mut self, token: Token) -> bool {
        if self.next_token_is(token) {
            self.advance_tokens();
            return true;
        }
        false
    }


}

#[cfg(test)]
mod tests {
    use super::{*, ast::{Statement, NodeType}};


    #[test]
    fn test_many_let_statements() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        if program.statments.len() != 3 {
            panic!("program statements does not contain 3 statments. Got {}", program.statments.len());
        } else {
            let tests = vec!["x", "y", "foobar"];

            for i in 0..tests.len() {
                if !test_let_statement(&program.statments[i], tests[i]) {
                    panic!()
                }
            }
        }
    }

    fn test_let_statement(stmt: &Statement, name: &str) -> bool {
        if let StatementType::LetStatment(_tok @ Token::Let, ident, _expr) = &stmt.kind {

            if let Some(ident_name) = ident.get_name() {
                return ident_name == name.to_string();
            } else {
                return false;
            }
        }
        false
    }
}