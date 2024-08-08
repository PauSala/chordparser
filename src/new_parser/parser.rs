use std::{iter::Peekable, slice::Iter};

use crate::{
    chord::{
        intervals::Interval,
        note::{Modifier, Note, NoteLiteral},
        Chord,
    },
    lexer::Lexer,
    token::{Token, TokenType},
};

use super::{
    ast::Ast,
    expression::Exp,
    expressions::{
        AddExp, AltExp, AugExp, BassExp, Dim7Exp, DimExp, ExtensionExp, HalfDimExp, MajExp,
        MinorExp, OmitExp, PowerExp, SlashBassExp, SusExp,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Context {
    Omit(bool),
    Add(bool),
    Sus,
    None,
}

pub struct Parser {
    lexer: Lexer,
    errors: Vec<String>,
    ast: Ast,
    op_count: i16,
    context: Context,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            lexer: Lexer::new(),
            errors: Vec::new(),
            ast: Ast::default(),
            op_count: 0,
            context: Context::None,
        }
    }

    pub fn parse(&mut self, input: &str) -> Chord {
        let binding = self.lexer.scan_tokens(input);
        let mut tokens = binding.iter().peekable();
        self.read_root(&mut tokens);
        self.read_tokens(&mut tokens);
        self.ast.to_chord(input)
    }

    pub fn cleanup(&mut self) {
        self.errors.clear();
        self.ast = Ast::default();
        self.op_count = 0;
        self.context = Context::None;
    }

    fn read_root(&mut self, tokens: &mut Peekable<Iter<Token>>) {
        let note = self.expect_note(tokens);
        match note {
            None => {
                self.errors.push("Expected note literal".to_string());
            }
            Some(n) => {
                self.ast.root = n;
            }
        }
    }

    fn read_tokens(&mut self, tokens: &mut Peekable<Iter<Token>>) {
        let mut next = tokens.next();
        while next.is_some() {
            self.process_token(next.unwrap(), tokens);
            next = tokens.next();
        }
    }

    fn expect_peek(&self, expected: TokenType, tokens: &mut Peekable<Iter<Token>>) -> bool {
        let val = tokens.peek();
        match val {
            None => false,
            Some(real) => real.token_type == expected,
        }
    }

    fn expect_extension(&self, tokens: &mut Peekable<Iter<Token>>) -> bool {
        let val = tokens.peek();
        match val {
            None => false,
            Some(real) => matches!(real.token_type, TokenType::Extension(_)),
        }
    }

    fn process_token(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        match &token.token_type {
            TokenType::Note(_) => self.note(),
            TokenType::Sharp => self.modifier(tokens, Modifier::Sharp),
            TokenType::Flat => self.modifier(tokens, Modifier::Flat),
            TokenType::Aug => self.ast.expressions.push(Exp::Aug(AugExp)),
            TokenType::Dim => self.dim(tokens),
            TokenType::HalfDim => self.ast.expressions.push(Exp::HalfDim(HalfDimExp)),
            TokenType::Extension(ext) => self.extension(ext),
            TokenType::Add => self.add(token, tokens),
            TokenType::Omit => self.omit(token, tokens),
            TokenType::Alt => self.ast.expressions.push(Exp::Alt(AltExp)),
            TokenType::Sus => self.sus(tokens),
            TokenType::Minor => self.ast.expressions.push(Exp::Minor(MinorExp)),
            TokenType::Maj => self.ast.expressions.push(Exp::Maj(MajExp)),
            TokenType::Maj7 => todo!(),
            TokenType::Slash => self.slash(tokens, token),
            TokenType::LParent => self.lparen(tokens),
            TokenType::RParent => self.rparen(),
            TokenType::Comma => self.comma(),
            TokenType::Bass => self.ast.expressions.push(Exp::Bass(BassExp)),
            TokenType::Illegal => self.errors.push("Illegal token".to_string()),
            TokenType::Eof => (),
        }
    }

    fn slash(&mut self, tokens: &mut Peekable<Iter<Token>>, token: &Token) {
        if self.expect_extension(tokens) {
            let alt = tokens.next().unwrap();
            if let TokenType::Extension(a) = &alt.token_type {
                match a.as_str() {
                    "9" => self
                        .ast
                        .expressions
                        .push(Exp::Add(AddExp::new(Interval::Ninth))),
                    _ => {
                        self.errors
                                 .push(format!("Error: Cannot use slash notation for tensions other than 9 at position {}", token.pos));
                    }
                }
            }
        } else {
            match self.expect_note(tokens) {
                None => {
                    self.errors.push(format!(
                        "Error: Expected note literal at position {}",
                        token.pos
                    ));
                }
                Some(b) => {
                    self.ast
                        .expressions
                        .push(Exp::SlashBass(SlashBassExp::new(b)));
                }
            }
        }
    }

    fn expect_note(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Option<Note> {
        let note = tokens.next();
        match note {
            None => {
                self.errors.push("Expected note literal".to_string());
                None
            }
            Some(n) => match &n.token_type {
                TokenType::Note(n) => {
                    let modifier = self.match_modifier(tokens);
                    Some(Note::new(NoteLiteral::from_string(n), modifier))
                }
                _ => {
                    self.errors.push("Expected note literal".to_string());
                    None
                }
            },
        }
    }

    fn dim(&mut self, tokens: &mut Peekable<Iter<Token>>) {
        if self.expect_peek(TokenType::Extension("7".to_owned()), tokens) {
            tokens.next();
            self.ast.expressions.push(Exp::Dim7(Dim7Exp));
            return;
        }
        self.ast.expressions.push(Exp::Dim(DimExp));
    }

    fn rparen(&mut self) {
        if self.op_count != 1 {
            self.errors
                .push("Error: Unexpected closing parenthesis".to_string());
        }
        self.context = Context::None;
        self.op_count -= 1;
    }

    fn lparen(&mut self, tokens: &mut Peekable<Iter<Token>>) {
        self.op_count += 1;
        while tokens.peek().is_some() {
            let token = tokens.next().unwrap();
            match token.token_type {
                TokenType::RParent => {
                    self.op_count -= 1;
                    break;
                }
                TokenType::LParent => {
                    self.errors
                        .push("Error: Nested parenthesis are not allowed ".to_string());
                }
                TokenType::Eof => {
                    self.errors
                        .push("Error: Missing closing parenthesis".to_string());
                    break;
                }
                _ => (),
            }
            // This will advance to next token
            self.process_token(token, tokens);
        }
    }

    fn match_modifier(&self, tokens: &mut Peekable<Iter<Token>>) -> Option<Modifier> {
        let mut modifier = None;
        if self.expect_peek(TokenType::Flat, tokens) {
            tokens.next();
            modifier = Some(Modifier::Flat);
        } else if self.expect_peek(TokenType::Sharp, tokens) {
            tokens.next();
            modifier = Some(Modifier::Sharp);
        }
        modifier
    }

    fn comma(&mut self) {
        match self.context {
            Context::Omit(_) => self.context = Context::Omit(true),
            Context::Add(_) => self.context = Context::Add(true),
            Context::None => (),
            Context::Sus => self.context = Context::None,
        }
    }

    fn omit(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        if self.op_count > 0 {
            self.context = Context::Omit(false);
        }
        if self.expect_peek(TokenType::Extension("5".to_string()), tokens) {
            tokens.next();
            self.ast
                .expressions
                .push(Exp::Omit(OmitExp::new(Interval::PerfectFifth)));
        } else if self.expect_peek(TokenType::Extension("3".to_string()), tokens) {
            tokens.next();
            self.ast
                .expressions
                .push(Exp::Omit(OmitExp::new(Interval::MajorThird)));
        } else {
            self.errors.push(format!(
                "Error: Omit has no target at position {}",
                token.pos
            ));
        }
    }

    fn add(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        if self.op_count > 0 {
            self.context = Context::Add(false);
        }
        let modifier = self.match_modifier(tokens);
        if self.expect_extension(tokens) {
            let next = tokens.next().unwrap();
            if let TokenType::Extension(t) = &next.token_type {
                let mut id = String::new();
                if let Some(m) = modifier {
                    id.push_str(m.to_string().as_str());
                }
                id.push_str(t);
                let interval = Interval::from_chord_notation(&id);
                if let Some(i) = interval {
                    self.ast.expressions.push(Exp::Add(AddExp::new(i)));
                } else {
                    self.errors.push("Invalid extension".to_string());
                }
            }
        } else if self.expect_peek(TokenType::Maj, tokens) {
            tokens.next();
            self.ast
                .expressions
                .push(Exp::Add(AddExp::new(Interval::MajorSeventh)));
            //skip seventh
            tokens.next();
        } else {
            self.errors
                .push(format!("Error: No Add target at pos {}", token.pos));
        }
    }

    fn modifier(&mut self, tokens: &mut Peekable<Iter<Token>>, modifier: Modifier) {
        if self.expect_extension(tokens) {
            let alt = tokens.next().unwrap();
            if let TokenType::Extension(a) = &alt.token_type {
                let mut id = modifier.to_string();
                id.push_str(a);
                let interval = Interval::from_chord_notation(&id);
                if let Some(int) = interval {
                    self.add_interval(int);
                } else {
                    self.errors.push("Invalid extension".to_string());
                }
            }
        } else {
            self.errors.push("Unexpected modifier".to_string());
        }
    }

    fn add_interval(&mut self, int: Interval) {
        match self.context {
            Context::Sus => match int {
                Interval::MinorSecond
                | Interval::MajorSecond
                | Interval::PerfectFourth
                | Interval::AugmentedFourth => {
                    self.ast.expressions.push(Exp::Sus(SusExp::new(int)));
                    self.context = Context::None;
                }
                _ => {
                    self.context = Context::None;
                    self.ast
                        .expressions
                        .push(Exp::Sus(SusExp::new(Interval::PerfectFourth)));
                    self.ast
                        .expressions
                        .push(Exp::Extension(ExtensionExp::new(int)));
                }
            },
            Context::Omit(true) => self.ast.expressions.push(Exp::Omit(OmitExp::new(int))),
            Context::Add(true) => self.ast.expressions.push(Exp::Add(AddExp::new(int))),
            _ => self
                .ast
                .expressions
                .push(Exp::Extension(ExtensionExp::new(int))),
        }
    }

    fn sus(&mut self, tokens: &mut Peekable<Iter<Token>>) {
        self.context = Context::Sus;
        let next = tokens.peek();
        match next {
            Some(t) => match &t.token_type {
                TokenType::Extension(_) | TokenType::Sharp | TokenType::Flat => (),
                _ => self
                    .ast
                    .expressions
                    .push(Exp::Sus(SusExp::new(Interval::PerfectFourth))),
            },
            None => todo!(),
        }
    }

    fn extension(&mut self, ext: &str) {
        if ext == "5" {
            self.ast.expressions.push(Exp::Power(PowerExp));
            return;
        }
        let interval = Interval::from_chord_notation(ext);
        if let Some(int) = interval {
            self.add_interval(int);
        } else {
            self.errors.push("Invalid extension".to_string());
        }
    }

    fn note(&mut self) {
        self.errors.push("Unexpected note".to_string());
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_work() {
        let mut parser = Parser::new();
        let res = parser.parse("CAlt");
        dbg!(res);
        dbg!(&parser.ast.is_valid());
        dbg!(&parser.ast.errors);
        dbg!(&parser.errors);
    }
}
