use std::{iter::Peekable, str::Chars};

use regex::Regex;

use crate::token::{Token, TokenType};
static EXTENSIONS: &str = r"\b(?:2|3|4|5|6|7|9|11|13)\b";
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    current: usize,
    reg_alt: Regex,
}

impl Lexer {
    pub fn new() -> Lexer {
        let reg_alt = Regex::new(EXTENSIONS).unwrap();
        Lexer {
            source: String::from(""),
            tokens: Vec::new(),
            current: 0,
            reg_alt,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self, source: &str) -> Vec<Token> {
        self.set_source(source);
        let source = self.source.clone();
        let mut iter = source.chars().peekable();
        while !self.is_at_end() {
            self.scan_token(&mut iter);
        }
        self.add_token(TokenType::Eof, self.current + 1);
        let res = self.tokens.clone();
        self.source = String::new();
        self.tokens.clear();
        self.current = 0;
        res
    }

    fn set_source(&mut self, source: &str) {
        source.clone_into(&mut self.source);
    }

    fn scan_token(&mut self, chars: &mut Peekable<Chars>) {
        let c = self.advance(chars);
        match c {
            None => (),
            Some(c) => match c {
                '#' | '♯' => self.add_token(TokenType::Sharp, self.current),
                '♭' => self.add_token(TokenType::Flat, self.current),
                '△' => self.add_token(TokenType::Maj7, self.current),
                '-' => self.add_token(TokenType::Minor, self.current),
                '°' => self.add_token(TokenType::Dim, self.current),
                'ø' => self.add_token(TokenType::HalfDim, self.current),
                '/' => self.add_token(TokenType::Slash, self.current),
                '+' => self.add_token(TokenType::Aug, self.current),
                ' ' => (),
                ',' => self.add_token(TokenType::Comma, self.current),
                '(' => self.add_token(TokenType::LParent, self.current),
                ')' => self.add_token(TokenType::RParent, self.current),
                c => {
                    if c.is_numeric() {
                        let pos = self.current;
                        let mut literal = String::from(c);
                        let p = chars.peek();
                        let mut cond = p.is_some() && p.unwrap().is_numeric();
                        while cond {
                            let c = self.advance(chars).unwrap();
                            literal.push(c);
                            let p = chars.peek();
                            cond = p.is_some() && p.unwrap().is_numeric();
                        }

                        self.parse_number(&literal, pos);
                        return;
                    }
                    if self.is_alphabetic(&c) {
                        let pos = self.current;
                        let mut literal = String::from(c);
                        let p = chars.peek();
                        let mut cond = p.is_some() && self.is_alphabetic(p.unwrap());

                        while cond {
                            let c = self.advance(chars).unwrap();
                            literal.push(c);
                            let p = chars.peek();
                            cond = p.is_some() && self.is_alphabetic(p.unwrap());
                        }
                        self.parse_string(&literal, pos);
                    } else {
                        self.add_token(TokenType::Illegal, self.current);
                    }
                }
            },
        }
    }

    fn parse_string(&mut self, s: &str, pos: usize) {
        let mut start = 0;
        let mut end = s.len();
        let mut is_match = false;
        let mut tokens = Vec::new();
        while end > 0 {
            let substring = &s[start..end];
            if let Some(m) = TokenType::from_string(substring) {
                tokens.push((m, pos + start));
                is_match = true;
                end = start;
                start = 0;
                continue;
            }
            start += 1;
            if end == start {
                start = 0;
                end -= 1;
            }
        }
        if !is_match {
            self.add_token(TokenType::Illegal, pos);
        }
        while let Some((token_type, pos)) = tokens.pop() {
            self.add_token(token_type, pos);
        }
    }

    fn parse_number(&mut self, s: &str, pos: usize) {
        let mut start = 0;
        let mut end = s.len();
        let mut is_match = false;
        while start < s.len() {
            let substring = &s[start..end];
            if self.reg_alt.is_match(substring) {
                self.add_token(TokenType::Extension(substring.to_string()), pos + start);
                is_match = true;
                start = end;
                end = s.len();
                continue;
            }
            end -= 1;
            if end == start {
                end = s.len();
                start += 1;
            }
        }
        if !is_match {
            self.add_token(TokenType::Illegal, pos);
        }
    }

    fn is_alphabetic(&self, c: &char) -> bool {
        c.is_ascii_alphabetic() //&& *c != 'b'
    }

    fn add_token(&mut self, token_type: TokenType, pos: usize) {
        self.tokens.push(Token::new(token_type, pos as u8));
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
