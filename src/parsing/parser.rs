//! [&str] to [Chord] parser.
//!

use std::{iter::Peekable, slice::Iter};

use crate::{
    chord::{
        intervals::Interval,
        note::{Modifier, Note, NoteLiteral},
        Chord,
    },
    token::{Token, TokenType},
};

use super::{
    ast::Ast,
    expression::Exp,
    expressions::{
        AddExp, AltExp, AugExp, BassExp, Dim7Exp, DimExp, ExtensionExp, HalfDimExp, MajExp,
        MinorExp, OmitExp, PowerExp, SlashBassExp, SusExp,
    },
    lexer::Lexer,
    parser_error::ParserErrors,
};

/// This is used to handle X(omit/add a,b) cases.
/// An omit/add modifier inside a parenthesis changes context to Omit(false)/Add(false).  
/// When a comma is encountered, if a context exits it is changed to true.    
/// This allows for handling subsequent tokens assuming this context.  
/// So in C7(omit3,5), the 5 is assumed as an omit. But in C7(omit3 5) it is not.
/// When parents are closed the context is reset to None.  
/// Commas with no context are ignored.  
#[derive(Debug, Clone, PartialEq, Eq)]
enum Context {
    Omit(bool),
    Add(bool),
    Sus,
    None,
}

/// The parser is responsible fo reading and parsing the user input, transforming it into a [Chord] struct.  
/// Every time a chord is parsed the parser is cleared, so its recommended to rehuse the parser instead of creating new ones.  
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

    /// Parses a chord from a string.
    ///   
    /// # Arguments
    /// * `input` - A string slice that holds the chord to be parsed.
    /// # Returns
    /// * A Result containing a [Chord] if the parsing was successful, otherwise a [ParserErrors] struct.
    ///   
    /// # Rules
    /// There is a set of semantic and syntactic rules to ensure chord's consistency, for now the parser will reject a chord if:  
    /// - There are multiple roots.
    /// - There are duplicate basses (like C/E/Eb).
    /// - There are two thirds.
    /// - There are two fifths (except for (b5, #5) which is allowed).
    /// - The △ modifier is not used as a synonim for Maj7. For example, F△7 will  fail due to multiple 7ths.
    /// - There are contradictory sevenths (like m7 and Maj7) or multiple ones.
    /// - There are illegal alterations (like #2, b4, #6).
    /// - An alteration has no target.
    /// - There are duplicate tensions, like 11, #11 (except for (b9, #9), which is allowed).
    /// - A sus modifier is not sus2, susb2, sus4 or sus#4.
    /// - An add3 is sharp or flat.
    /// - An Omit modifier has no target (this includes wrong targets: any target which is not a 3 or 5).
    /// - There are more than one sus modifier.
    /// - Slash notation is used for anything other than 9 (6/9) or bass notation.
    pub fn parse(&mut self, input: &str) -> Result<Chord, ParserErrors> {
        let binding = self.lexer.scan_tokens(input);
        let mut tokens = binding.iter().peekable();
        self.read_root(&mut tokens);
        self.read_tokens(&mut tokens);
        if self.errors.len() > 0 {
            return Err(ParserErrors::new(self.errors.clone()));
        }
        let res = self.ast.to_chord(input);
        self.cleanup();
        res
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
            TokenType::Maj7 => {
                self.ast.expressions.push(Exp::Maj(MajExp));
                self.ast
                    .expressions
                    .push(Exp::Extension(ExtensionExp::new(Interval::MinorSeventh)));
            }
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
                        self.errors.push(format!(
                            "Cannot use slash notation for tensions other than 9 at position {}",
                            token.pos
                        ));
                    }
                }
            }
        } else {
            match self.expect_note(tokens) {
                None => {
                    self.errors
                        .push(format!("Expected note literal at position {}", token.pos));
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
                .push("Unexpected closing parenthesis".to_string());
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
                        .push("Nested parenthesis are not allowed ".to_string());
                }
                TokenType::Eof => {
                    self.errors.push("Missing closing parenthesis".to_string());
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
            self.errors
                .push(format!("Omit has no target at position {}", token.pos));
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
            if !self.expect_peek(TokenType::Extension("7".to_string()), tokens) {
                self.errors.push("Wrong add target".to_string());
                return;
            }
            //skip seventh
            tokens.next();
        } else {
            self.errors
                .push(format!("No Add target at pos {}", token.pos));
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
