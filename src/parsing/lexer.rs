use super::token::{Token, TokenType};
use regex::Regex;
use std::{iter::Peekable, str::Chars};

static EXTENSIONS: &str = r"\b(?:2|3|4|5|6|7|9|11|13)\b";

pub struct Lexer {
    tokens: Vec<Token>,
    current: usize,
    reg_alt: Regex,
    input_len: usize,
}

impl Lexer {
    pub fn new() -> Lexer {
        // For some reason, generating this with lazy_static! does not improve performance at all.
        let reg_alt = Regex::new(EXTENSIONS).unwrap();
        Lexer {
            input_len: 0,
            tokens: Vec::new(),
            current: 0,
            reg_alt,
        }
    }

    pub fn scan_tokens(&mut self, source: &str) -> Vec<Token> {
        self.input_len = source.len();
        let mut iter = source.chars().peekable();
        while !self.is_at_end() {
            self.scan_token(&mut iter);
        }
        self.add_token(TokenType::Eof, self.current + 1, 0);
        let res = self.tokens.clone();
        self.tokens.clear();
        self.current = 0;
        res
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input_len
    }

    fn scan_token(&mut self, chars: &mut Peekable<Chars>) {
        let c = self.advance(chars);
        match c {
            None => (),
            Some(c) => match c {
                '#' | '♯' => self.add_token(TokenType::Sharp, self.current, 1),
                '♭' => self.add_token(TokenType::Flat, self.current, 1),
                '△' | '^' | 'Δ' => self.add_token(TokenType::Maj7, self.current, 1),
                '-' => self.add_token(TokenType::Hyphen, self.current, 1),
                '°' => self.add_token(TokenType::Dim, self.current, 1),
                'ø' => self.add_token(TokenType::HalfDim, self.current, 1),
                '/' => self.add_token(TokenType::Slash, self.current, 1),
                '+' => self.add_token(TokenType::Aug, self.current, 1),
                ' ' => (),
                ',' => self.add_token(TokenType::Comma, self.current, 1),
                '(' => self.add_token(TokenType::LParent, self.current, 1),
                ')' => self.add_token(TokenType::RParent, self.current, 1),
                c => {
                    if c.is_numeric() {
                        let pos = self.current;
                        let mut literal = String::from(c);
                        let p = chars.peek();
                        let mut cond = p.is_some_and(|p| p.is_numeric());
                        while cond {
                            let c = self.advance(chars).unwrap();
                            literal.push(c);
                            let p = chars.peek();
                            cond = p.is_some_and(|p| p.is_numeric());
                        }

                        self.parse_number(&literal, pos);
                        return;
                    }
                    if self.is_alphabetic(&c) {
                        let pos = self.current;
                        let mut literal = String::from(c);
                        let p = chars.peek();
                        let mut cond = p.is_some_and(|p| self.is_alphabetic(p));

                        while cond {
                            let c = self.advance(chars).unwrap();
                            literal.push(c);
                            let p = chars.peek();
                            cond = p.is_some_and(|p| self.is_alphabetic(p));
                        }
                        self.parse_string(&literal, pos);
                    } else {
                        self.add_token(TokenType::Illegal, self.current, 1);
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
    fn parse_string(&mut self, s: &str, pos: usize) {
        let mut start = 0;
        let mut end = s.len();
        let mut tokens = Vec::new();
        let mut errors = Vec::new();
        while end > 0 {
            let substring = &s[start..end];
            if let Some(m) = TokenType::from_string(substring) {
                tokens.push((m, pos + start, substring.len()));
                end = start;
                start = 0;
                continue;
            }

            start += 1;
            if end == start {
                // dbg!(start, end, substring, pos);
                errors.push((TokenType::Illegal, (pos + start - 1)));
                start = 0;
                end -= 1;
            }
        }

        while let Some((token_type, pos, len)) = tokens.pop() {
            self.add_token(token_type, pos, len);
        }

        while let Some((token_type, pos)) = errors.pop() {
            self.add_token(token_type, pos, 1);
        }
    }

    fn parse_number(&mut self, s: &str, pos: usize) {
        let mut start = 0;
        let mut end = s.len();
        let mut errors = Vec::new();
        while start < s.len() {
            let substring = &s[start..end];
            if self.reg_alt.is_match(substring) {
                self.add_token(
                    TokenType::Extension(substring.parse().unwrap_or(0)),
                    pos + start,
                    substring.len(),
                );
                start = end;
                end = s.len();
                continue;
            }
            end -= 1;
            if end == start {
                errors.push((TokenType::Illegal, pos + start));
                end = s.len();
                start += 1;
            }
        }

        while let Some((token_type, pos)) = errors.pop() {
            self.add_token(token_type, pos, 1);
        }
    }

    fn is_alphabetic(&self, c: &char) -> bool {
        c.is_ascii_alphabetic()
    }

    fn add_token(&mut self, token_type: TokenType, pos: usize, len: usize) {
        self.tokens.push(Token::new(token_type, pos, len));
    }

    fn advance(&mut self, chars: &mut Peekable<Chars>) -> Option<char> {
        self.current += 1;
        chars.next()
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}
