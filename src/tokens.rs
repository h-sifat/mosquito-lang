#[derive(Debug)]
pub enum Token {
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Semicolon,
    Dot,
    Eof,
}

#[derive(Debug)]
pub struct Spanned {
    pub token: Token,
    pub line: usize,
}
