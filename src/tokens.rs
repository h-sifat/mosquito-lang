use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
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

    Number(f64),
}

#[derive(Debug, PartialEq)]
pub struct Spanned {
    pub token: TokenType,
    pub line: usize,
}

pub static TOKEN_MAP: LazyLock<HashMap<char, TokenType>> = LazyLock::new(|| {
    HashMap::from([
        ('(', TokenType::LParen),
        (')', TokenType::RParen),
        ('{', TokenType::LBrace),
        ('}', TokenType::RBrace),
        ('[', TokenType::LBracket),
        (']', TokenType::RBracket),
        (',', TokenType::Comma),
        (';', TokenType::Semicolon),
        ('.', TokenType::Dot),
    ])
});
