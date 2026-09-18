use crate::tokens::{Spanned, TOKEN_MAP, TokenType};

pub struct Lexer {
    source: Vec<char>,
    cursor_idx: usize,
    pub line_num: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        return Lexer {
            source: source.chars().collect(),
            cursor_idx: 0,
            line_num: 0,
        };
    }

    pub fn peek(&self) -> Option<char> {
        let (_, _, res_char) = self.do_advance();
        res_char
    }

    fn advance(&mut self) -> Option<char> {
        let (next_idx, line_num, res_char) = self.do_advance();

        println!("advance: {}, {}, {:?}", next_idx, line_num, res_char);

        self.cursor_idx = next_idx;
        self.line_num = line_num;

        res_char
    }

    /// returns `(next_idx, line_idx, char)`
    fn do_advance(&self) -> (usize, usize, Option<char>) {
        let src = &self.source;

        let mut in_comment = false;
        let mut idx = self.cursor_idx;
        let mut line = self.line_num;
        let mut res_char: Option<char> = None;

        if line == 0 {
            line = 1;
        }

        while idx < src.len() {
            if res_char.is_some() {
                return (idx, line, res_char);
            }

            let curr = src[idx];
            println!(
                "while --> idx: {}, in_comment: {}, curr: '{}'",
                idx, in_comment, curr
            );

            match curr {
                '/' => {
                    if let Some('/') = src.get(idx + 1) {
                        in_comment = true;
                        idx += 2;
                    } else {
                        res_char = Some('/');
                    }
                }
                '\n' => {
                    if in_comment {
                        in_comment = false;
                    }

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

        (idx, line, res_char)
    }

    pub fn tokenize(&mut self) -> Vec<Spanned> {
        let mut tokens = vec![];

        loop {
            if let Some(c) = self.advance() {
                let token = TOKEN_MAP
                    .get(&c)
                    .copied()
                    .expect(&format!("Unknown token '{}'!", c));

                tokens.push(Spanned {
                    token,
                    line: self.line_num,
                });
            } else {
                tokens.push(Spanned {
                    token: TokenType::Eof,
                    line: self.line_num,
                });
                break;
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests;
