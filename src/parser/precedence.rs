use crate::token::Token;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equality,    // == or !=
    LessGreater, // < or >
    AddSubtract, // + or -
    TimesDivide, // * or /
    Prefix,      // -X or !X
    Call,        // myFunction(X)
}

impl From<&Token> for Precedence {
    fn from(value: &Token) -> Self {
        match value {
            token![==] | token![!=] => Precedence::Equality,
            token![<] | token![>] => Precedence::LessGreater,
            token![+] | token![-] => Precedence::AddSubtract,
            token![*] | token![/] => Precedence::TimesDivide,
            token!['('] => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}
