#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Punctuations
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

    // Keywords
    Let,
    Const,
    Mut,
    Function,
    Return,
    Class,
    Pub,
    Init,
    SelfKw,
    Static,
    Get,
    New,
    Enum,
    Match,
    If,
    Else,
    While,
    For,
    Typeof,
    Void,
    Break,
    Continue,

    // Operators
    Assignment,
    Plus,
    Minus,
    Multiplication,
    Division,

    // Data Values
    Number(f64),
    Ident(String),
    Bool(bool),
    Char(char),
}

impl TokenType {
    pub fn to_punctuation_token(c: &char) -> Option<TokenType> {
        match c {
            '(' => Some(TokenType::LParen),
            ')' => Some(TokenType::RParen),
            '{' => Some(TokenType::LBrace),
            '}' => Some(TokenType::RBrace),
            '[' => Some(TokenType::LBracket),
            ']' => Some(TokenType::RBracket),
            ',' => Some(TokenType::Comma),
            ';' => Some(TokenType::Semicolon),
            '.' => Some(TokenType::Dot),
            '=' => Some(TokenType::Assignment),
            '+' => Some(TokenType::Plus),
            '-' => Some(TokenType::Minus),
            '*' => Some(TokenType::Multiplication),
            '/' => Some(TokenType::Division),
            _ => None,
        }
    }

    pub fn to_keyword_or_bool_data_value(s: &str) -> Option<TokenType> {
        use TokenType::*;

        match s {
            "true" => Some(Bool(true)),
            "false" => Some(Bool(false)),
            "let" => Some(Let),
            "const" => Some(Const),
            "mut" => Some(Mut),
            "function" => Some(Function),
            "return" => Some(Return),
            "class" => Some(Class),
            "pub" => Some(Pub),
            "init" => Some(Init),
            "self" => Some(SelfKw),
            "static" => Some(Static),
            "get" => Some(Get),
            "new" => Some(New),
            "enum" => Some(Enum),
            "match" => Some(Match),
            "if" => Some(If),
            "else" => Some(Else),
            "while" => Some(While),
            "for" => Some(For),
            "typeof" => Some(Typeof),
            "void" => Some(Void),
            "break" => Some(Break),
            "continue" => Some(Continue),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Spanned {
    pub token: TokenType,
    pub line: usize,
}
