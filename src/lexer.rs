use crate::{
    lexer::ParseState::Comment,
    tokens::{Spanned, TOKEN_MAP, TokenType},
};

#[derive(Debug, PartialEq, Clone)]
enum NumState {
    Int,
    PendingFrac,
    Frac,
}

#[derive(Debug, PartialEq, Clone)]
enum ParseState {
    Init,
    Pending(char),
    Comment,
    Number {
        state: NumState,
        int: String,
        frac: String,
    },
}

impl ParseState {
    fn to_token(&self) -> Option<TokenType> {
        match &self {
            &Self::Pending(c) => Some(
                TOKEN_MAP
                    .get(&c)
                    .copied()
                    .expect(&format!("Unknown token '{}'!", c)),
            ),
            &Self::Number { int, frac, .. } => {
                let combined = (int.clone() + "." + frac);
                let num = combined
                    .parse::<f64>()
                    .expect(&format!("Invalid number: '{}'!", combined));

                Some(TokenType::Number(num))
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
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

    pub fn peek(&self) -> Option<TokenType> {
        let (_, _, parse_state) = self.do_advance();
        parse_state.and_then(|s| s.to_token())
    }

    fn advance(&mut self) -> Option<TokenType> {
        let (next_idx, line_num, parse_state) = self.do_advance();

        println!("advance: {}, {}, {:?}", next_idx, line_num, parse_state);

        self.cursor_idx = next_idx;
        self.line_num = line_num;

        parse_state.and_then(|s| s.to_token())
    }

    /// returns `(next_idx, line_idx, state)`
    fn do_advance(&self) -> (usize, usize, Option<ParseState>) {
        let src = &self.source;

        let mut idx = self.cursor_idx;
        let mut line = self.line_num;
        let mut parse_state = ParseState::Init;
        let mut res_state: Option<ParseState> = None;

        if line == 0 {
            line = 1;
        }

        while idx < src.len() {
            if res_state.is_some() {
                return (idx, line, res_state);
            }

            let curr = src[idx];

            println!(
                "idx: {idx}, curr: '{curr}', is_digit: {}, res_state: {:?}",
                curr.is_digit(10),
                res_state
            );

            match curr {
                '/' => {
                    if let Some('/') = src.get(idx + 1) {
                        parse_state = ParseState::Comment;
                        idx += 2;
                    } else {
                        parse_state = ParseState::Pending('/');
                    }
                }
                '\n' => {
                    match parse_state {
                        ParseState::Comment => {
                            parse_state = ParseState::Init;
                        }
                        ParseState::Number { .. } => {
                            res_state = Some(parse_state.clone());
                        }
                        _ => {}
                    }

                    line += 1;
                }
                _ => match parse_state {
                    ParseState::Init if curr != ' ' => {
                        if curr.is_digit(10) {
                            parse_state = ParseState::Number {
                                state: NumState::Int,
                                int: curr.to_string(),
                                frac: String::new(),
                            }
                        } else {
                            res_state = Some(ParseState::Pending(curr));
                        }
                    }

                    ParseState::Number {
                        ref mut state,
                        ref mut int,
                        ref mut frac,
                    } => {
                        if curr == '.' {
                            if state == &NumState::Int && idx + 1 == src.len() {
                                return (idx as usize, line, Some(parse_state));
                            }

                            match state {
                                NumState::PendingFrac => {
                                    // return the number and walk back to the prev decimal point idx
                                    return (idx - 1 as usize, line, Some(parse_state));
                                }
                                NumState::Int => {
                                    *state = NumState::PendingFrac;
                                }
                                NumState::Frac => {
                                    // return the number. decimal point after fractional part
                                    return (idx, line, Some(parse_state));
                                }
                            }
                        } else if curr.is_digit(10) {
                            match state {
                                NumState::Int => {
                                    int.push(curr);
                                }
                                _ => {
                                    frac.push(curr);
                                    *state = NumState::Frac;
                                }
                            }
                        } else {
                            // return the number
                            return (idx, line, Some(parse_state));
                        }
                    }
                    _ => {}
                },
            }

            idx += 1;
        }

        (idx, line, res_state.or(Some(parse_state)))
    }

    pub fn tokenize(&mut self) -> Vec<Spanned> {
        let mut tokens = vec![];

        loop {
            if let Some(token) = self.advance() {
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
