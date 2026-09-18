use crate::tokens::{Spanned, Token};

pub struct Lexer {
    source: Vec<char>,
    cursor: usize,
    pub line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        return Lexer {
            source: source.chars().collect(),
            cursor: 0,
            line: 0,
        };
    }

    pub fn peek(&self) -> Option<char> {
        let (_, _, res_char) = self.do_advance();
        res_char
    }

    fn advance(&mut self) -> Option<char> {
        let (next_idx, line_idx, res_char) = self.do_advance();

        self.cursor = next_idx;
        self.line = line_idx;

        res_char
    }

    /// returns `(next_idx, line_idx, char)`
    fn do_advance(&self) -> (usize, usize, Option<char>) {
        let src = &self.source;

        let mut in_comment = false;
        let mut idx = self.cursor;
        let mut line = self.line;
        let mut res_char: Option<char> = None;

        while idx < src.len() {
            if res_char.is_some() {
                return (idx, line, res_char);
            }

            let curr = src[idx];

            match curr {
                '/' => {
                    if let Some('/') = src.get(idx + 1) {
                        in_comment = true;
                        idx += 2;
                    } else {
                        res_char = Some('/');
                    }
                }
                '\n' if in_comment => {
                    in_comment = false;
                    line += 1;
                }
                _ => {
                    if !in_comment && curr != ' ' {
                        res_char = Some(curr);
                    }
                }
            }

            idx += 1;
        }

        (idx, line, None)
    }

    pub fn tokenize(&mut self) -> Vec<Spanned> {
        let mut spanned_tokens = vec![];

        loop {
            if let Some(_c) = self.advance() {
                todo!()
            } else {
                let spanned = Spanned {
                    line: self.line,
                    token: Token::Eof,
                };

                spanned_tokens.push(spanned);
                break;
            }
        }

        spanned_tokens
    }
}
