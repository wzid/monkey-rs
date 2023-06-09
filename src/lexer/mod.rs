use crate::{token, token::Token};

#[cfg(test)]
mod tests;

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

    fn read_while(&mut self, mut predicate: impl FnMut(u8) -> bool) {
        while predicate(self.ch) {
            self.read_next_char()
        }
    }

    fn is_next_char(&self, check: u8) -> bool {
        self.next_pos < self.input.len() && check == self.input.as_bytes()[self.next_pos]
    }

    pub fn next_token(&mut self) -> Token {
        self.read_while(|cha| cha.is_ascii_whitespace());

        let tok = match self.ch {
            b'=' => {
                // ==
                if self.is_next_char(b'=') {
                    // Consume the next char
                    self.read_next_char();
                    token![==]
                } else {
                    token![=]
                }
            }
            b'!' => {
                // !=
                if self.is_next_char(b'=') {
                    self.read_next_char();
                    token![!=]
                } else {
                    token![!]
                }
            }
            b'+' => token![+],
            b'-' => token![-],
            b'*' => token![*],
            b'/' => token![/],
            b'<' => token![<],
            b'>' => token![>],

            b';' => token![;],
            b',' => token![,],

            b'(' => token!['('],
            b')' => token![')'],
            b'{' => token!['{'],
            b'}' => token!['}'],

            b'\0' => token![EOF],
            // This makes sure that the identifier consists of letters and/or underscores
            b if is_identifier_or_keyword(b) => return self.identifier_or_keyword(),

            // Since we modify the position in this statement and the statement above we do not
            // want to to modify it again after the switch statement with the extra `self.read_char`
            b if b.is_ascii_digit() => return self.number(),
            _ => token![ILLEGAL],
        };

        self.read_next_char();

        tok
    }

    fn identifier_or_keyword(&mut self) -> Token {
        let start_pos = self.pos;
        
        self.read_while(is_identifier_or_keyword);

        let identifier = &self.input[start_pos..self.pos];

        token::lookup_identifier(identifier)
    }

    fn number(&mut self) -> Token {
        let start_pos = self.pos;
        // Loop while the character is a digit
        self.read_while(|cha| cha.is_ascii_digit());

        let int_str = &self.input[start_pos..self.pos];

        let int = int_str.parse::<i64>().unwrap();

        token![INT(int)]
    }

}
fn is_identifier_or_keyword(check: u8) -> bool {
    matches!(check, b'a'..=b'z' | b'A'..=b'Z' | b'_')
}
