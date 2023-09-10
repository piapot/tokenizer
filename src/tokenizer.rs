use crate::{Token, TokenKind};

#[derive(Debug)]
pub struct Tokenizer {
    input: Vec<u8>,
    cursor: usize,
    line: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input: Vec::from(input),
            cursor: 0,
            line: 1,
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        if let Some(char) = self.input.get(self.cursor) {
            match *char {
                b'\n' => self.tokenize_linefeed(),
                c if c.is_ascii_whitespace() && c != b'\n' => self.tokenize_whitespace(),
                _ => unreachable!(),
            }
        } else {
            None
        }
    }

    fn tokenize_linefeed(&mut self) -> Option<Token> {
        self.input.get(self.cursor).map(|text| {
            let start = self.cursor;
            let end = start + 1;
            let line = self.line;
            self.cursor = end;
            self.line += 1;
            Token::new(TokenKind::Linefeed, vec![*text], line, [start, end])
        })
    }

    fn tokenize_whitespace(&mut self) -> Option<Token> {
        let start = self.cursor;
        let chars = self.take_while(|c| c.is_ascii_whitespace() && c != b'\n');

        chars.map(|chars| {
            let kind = TokenKind::Whitespace;
            let line = self.line;
            let end = start + chars.len();
            Token::new(kind, chars, line, [start, end])
        })
    }

    fn take_while(&mut self, f: fn(u8) -> bool) -> Option<Vec<u8>> {
        let mut chars = vec![];

        while let Some(char) = self.input.get(self.cursor) {
            if f(*char) {
                self.cursor += 1;
                chars.push(*char)
            } else {
                break;
            }
        }

        match chars.len() == 0 {
            false => Some(chars),
            true => None,
        }
    }
}
