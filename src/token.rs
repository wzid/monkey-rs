#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,
    
    // Identifiers + literals
    Ident(String),
    Int(i32),
    
    // Operators
    Assign,
    Negate,
    Plus,
    Minus,
    Multiply,
    Divide,
    
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    
    // Delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // Reseved keywords
    Func,
    Let,
    If,
    True,
    False,
    Else,
    Return,
}

pub fn lookup_identifier(identifier: &str) -> Token {
    match identifier {
        "fn" => Token::Func,
        "let" => Token::Let,
        "if" => Token::If,
        "true" => Token::True,
        "false" => Token::False,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Ident(String::from(identifier))
    }
}