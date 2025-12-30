use super::token::{Token, TokenType};
use std::{iter::Peekable, str::Chars};

pub struct Lexer {
    current: usize,
    input_len: usize,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            input_len: 0,
            current: 0,
        }
    }

    fn is_valid_extension(&self, s: &str) -> bool {
        matches!(s, "2" | "3" | "4" | "5" | "6" | "7" | "9" | "11" | "13")
    }

    pub fn scan_tokens<'a>(
        &mut self,
        source: &'a str,
        tokens: &mut Vec<Token<'a>>,
    ) -> Vec<Token<'a>> {
        self.input_len = source.len();
        let mut iter = source.chars().peekable();
        while !self.is_at_end() {
            self.scan_token(&mut iter, tokens, source);
        }
        self.add_token(TokenType::Eof, self.current + 1, 0, tokens);
        self.current = 0;
        std::mem::take(tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input_len
    }

    fn scan_token<'a>(
        &mut self,
        chars: &mut Peekable<Chars>,
        tokens: &mut Vec<Token<'a>>,
        source: &'a str,
    ) {
        let c = self.advance(chars);
        match c {
            None => (),
            Some(c) => match c {
                '#' | '♯' => self.add_token(TokenType::Sharp, self.current, 1, tokens),
                '♭' => self.add_token(TokenType::Flat, self.current, 1, tokens),
                '△' | '^' | 'Δ' => self.add_token(TokenType::Maj7, self.current, 1, tokens),
                '-' => self.add_token(TokenType::Hyphen, self.current, 1, tokens),
                '°' => self.add_token(TokenType::Dim, self.current, 1, tokens),
                'ø' => self.add_token(TokenType::HalfDim, self.current, 1, tokens),
                '/' => self.add_token(TokenType::Slash, self.current, 1, tokens),
                '+' => self.add_token(TokenType::Aug, self.current, 1, tokens),
                ' ' => (),
                ',' => self.add_token(TokenType::Comma, self.current, 1, tokens),
                '(' => self.add_token(TokenType::LParent, self.current, 1, tokens),
                ')' => self.add_token(TokenType::RParent, self.current, 1, tokens),
                c => {
                    let char_len = c.len_utf8();
                    let start_pos = self.current - char_len;

                    if c.is_numeric() {
                        while chars.peek().is_some_and(|p| p.is_numeric()) {
                            self.advance(chars);
                        }

                        let literal = &source[start_pos..self.current];
                        self.parse_number(literal, start_pos, tokens);
                        return;
                    }

                    if self.is_alphabetic(&c) {
                        while chars.peek().is_some_and(|p| self.is_alphabetic(p)) {
                            self.advance(chars);
                        }

                        let literal = &source[start_pos..self.current];
                        self.parse_string(literal, start_pos, tokens);
                    } else {
                        self.add_token(TokenType::Illegal, start_pos, char_len, tokens);
                    }
                }
            },
        }
    }

    /// Parses a string literal
    /// # Arguments
    /// * `s` - The string to parse
    /// * `pos` - The position of the string in the source
    ///
    /// The parsing is done by checking first all the string and advancing the start to ensure that the last longest match is found.  
    /// For example, for `Cminomit5`, those are the handled parts:  
    /// `Cminomit` -> `minomit` -> `inomit` -> `omit` (match!) -> `Cmin` -> `min` (match!) -> `C` (match!)  
    fn parse_string<'a>(&mut self, input: &'a str, pos: usize, tokens: &mut Vec<Token<'a>>) {
        let mut start = 0;
        while start < input.len() {
            let mut matched = false;
            for end in (start + 1..=input.len()).rev() {
                if !input.is_char_boundary(end) {
                    continue;
                }

                let substring = &input[start..end];
                if let Some(m) = TokenType::from_string(substring) {
                    self.add_token(m, pos + start, substring.len(), tokens);
                    start = end;
                    matched = true;
                    break;
                }
            }

            if !matched && let Some(c) = input[start..].chars().next() {
                let c_len = c.len_utf8();
                self.add_token(TokenType::Illegal, pos + start, c_len, tokens);
                start += c_len;
            }
        }
    }

    fn parse_number<'a>(&mut self, input: &'a str, pos: usize, tokens: &mut Vec<Token<'a>>) {
        let mut start = 0;
        let mut end = input.len();
        let mut errors = Vec::new();
        while start < input.len() {
            let substring = &input[start..end];
            if self.is_valid_extension(substring) {
                self.add_token(
                    TokenType::Extension(substring.parse().unwrap_or(0)),
                    pos + start,
                    substring.len(),
                    tokens,
                );
                start = end;
                end = input.len();
                continue;
            }
            if let Some(last_char) = input[..end].chars().next_back() {
                end -= last_char.len_utf8();
            }
            if end == start {
                errors.push((TokenType::Illegal, pos + start));
                end = input.len();
                start += 1;
            }
        }

        while let Some((token_type, pos)) = errors.pop() {
            self.add_token(token_type, pos, 1, tokens);
        }
    }

    fn is_alphabetic(&self, c: &char) -> bool {
        c.is_ascii_alphabetic()
    }

    fn add_token<'a>(
        &mut self,
        token_type: TokenType<'a>,
        pos: usize,
        len: usize,
        tokens: &mut Vec<Token<'a>>,
    ) {
        tokens.push(Token::new(token_type, pos, len));
    }
    fn advance(&mut self, chars: &mut Peekable<Chars>) -> Option<char> {
        let c = chars.next()?;
        self.current += c.len_utf8();
        Some(c)
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}
