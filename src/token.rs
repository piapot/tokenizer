#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Whitespace,
    Linefeed,
    Digit,
    Word,
    Symbol,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    text: Vec<u8>,
    line: usize,
    span: [usize; 2],
}

impl Token {
    pub fn new(kind: TokenKind, text: Vec<u8>, line: usize, span: [usize; 2]) -> Self {
        Self {
            kind,
            text,
            line,
            span,
        }
    }
}
