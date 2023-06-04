use crate::token::{Token, lookup_identifier};

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    next_pos: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            pos: 0,
            next_pos: 0,
            ch: 0,
        };
        
        lexer.read_next_char();

        lexer
    }


    fn read_next_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.next_pos];
        }
        
        self.pos = self.next_pos;
        self.next_pos += 1;
    }

    fn is_next_char(&self, check: u8) -> bool {
        self.next_pos < self.input.len() && check == self.input.as_bytes()[self.next_pos]
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_next_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => {
                // ==
                if self.is_next_char(b'=') {
                    // Consume the next char
                    self.read_next_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            },
            b'!' => {
                // !=
                if self.is_next_char(b'=') {
                    self.read_next_char();
                    Token::NotEqual
                } else {
                    Token::Negate
                }
            },
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Multiply,
            b'/' => Token::Divide,
            b'<' => Token::LessThan,
            b'>' => Token::GreaterThan,

            b';' => Token::Semicolon,
            b',' => Token::Comma,

            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            
            b'\0' => Token::Eof,
            // This makes sure that the identifier consists of letters and/or underscores
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                return self.consume_identifier()
            },
            // Since we modify the position in this statement and the statement above we do not
            // want to to modify it again after the switch statement with the extra `self.read_char`
            b'0'..=b'9' => {
                return self.consume_number()
            },
            _ => Token::Illegal
        };

        self.read_next_char();

        tok
    }

    fn consume_identifier(&mut self) -> Token {
        let start_pos = self.pos;

        // Allow underscores in identifiers like variables
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_next_char();
        }

        let identifier = &self.input[start_pos..self.pos];

        lookup_identifier(identifier)
    }

    fn consume_number(&mut self) -> Token {
        let start_pos = self.pos;

        // Loop while the character is a digit
        while self.ch.is_ascii_digit() {
            self.read_next_char();
        }


        let literal_number = &self.input[start_pos..self.pos];

        // Parse the str slice as an i32
        Token::Int(literal_number.parse().unwrap())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token_basic() {
        let input = "=+(){},;";

        let tests = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
        ];

        let mut lexer = Lexer::new(input);

        for expect in tests {
            let tok = lexer.next_token();

            assert_eq!(expect, tok);
        }
    }

    #[test]
    fn test_next_token_program() {
        let input = r#"
        let five = 5;
        let ten = 10;
        
        let add = fn(x, y) {
            x + y;
        };
        
        let result = add(five, ten);"#;

        let tests = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Func,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::Lparen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
        ];

        let mut lexer = Lexer::new(input);

        for expect in tests {
            let tok = lexer.next_token();

            assert_eq!(expect, tok);
        }
    }

    #[test]
    fn test_next_token_reserved() {
        let input = r#"
        if (5 < 10) {
            return true;
        } else {
            return false;
        }"#;

        let tests = vec![
            Token::If,
            Token::Lparen,
            Token::Int(5),
            Token::LessThan,
            Token::Int(10),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rbrace,
        ];

        let mut lexer = Lexer::new(input);

        for expect in tests {
            let tok = lexer.next_token();

            assert_eq!(expect, tok);
        }
    }

    #[test]
    fn test_next_token_operators() {
        let input = r#"
        !-/*5
        5 < 10 > 5"#;

        let tests = vec![
            Token::Negate,
            Token::Minus,
            Token::Divide,
            Token::Multiply,
            Token::Int(5),
            Token::Int(5),
            Token::LessThan,
            Token::Int(10),
            Token::GreaterThan,
            Token::Int(5)
        ];

        let mut lexer = Lexer::new(input);

        for expect in tests {
            let tok = lexer.next_token();

            assert_eq!(expect, tok);
        }
    }

    #[test]
    fn test_next_token_double_char() {
        let input = r#"
        10 == 10;
        10 != 9;"#;

        let tests = vec![
            Token::Int(10),
            Token::Equal,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEqual,
            Token::Int(9),
            Token::Semicolon,
        ];

        let mut lexer = Lexer::new(input);

        for expect in tests {
            let tok = lexer.next_token();

            assert_eq!(expect, tok);
        }
    }
}
