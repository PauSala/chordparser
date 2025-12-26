//! # Chord parsing module
pub(crate) mod ast;
pub(crate) mod expression;
pub(crate) mod expressions;
pub(crate) mod lexer;
pub mod parser_error;
pub(crate) mod token;

use std::{iter::Peekable, slice::Iter};

use ast::Ast;
use expression::Exp;
use expressions::{
    AddExp, AltExp, AugExp, BassExp, Dim7Exp, DimExp, ExtensionExp, HalfDimExp, MajExp, MinorExp,
    OmitExp, PowerExp, SlashBassExp, SusExp,
};
use lexer::Lexer;
use parser_error::{ParserError, ParserErrors};
use token::{Token, TokenType};

use crate::{
    chord::{
        Chord,
        intervals::Interval,
        note::{Modifier, Note, NoteLiteral},
    },
    parsing::expressions::Maj7Exp,
};

/// This is used to handle X(omit/add a,b) cases.
/// An omit/add modifier inside a parenthesis changes context to Omit(false)/Add(false).  
/// When a comma is encountered, if a context exits it is changed to true.    
/// This allows for handling subsequent tokens assuming this context.  
/// So in C7(omit3,5), the 5 is assumed as an omit, but in C7(omit3 5) it is not.
/// When parents are closed the context is reset to None.  
/// Commas with no context are ignored.  
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Context {
    None,
    Sus,
    Group(GroupContext),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GroupContext {
    kind: GroupKind,
    active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GroupKind {
    Omit,
    Add,
}

impl Context {
    fn start_group(kind: GroupKind) -> Self {
        Context::Group(GroupContext {
            kind,
            active: false,
        })
    }

    fn on_comma(self) -> Self {
        match self {
            Context::Group(mut group) => {
                group.active = true;
                Context::Group(group)
            }
            Context::Sus => Context::None,
            Context::None => Context::None,
        }
    }
}

/// The parser is responsible fo reading and parsing the user input, transforming it into a [Chord] struct.  
/// Every time a chord is parsed the parser is cleared, so its recommended to rehuse the parser instead of creating new ones.  
pub struct Parser {
    lexer: Lexer,
    errors: Vec<ParserError>,
    open_parent_count: i16,
    context: Context,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            lexer: Lexer::new(),
            errors: Vec::new(),
            open_parent_count: 0,
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
    /// - There are no Root.
    /// - There are multiple roots.
    /// - There are duplicate basses (like C/E/Eb).
    /// - There are two thirds.
    /// - There are two fifths (except for (b5, #5) which is allowed).
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
        let mut ast = Ast::default();
        let binding = self.lexer.scan_tokens(input);
        let tokens = self.pre_process(&binding);
        let mut tokens = tokens.iter().peekable();

        self.read_root(&mut tokens, &mut ast);
        self.read_tokens(&mut tokens, &mut ast);
        if !self.errors.is_empty() {
            return Err(ParserErrors::new(self.errors.clone()));
        }
        let res = ast.build_chord(input);
        self.cleanup();
        res
    }

    fn cleanup(&mut self) {
        self.errors.clear();
        self.open_parent_count = 0;
        self.context = Context::None;
    }

    fn read_root(&mut self, tokens: &mut Peekable<Iter<Token>>, ast: &mut Ast) {
        match self.expect_note(tokens) {
            Some(note) => ast.root = note,
            None => self.errors.push(ParserError::MissingRootNote),
        }
    }

    fn read_tokens(&mut self, tokens: &mut Peekable<Iter<Token>>, ast: &mut Ast) {
        let mut next = tokens.next();
        while next.is_some() {
            self.process_token(next.unwrap(), tokens, ast);
            next = tokens.next();
        }
    }

    fn process_token(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>, ast: &mut Ast) {
        match &token.token_type {
            TokenType::Note(_) => self.note(token),
            TokenType::Sharp => self.modifier(tokens, Modifier::Sharp, token, ast),
            TokenType::Flat => self.modifier(tokens, Modifier::Flat, token, ast),
            TokenType::Aug => self.aug(tokens, ast),
            TokenType::Dim => self.dim(tokens, ast),
            TokenType::Dim7 => ast.expressions.push(Exp::Dim7(Dim7Exp)),
            TokenType::HalfDim => ast.expressions.push(Exp::HalfDim(HalfDimExp)),
            TokenType::Extension(ext) => self.extension(ext, token, ast),
            TokenType::Add => self.add(token, tokens, ast),
            TokenType::Omit => self.omit(token, tokens, ast),
            TokenType::Alt => ast.expressions.push(Exp::Alt(AltExp)),
            TokenType::Sus => self.sus(tokens, ast),
            TokenType::Minor => ast.expressions.push(Exp::Minor(MinorExp)),
            TokenType::Hyphen => self.hyphen(tokens, token.pos, ast),
            TokenType::Maj => ast.expressions.push(Exp::Maj(MajExp)),
            TokenType::Maj7 => ast.expressions.push(Exp::Maj7(Maj7Exp)),
            TokenType::Slash => self.slash(tokens, token, ast),
            TokenType::LParent => self.lparen(tokens, token.pos, ast),
            TokenType::RParent => self.rparen(token.pos),
            TokenType::Comma => self.comma(),
            TokenType::Bass => ast.expressions.push(Exp::Bass(BassExp)),
            TokenType::Illegal => self.errors.push(ParserError::IllegalToken(token.pos)),
            TokenType::Eof => (),
        }
    }

    fn slash(&mut self, tokens: &mut Peekable<Iter<Token>>, token: &Token, ast: &mut Ast) {
        if let Some(Token {
            token_type: TokenType::Extension(a),
            pos,
            ..
        }) = tokens.next_if(|t| self.is_extension(t))
        {
            match a {
                9 => ast
                    .expressions
                    .push(Exp::Add(AddExp::new(Interval::Ninth, *pos))),
                _ => {
                    let next_pos = tokens.peek().map_or(token.pos, |t| t.pos);
                    self.errors
                        .push(ParserError::IllegalSlashNotation(next_pos));
                }
            }
        } else if let Some(b) = self.expect_note(tokens) {
            ast.expressions.push(Exp::SlashBass(SlashBassExp::new(b)));
        } else {
            let next_pos = tokens.peek().map_or(token.pos, |t| t.pos);
            self.errors
                .push(ParserError::IllegalSlashNotation(next_pos));
        }

        if !self.expect_peek(TokenType::Eof, tokens) {
            let next_pos = tokens.peek().map_or(token.pos, |t| t.pos);
            self.errors
                .push(ParserError::IllegalSlashNotation(next_pos));
        }
    }

    fn hyphen(&mut self, tokens: &mut Peekable<Iter<Token>>, pos: usize, ast: &mut Ast) {
        if tokens
            .next_if(|t| matches!(t.token_type, TokenType::Extension(e) if e == 5))
            .is_some()
        {
            ast.expressions.push(Exp::Extension(ExtensionExp {
                interval: Interval::DiminishedFifth,
                pos,
            }));
        } else {
            ast.expressions.push(Exp::Minor(MinorExp));
        }
    }

    fn aug(&mut self, tokens: &mut Peekable<Iter<Token>>, ast: &mut Ast) {
        let _ = tokens.next_if(|t| matches!(t.token_type, TokenType::Extension(e) if e == 5));
        ast.expressions.push(Exp::Aug(AugExp));
    }

    fn dim(&mut self, tokens: &mut Peekable<Iter<Token>>, ast: &mut Ast) {
        if tokens
            .next_if(|t| matches!(t.token_type, TokenType::Extension(e) if e == 7))
            .is_some()
        {
            ast.expressions.push(Exp::Dim7(Dim7Exp));
        } else {
            ast.expressions.push(Exp::Dim(DimExp));
        }
    }

    fn rparen(&mut self, pos: usize) {
        if self.open_parent_count != 1 {
            self.errors
                .push(ParserError::UnexpectedClosingParenthesis(pos));
        }
        self.context = Context::None;
        self.open_parent_count -= 1;
    }

    fn lparen(&mut self, tokens: &mut Peekable<Iter<Token>>, pos: usize, ast: &mut Ast) {
        self.open_parent_count += 1;
        self.context = Context::None;
        while let Some(token) = tokens.next() {
            match token.token_type {
                TokenType::RParent => {
                    self.open_parent_count -= 1;
                    break;
                }
                TokenType::LParent => {
                    self.errors.push(ParserError::NestedParenthesis(pos));
                }
                TokenType::Eof => {
                    self.errors
                        .push(ParserError::MissingClosingParenthesis(pos));
                    break;
                }
                _ => (),
            }
            // Process next tokens
            self.process_token(token, tokens, ast);
        }
    }

    fn comma(&mut self) {
        self.context = self.context.on_comma();
    }

    fn omit(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>, ast: &mut Ast) {
        if self.open_parent_count > 0 {
            self.context = Context::start_group(GroupKind::Omit);
        }

        if self.consume_extension_if(tokens, 5, || {
            ast.expressions.push(Exp::Omit(OmitExp::new(
                Interval::PerfectFifth,
                token.pos + token.len,
            )));
        }) {
            return;
        }

        if self.consume_extension_if(tokens, 3, || {
            ast.expressions.push(Exp::Omit(OmitExp::new(
                Interval::MajorThird,
                token.pos + token.len,
            )));
        }) {
            return;
        }

        self.errors.push(ParserError::IllegalOrMissingOmitTarget((
            token.pos, token.len,
        )));
    }

    fn add(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>, ast: &mut Ast) {
        if self.open_parent_count > 0 {
            self.context = Context::start_group(GroupKind::Add);
        }

        let modifier = self
            .match_modifier(tokens)
            .map_or(String::new(), |m| m.to_string());

        // Extension after optional modifier
        if let Some(Token {
            token_type: TokenType::Extension(ext),
            pos,
            ..
        }) = tokens.next_if(|t| self.is_extension(t))
        {
            match Interval::from_chord_notation(&format!("{}{}", modifier, ext)) {
                Some(interval) => ast.expressions.push(Exp::Add(AddExp::new(interval, *pos))),
                None => self.errors.push(ParserError::InvalidExtension(token.pos)),
            }
            return;
        }

        // Maj7
        if tokens
            .next_if(|t| matches!(t.token_type, TokenType::Maj7))
            .is_some()
        {
            ast.expressions.push(Exp::Add(AddExp::new(
                Interval::MajorSeventh,
                token.pos + token.len,
            )));
            return;
        }

        self.errors
            .push(ParserError::MissingAddTarget((token.pos, token.len)));
    }

    fn modifier(
        &mut self,
        tokens: &mut Peekable<Iter<Token>>,
        modifier: Modifier,
        token: &Token,
        ast: &mut Ast,
    ) {
        let extension = match tokens.next_if(|t| self.is_extension(t)) {
            Some(Token {
                token_type: TokenType::Extension(ext),
                ..
            }) => ext,
            _ => {
                self.errors.push(ParserError::UnexpectedModifier(token.pos));
                return;
            }
        };

        match Interval::from_chord_notation(&format!("{}{}", modifier, extension)) {
            Some(int) => self.add_interval(int, token.pos, ast),
            None => self
                .errors
                .push(ParserError::InvalidExtension(token.pos + 1)),
        }
    }

    fn is_extension(&self, token: &Token) -> bool {
        matches!(token.token_type, TokenType::Extension(_))
    }

    fn sus(&mut self, tokens: &mut Peekable<Iter<Token>>, ast: &mut Ast) {
        self.context = Context::Sus;

        if !matches!(
            tokens.peek().map(|t| &t.token_type),
            Some(TokenType::Extension(_) | TokenType::Sharp | TokenType::Flat)
        ) {
            ast.expressions
                .push(Exp::Sus(SusExp::new(Interval::PerfectFourth)));
            self.context = Context::None;
        }
    }

    fn add_sus_exp(&mut self, int: Interval, ast: &mut Ast) {
        ast.expressions.push(Exp::Sus(SusExp::new(int)));
        self.context = Context::None;
    }

    fn extension(&mut self, ext: &u8, token: &Token, ast: &mut Ast) {
        if *ext == 5 && self.context == Context::None {
            ast.expressions.push(Exp::Power(PowerExp));
        } else if let Some(int) = Interval::from_chord_notation(&ext.to_string()) {
            self.add_interval(int, token.pos, ast);
        } else {
            self.errors.push(ParserError::InvalidExtension(token.pos));
        }
    }

    fn note(&mut self, token: &Token) {
        self.errors.push(ParserError::UnexpectedNote(token.pos));
    }

    fn add_interval(&mut self, int: Interval, pos: usize, ast: &mut Ast) {
        match self.context {
            Context::Sus => {
                if self.allowed_sus_interval(int) {
                    self.add_sus_exp(int, ast);
                } else {
                    // Csus13 -> here we receive a 13, sus needs to be pushed
                    self.add_sus_exp(Interval::PerfectFourth, ast);
                    ast.expressions
                        .push(Exp::Extension(ExtensionExp::new(int, pos)));
                }
            }
            Context::Group(g) if g.active => match g.kind {
                GroupKind::Omit => ast.expressions.push(Exp::Omit(OmitExp::new(int, pos))),
                GroupKind::Add => ast.expressions.push(Exp::Add(AddExp::new(int, pos))),
            },
            _ => match int {
                // This is for the C4 as Csus case
                Interval::PerfectFourth => ast.expressions.push(Exp::Sus(SusExp::new(int))),
                // #4 is not allowed
                Interval::AugmentedFourth => self.errors.push(ParserError::InvalidExtension(pos)),
                _ => ast
                    .expressions
                    .push(Exp::Extension(ExtensionExp::new(int, pos))),
            },
        }
    }

    fn consume_extension_if<F>(
        &mut self,
        tokens: &mut Peekable<Iter<Token>>,
        target: u8,
        f: F,
    ) -> bool
    where
        F: FnOnce(),
    {
        if let Some(Token {
            token_type: TokenType::Extension(..),
            ..
        }) = tokens.next_if(|t| matches!(t.token_type, TokenType::Extension(e) if e == target))
        {
            f();
            true
        } else {
            false
        }
    }

    fn expect_peek(&self, expected: TokenType, tokens: &mut Peekable<Iter<Token>>) -> bool {
        matches!(tokens.peek(), Some(token) if token.token_type == expected)
    }

    fn allowed_sus_interval(&self, int: Interval) -> bool {
        matches!(
            int,
            Interval::MinorSecond
                | Interval::MajorSecond
                | Interval::PerfectFourth
                | Interval::AugmentedFourth
        )
    }

    /// Returns Some(modifier) and advances tokens or returnes None if any
    fn match_modifier(&self, tokens: &mut Peekable<Iter<Token>>) -> Option<Modifier> {
        match tokens.peek().map(|t| &t.token_type) {
            Some(TokenType::Flat) => {
                tokens.next();
                Some(Modifier::Flat)
            }
            Some(TokenType::Sharp) => {
                tokens.next();
                Some(Modifier::Sharp)
            }
            _ => None,
        }
    }

    fn expect_note(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Option<Note> {
        match &tokens.next()?.token_type {
            TokenType::Note(n) => {
                let modifier = self.match_modifier(tokens);
                Some(Note::new(NoteLiteral::from_string(n), modifier))
            }
            _ => None,
        }
    }

    /// Normalizes the token stream by collapsing 7ths if possible
    fn pre_process(&self, tokens: &[Token]) -> Vec<Token> {
        self.fold_7(
            &self.fold_7(&self.concat_maj7(tokens), TokenType::Dim, TokenType::Dim7),
            TokenType::Maj,
            TokenType::Maj7,
        )
    }

    /// Fold Maj + consecutive 7 into Maj7 Token, including ([Δ |^] + 7)
    fn concat_maj7(&self, tokens: &[Token]) -> Vec<Token> {
        let mut out = Vec::with_capacity(tokens.len());
        let mut i = 0;

        while i < tokens.len() {
            match (&tokens[i].token_type, tokens.get(i + 1)) {
                (TokenType::Maj | TokenType::Maj7, Some(next))
                    if matches!(next.token_type, TokenType::Extension(7)) =>
                {
                    out.push(Token {
                        token_type: TokenType::Maj7,
                        pos: tokens[i].pos,
                        len: tokens[i].len + next.len,
                    });
                    i += 2;
                }

                _ => {
                    out.push(tokens[i].clone());
                    i += 1;
                }
            }
        }

        out
    }

    fn fold_7(
        &self,
        tokens: &[Token],
        match_token: TokenType,
        insert_token_type: TokenType,
    ) -> Vec<Token> {
        let mut out: Vec<Token> = Vec::with_capacity(tokens.len());
        let mut pending_match = Vec::new();
        let mut pending_seven = Vec::new();

        for token in tokens {
            let current_idx = out.len();

            match &token.token_type {
                t if *t == match_token => {
                    if let Some(prev_idx) = pending_seven.pop() {
                        out[prev_idx] =
                            self.merge_tokens(&out[prev_idx], token, &insert_token_type);
                    } else {
                        pending_match.push(current_idx);
                        out.push(token.clone());
                    }
                }
                TokenType::Extension(7) => {
                    if let Some(prev_idx) = pending_match.pop() {
                        out[prev_idx] =
                            self.merge_tokens(&out[prev_idx], token, &insert_token_type);
                    } else {
                        pending_seven.push(current_idx);
                        out.push(token.clone());
                    }
                }
                _ => out.push(token.clone()),
            }
        }
        out
    }

    fn merge_tokens(&self, t1: &Token, t2: &Token, new_type: &TokenType) -> Token {
        Token {
            token_type: new_type.clone(),
            pos: t1.pos.min(t2.pos),
            len: t1.len,
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
