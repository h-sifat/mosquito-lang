use crate::tokens::{Spanned, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    UnterminatedCharLiteral {
        line: usize,
        start_idx: usize,
    },
    InvalidCharLiteral {
        line: usize,
        start_idx: usize,
        message: String,
    },
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::UnterminatedCharLiteral { line, .. } => {
                write!(f, "line {line}: unterminated character literal")
            }
            LexError::InvalidCharLiteral { line, message, .. } => {
                write!(f, "line {line}: {message}")
            }
        }
    }
}

impl std::error::Error for LexError {}

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    cursor_idx: usize,
    column_idx: usize,
    pub line_num: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            cursor_idx: 0,
            line_num: 1,
            column_idx: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.cursor_idx).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.cursor_idx + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;

        if c == '\n' {
            self.line_num += 1;
            self.column_idx = 0;
        } else {
            self.column_idx += 1;
        }

        self.cursor_idx += 1;

        Some(c)
    }

    fn skip_trivia(&mut self) {
        loop {
            match self.peek() {
                Some(' ') | Some('\t') | Some('\r') | Some('\n') => {
                    self.advance();
                }
                Some('/') if self.peek_next() == Some('/') => {
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }

    fn eat_digits(&mut self, lexeme: &mut String) {
        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() {
                break;
            }

            lexeme.push(c);
            self.advance();
        }
    }

    fn get_escaped_char(c: char) -> Option<char> {
        match c {
            'n' => Some('\n'),
            't' => Some('\t'),
            _ => None,
        }
    }

    /// Before calling this function make sure that the current char is: '
    fn scan_char_literal(&mut self) -> Result<TokenType, LexError> {
        let start_idx = self.column_idx;

        if self.advance() != Some('\'') {
            panic!("Not start of character literal");
        }

        let mut has_escape = false;
        let mut has_reached_end = false;
        let mut res: Vec<char> = vec![];

        while let Some(c) = self.peek() {
            match c {
                '\\' | '\'' => {
                    if has_escape {
                        res.push(c);
                        has_escape = false;
                    } else {
                        if c == '\\' {
                            has_escape = true
                        } else {
                            // end of literal
                            self.advance();
                            has_reached_end = true;
                            break;
                        }
                    }
                }

                '\n' => {
                    return Err(LexError::UnterminatedCharLiteral {
                        start_idx,
                        line: self.line_num,
                    });
                }
                _ => {
                    if has_escape {
                        if let Some(escaped) = Self::get_escaped_char(c) {
                            res.push(escaped);
                        } else {
                            return Err(LexError::InvalidCharLiteral {
                                start_idx,
                                line: self.line_num,
                                message: format!("Invalid escape sequence '\\{c}'"),
                            });
                        }

                        has_escape = false;
                    } else {
                        res.push(c);
                    }
                }
            }

            self.advance();
        }

        if !has_reached_end {
            return Err(LexError::UnterminatedCharLiteral {
                start_idx,
                line: self.line_num,
            });
        }

        let error_message: Option<&str> = if res.is_empty() {
            Some("Empty character literal!")
        } else if res.len() > 1 {
            Some("Multiple characters in char literal!")
        } else {
            None
        };

        if let Some(message) = error_message {
            return Err(LexError::InvalidCharLiteral {
                start_idx,
                line: self.line_num,
                message: message.to_owned(),
            });
        }

        Ok(TokenType::Char(res[0]))
    }

    fn scan_number(&mut self) -> f64 {
        let mut lexeme = String::new();

        self.eat_digits(&mut lexeme);

        // only treat '.' as a decimal point if a digit follows it —
        // otherwise it's member-access Dot and we leave it untouched
        let has_fraction =
            self.peek() == Some('.') && self.peek_next().is_some_and(|c| c.is_ascii_digit());

        if has_fraction {
            lexeme.push('.');
            self.advance();

            self.eat_digits(&mut lexeme);
        }

        lexeme
            .parse::<f64>()
            .unwrap_or_else(|_| panic!("scan_number produced an unparsable lexeme: '{lexeme}'"))
    }

    fn is_start_of_identifier(&self, c: char) -> bool {
        return c.is_ascii_alphabetic() || c == '_';
    }

    /// make sure that the self.is_start_of_identifier(current_char) == true
    fn scan_identifier(&mut self) -> String {
        let mut ident = String::new();

        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        ident
    }

    pub fn tokenize(&mut self) -> Result<Vec<Spanned>, LexError> {
        let mut tokens = vec![];

        loop {
            self.skip_trivia();
            let line = self.line_num;

            let Some(c) = self.peek() else {
                tokens.push(Spanned {
                    token: TokenType::Eof,
                    line,
                });
                break;
            };

            let token = if c.is_ascii_digit() {
                TokenType::Number(self.scan_number())
            } else if let Some(token) = TokenType::to_punctuation_token(&c) {
                self.advance();
                token
            } else if self.is_start_of_identifier(c) {
                let ident = self.scan_identifier();
                TokenType::to_keyword_or_bool_data_value(&ident).unwrap_or(TokenType::Ident(ident))
            } else if c == '\'' {
                self.scan_char_literal()?
            } else {
                println!("Here, c: {c}");
                todo!()
            };

            tokens.push(Spanned { token, line });
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests;
