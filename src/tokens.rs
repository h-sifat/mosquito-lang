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
    MultiplicationEqual,
    DivisionEqual,
    Equality,
    Inequality,
    LessThanOrEqual,
    GreaterThanOrEqual,
    FatArrow,
    LessThan,
    GreaterThan,
    Colon,
    Question,
    PlusEqual,
    MinusEqual,
    DoubleColon,
    LogicalNot,
    LogicalOr,
    LogicalAnd,
    LogicalOrEqual,
    LogicalAndEqual,

    // Data Values
    Number(f64),
    Ident(String),
    Bool(bool),
    Char(char),
    StringVal(String),
}

impl TokenType {
    pub fn to_punctuation_token(c: &char) -> Option<TokenType> {
        use TokenType::*;

        match c {
            '(' => Some(LParen),
            ')' => Some(RParen),
            '{' => Some(LBrace),
            '}' => Some(RBrace),
            '[' => Some(LBracket),
            ']' => Some(RBracket),
            ',' => Some(Comma),
            ';' => Some(Semicolon),
            '.' => Some(Dot),
            '?' => Some(Question),
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
