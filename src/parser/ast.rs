use crate::token::Token;

pub struct Node {
    kind: NodeType
}

#[derive(PartialEq)]
pub enum NodeType {
    Root,
    Stmt(Statement),
    Expr(Expression),
}

#[derive(PartialEq, Clone)]
pub struct Statement {
    pub kind: StatementType
}

#[derive(PartialEq, Clone)]
pub enum StatementType {
    // Let token, Identifer struct and the optional expression
    LetStatment(Token, Identifier, Option<Expression>),
}

impl Statement {
    pub fn new(kind: StatementType) -> Self {
        Statement { kind }
    }

    pub fn from(kind: Option<StatementType>) -> Option<Statement> {
        kind.map(Self::new)
    }
}

#[derive(PartialEq, Clone)]
pub struct Expression {
    pub kind: ExpressionType 
}


#[derive(PartialEq, Clone)]
pub enum ExpressionType {

}

#[derive(PartialEq, Clone)]
pub struct Identifier {
    pub token: Token, // Token::Ident(string name)
}

impl Identifier {
    pub fn new(token: Token) -> Self {
        Identifier { token }
    }

    pub fn new_from_value(value : String) -> Self {
        Identifier { token: Token::Ident(value) }
    }

    pub fn get_name(&self) -> Option<String> {
        match &self.token {
            Token::Ident(s) => Some(s.clone()),
            _ => None
        }
    }
}