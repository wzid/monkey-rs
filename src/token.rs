use std::fmt::Display;

use crate::token;

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
#[repr(u8)]
pub enum Token {
    Illegal,
    Eof,

    // Identifiers and literals
    Ident(String),
    Int(i64),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LessThan,
    GreaterThan,

    Equal,
    NotEqual,

    // Delimeters
    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // Keywords
    Func,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            token![ILLEGAL] => write!(f, "ILLEGAL"),
            token![EOF] => write!(f, "EOF"),
            Token::Ident(value) => write!(f, "{}", value),
            Token::Int(value) => write!(f, "{}", value),
            token![=] => write!(f, "="),
            token![+] => write!(f, "+"),
            token![-] => write!(f, "-"),
            token![!] => write!(f, "!"),
            token![*] => write!(f, "*"),
            token![/] => write!(f, "/"),
            token![<] => write!(f, "<"),
            token![>] => write!(f, ">"),
            token![==] => write!(f, "=="),
            token![!=] => write!(f, "!="),
            token![,] => write!(f, ","),
            token![;] => write!(f, ";"),
            token!['('] => write!(f, "("),
            token![')'] => write!(f, ")"),
            token!['{'] => write!(f, "{{"),
            token!['}'] => write!(f, "}}"),
            token![FN] => write!(f, "fn"),
            token![LET] => write!(f, "let"),
            token![TRUE] => write!(f, "true"),
            token![FALSE] => write!(f, "false"),
            token![IF] => write!(f, "if"),
            token![ELSE] => write!(f, "else"),
            token![RETURN] => write!(f, "return"),
        }
    }
}

pub fn lookup_identifier(identifier: &str) -> Token {
    match identifier {
        "fn" => token![FN],
        "let" => token![LET],
        "if" => token![IF],
        "true" => token![TRUE],
        "false" => token![FALSE],
        "else" => token![ELSE],
        "return" => token![RETURN],
        _ => token![IDENT(identifier.to_string())],
    }
}

#[macro_export]
macro_rules! token {
    [ILLEGAL] => { $crate::token::Token::Illegal };
    [EOF] => { crate::token::Token::Eof };
    [IDENT($val:expr)] => { crate::token::Token::Ident($val.to_string()) };
    [INT($val:expr)] => { crate::token::Token::Int($val) };
    [=] => { crate::token::Token::Assign };
    [+] => { crate::token::Token::Plus };
    [-] => { crate::token::Token::Minus };
    [!] => { crate::token::Token::Bang };
    [*] => { crate::token::Token::Asterisk };
    [/] => { crate::token::Token::Slash };
    [<] => { crate::token::Token::LessThan };
    [>] => { crate::token::Token::GreaterThan };
    [==] => { crate::token::Token::Equal };
    [!=] => { crate::token::Token::NotEqual };
    [,] => { crate::token::Token::Comma };
    [;] => { crate::token::Token::Semicolon };
    ['('] => { crate::token::Token::Lparen };
    [')'] => { crate::token::Token::Rparen };
    ['{'] => { crate::token::Token::Lbrace };
    ['}'] => { crate::token::Token::Rbrace };
    [FN] => { crate::token::Token::Func };
    [LET] => { crate::token::Token::Let };
    [TRUE] => { crate::token::Token::True };
    [FALSE] => { crate::token::Token::False };
    [IF] => { crate::token::Token::If };
    [ELSE] => { crate::token::Token::Else };
    [RETURN] => { crate::token::Token::Return };
}