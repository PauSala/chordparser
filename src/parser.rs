//! [&str] to [Chord] parser.

use std::{iter::Peekable, slice::Iter, vec};

use crate::{
    chord::{
        chord_ir::ChordIr,
        intervals::{Interval, SemInterval},
        note::{Modifier, Note, NoteDescriptor, NoteLiteral},
        Chord,
    },
    lexer::Lexer,
    parser_error::ParserErrors,
    token::{Token, TokenType},
    transformer::{
        implicit_eleventh, implicit_fifth, implicit_min_seventh, implicit_ninth, implicit_third,
        remove_omits, Transformer,
    },
    validator::{
        no_double_eleventh, no_double_thirteenth, no_duplicate_seventh, no_minor_and_major_seventh,
        no_minor_and_major_thirds, no_natural_and_altered_nine, no_perfect_fifth_and_altered_fifth,
        Validator,
    },
};

/// This is used to handle (Omit/add a,b) cases.
/// An omit/add modifier inside a parenthesis changes context to Omit(false)/Add(false).
/// When a comma is encountered, if a context exits it is changed to true.  
/// When parents are closed the context is reset to None.  
/// Commas with no context are ignored.  
#[derive(Debug, Clone, PartialEq, Eq)]
enum Context {
    Omit(bool),
    Add(bool),
    None,
}

/// The parser is responsible fo reading and parsing the user input, transforming it into a [Chord] struct.  
/// Every time a chord is parsed the parser is cleared, so its recommended to rehuse the parser instead of creating new ones.  
pub struct Parser {
    lexer: Lexer,
    errors: Vec<String>,
    ir: ChordIr,
    transformers: Vec<Transformer>,
    validators: Vec<Validator>,
    parent_stack: i16,
    context: Context,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            lexer: Lexer::new(),
            errors: Vec::new(),
            ir: ChordIr::new(),
            transformers: vec![
                implicit_third,
                implicit_fifth,
                implicit_min_seventh,
                implicit_ninth,
                implicit_eleventh,
                remove_omits,
            ],
            validators: vec![
                no_minor_and_major_thirds,
                no_perfect_fifth_and_altered_fifth,
                no_duplicate_seventh,
                no_minor_and_major_seventh,
                no_natural_and_altered_nine,
                no_double_eleventh,
                no_double_thirteenth,
            ],
            parent_stack: 0,
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
    /// - M | Ma | Maj modifier is used without a 7, 9, 11 or 13. This not includes the △ modifier, which is allowed to be used alone.
    /// - There are contradictory sevenths (like m7 and Maj7) or multiple ones.
    /// - There are illegal alterations (like #2, b4, #6).
    /// - An alteration has no target.
    /// - There are duplicate tensions, like 11, #11 (except for (b9, #9), which is allowed).
    /// - A sus modifier is not sus2, susb2, sus4 or sus#4.
    /// - An add3 is sharp or flat.
    /// - An Omit modifier has no target (this includes wrong targets: any target which is not a 3 or 5).
    /// - There are more than one sus modifier.
    /// - Slash notation is used for anything other than 9 and 11.
    pub fn parse(&mut self, input: &str) -> Result<Chord, ParserErrors> {
        input.clone_into(&mut self.ir.name);
        let binding = self.lexer.scan_tokens(input);
        let mut tokens = binding.iter().peekable();

        self.read_tokens(&mut tokens);
        self.transform();
        self.validate();

        if !self.errors.is_empty() {
            let errors = self.errors.clone();
            self.clean_up();
            return Err(ParserErrors::new(errors));
        }
        let mut res = self.ir.clone();
        res.sort_by_semitone();
        self.clean_up();
        Ok(res.create_chord())
    }

    fn transform(&mut self) {
        for t in &self.transformers {
            t(&mut self.ir);
        }
    }

    fn validate(&mut self) {
        for v in &self.validators {
            v(&mut self.ir, &mut self.errors);
        }
    }

    fn clean_up(&mut self) {
        self.errors.clear();
        self.ir = ChordIr::new();
        self.parent_stack = 0;
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

    fn expect_note(&self, tokens: &mut Peekable<Iter<Token>>) -> bool {
        let val = tokens.peek();
        match val {
            None => false,
            Some(real) => matches!(real.token_type, TokenType::Note(_)),
        }
    }

    fn process_token(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        match &token.token_type {
            TokenType::Note(n) => self.process_note(n, token, tokens),
            TokenType::Maj => self.process_maj(token, tokens),
            TokenType::Maj7 => self.process_maj7(token, tokens),
            TokenType::Minor => self.process_minor(token, tokens),
            TokenType::Sharp => self.process_modifier(token, tokens, Modifier::Sharp),
            TokenType::Flat => self.process_modifier(token, tokens, Modifier::Flat),
            TokenType::Extension(alt) => self.process_extension(token, alt),
            TokenType::Aug => self.process_aug(token),
            TokenType::Dim => self.process_dim(token, tokens),
            TokenType::HalfDim => self.process_halfdim(token, tokens),
            TokenType::Sus => self.process_sus(token, tokens),
            TokenType::Add => self.process_add(token, tokens),
            TokenType::Omit => self.process_omit(token, tokens),
            TokenType::Alt => self.process_alt(),
            TokenType::Slash => self.process_slash(token, tokens),
            TokenType::LParent => self.process_lparent(tokens),
            TokenType::RParent => self.process_rparent(token),
            TokenType::Illegal => self
                .errors
                .push(format!("Illegal character at position {}", token.pos)),
            TokenType::Eof => (),
            TokenType::Comma => self.process_comma(),
        }
    }

    fn process_comma(&mut self) {
        match self.context {
            Context::Omit(_) => self.context = Context::Omit(true),
            Context::Add(_) => self.context = Context::Add(true),
            Context::None => {}
        }
    }

    fn process_omit(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        if self.parent_stack > 0 {
            self.context = Context::Omit(false);
        }
        if self.expect_peek(TokenType::Extension("5".to_string()), tokens) {
            tokens.next();
            self.ir.omits.five = true;
        } else if self.expect_peek(TokenType::Extension("3".to_string()), tokens) {
            tokens.next();
            self.ir.omits.third = true;
        } else {
            self.errors.push(format!(
                "Error: Omit has no target at position {}",
                token.pos
            ));
        }
    }

    fn process_lparent(&mut self, tokens: &mut Peekable<Iter<Token>>) {
        self.parent_stack += 1;
        while tokens.peek().is_some() {
            let token = tokens.next().unwrap();
            match token.token_type {
                TokenType::RParent => break,
                TokenType::LParent => {
                    self.errors.push(format!(
                        "Error: Nested parenthesis are not allowed at position {}",
                        token.pos
                    ));
                }
                TokenType::Eof => {
                    self.errors.push(format!(
                        "Error: Missing closing parenthesis at position {}",
                        token.pos
                    ));
                    break;
                }
                _ => (),
            }
            // This will advance to next token
            self.process_token(token, tokens);
        }
    }

    fn process_rparent(&mut self, token: &Token) {
        self.parent_stack -= 1;
        self.context = Context::None;
        if self.parent_stack != 0 {
            self.errors.push(format!(
                "Error: Unmatched parenthesis at position {}",
                token.pos
            ));
        }
    }

    fn process_aug(&mut self, token: &Token) {
        self.ir.notes.push(NoteDescriptor::new(
            Interval::MajorThird,
            token.pos as usize,
        ));
        self.ir.notes.push(NoteDescriptor::new(
            Interval::AugmentedFifth,
            token.pos as usize,
        ));
    }

    fn process_slash(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        if self.expect_extension(tokens) {
            let alt = tokens.next().unwrap();
            if let TokenType::Extension(a) = &alt.token_type {
                match a.as_str() {
                    "9" | "11" => self.add_tension(a, token, None, false),
                    _ => {
                        self.errors
                                 .push(format!("Error: Cannot use slash notation for tensions other than 9 and 11 at position {}", token.pos));
                    }
                }
            }
        }
        if self.expect_note(tokens) {
            let note = tokens.next().unwrap();
            let mut modifier = None;
            if self.expect_peek(TokenType::Flat, tokens)
                || self.expect_peek(TokenType::Sharp, tokens)
            {
                let alt = tokens.next().unwrap();
                modifier = {
                    match alt.token_type {
                        TokenType::Sharp => Some(Modifier::Sharp),
                        _ => Some(Modifier::Flat),
                    }
                };
            }
            if self.ir.bass.is_some() {
                self.errors
                    .push(format!("Error: Duplicate bass at position {}", token.pos));
                return;
            }
            if let TokenType::Note(n) = &note.token_type {
                let literal = NoteLiteral::from_string(n);
                self.ir.bass = Some(Note::new(literal, modifier))
            }
        }
    }

    fn process_alt(&mut self) {
        self.ir
            .notes
            .push(NoteDescriptor::new(Interval::DiminishedFifth, 0));
        self.ir
            .notes
            .push(NoteDescriptor::new(Interval::MinorSeventh, 0));
        self.ir
            .notes
            .push(NoteDescriptor::new(Interval::FlatNinth, 0));
        self.ir
            .notes
            .push(NoteDescriptor::new(Interval::SharpNinth, 0));
        self.ir
            .notes
            .push(NoteDescriptor::new(Interval::SharpEleventh, 0));
        self.ir
            .notes
            .push(NoteDescriptor::new(Interval::FlatThirteenth, 0));
    }

    fn process_add(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        if self.parent_stack > 0 {
            self.context = Context::Add(false);
        }
        let mut modifier = None;
        if self.expect_peek(TokenType::Flat, tokens) {
            tokens.next();
            modifier = Some(Modifier::Flat);
        }
        if self.expect_peek(TokenType::Sharp, tokens) {
            tokens.next();
            modifier = Some(Modifier::Sharp);
        }
        if self.expect_peek(TokenType::Maj, tokens) {
            tokens.next();
            self.process_maj(token, tokens);
            return;
        }
        if self.expect_peek(TokenType::Maj7, tokens) {
            tokens.next();
            self.process_maj7(token, tokens);
            return;
        }
        if self.expect_extension(tokens) {
            let next = tokens.next().unwrap();
            if let TokenType::Extension(t) = &next.token_type {
                match t.as_str() {
                    "2" => self.add_tension("9", token, modifier, true),
                    "3" => {
                        if modifier.is_some() {
                            self.errors.push(format!(
                                "Error: Add 3 cannot be sharp or flat at pos {}",
                                token.pos
                            ));
                            return;
                        }
                        self.ir.notes.push(NoteDescriptor::new(
                            Interval::MajorThird,
                            token.pos as usize,
                        ));
                    }
                    "4" => {
                        if modifier.is_some() {
                            self.errors.push(format!(
                                "Error: Add 4 cannot be sharp or flat at pos {}",
                                token.pos
                            ));
                            return;
                        }
                        self.ir.notes.push(NoteDescriptor::new(
                            Interval::PerfectFourth,
                            token.pos as usize,
                        ));
                    }
                    "6" => match modifier {
                        Some(m) => match m {
                            Modifier::Sharp => self
                                .errors
                                .push(format!("Error: A 6 cannot be sharp at pos {}", token.pos)),
                            Modifier::Flat => self.ir.notes.push(NoteDescriptor::new(
                                Interval::MinorSixth,
                                token.pos as usize,
                            )),
                            _ => (),
                        },
                        None => self.ir.notes.push(NoteDescriptor::new(
                            Interval::MajorSixth,
                            token.pos as usize,
                        )),
                    },
                    "9" | "11" | "13" => {
                        self.add_tension(t, token, modifier, true);
                    }
                    _ => self
                        .errors
                        .push(format!("Error: invalid Add target at pos {}", token.pos)),
                }
            }
        } else {
            self.errors
                .push(format!("Error: No Add target at pos {}", token.pos));
        }
    }

    fn process_sus(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        if self.ir.is_sus {
            self.errors.push(format!(
                "Error: A Sus chord should not have more than one Sus modifier at pos {}",
                token.pos
            ));
            return;
        }
        if self.expect_peek(TokenType::Sharp, tokens) {
            if self.expect_peek(TokenType::Extension("4".to_string()), tokens) {
                tokens.next();
                self.ir.notes.push(NoteDescriptor::new(
                    Interval::AugmentedFourth,
                    token.pos as usize,
                ));
                tokens.next();
                return;
            }
        }
        if self.expect_peek(TokenType::Extension("2".to_string()), tokens) {
            tokens.next();
            self.ir
                .notes
                .push(NoteDescriptor::new(Interval::Ninth, token.pos as usize));
            return;
        }
        if self.expect_peek(TokenType::Extension("4".to_string()), tokens) {
            tokens.next();
            self.ir.notes.push(NoteDescriptor::new(
                Interval::PerfectFourth,
                token.pos as usize,
            ));
            self.ir.is_sus = true;
            return;
        }
        self.ir.notes.push(NoteDescriptor::new(
            Interval::PerfectFourth,
            token.pos as usize,
        ));
        self.ir.is_sus = true;
    }

    fn process_dim(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        self.ir.notes.push(NoteDescriptor::new(
            Interval::DiminishedFifth,
            token.pos as usize,
        ));
        self.ir.notes.push(NoteDescriptor::new(
            Interval::MinorThird,
            token.pos as usize,
        ));
        if self.expect_peek(TokenType::Extension("7".to_owned()), tokens) {
            tokens.next();
            self.ir.notes.push(NoteDescriptor::new(
                Interval::DiminishedSeventh,
                token.pos as usize,
            ));
        }
    }

    fn process_halfdim(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        self.ir.notes.push(NoteDescriptor::new(
            Interval::DiminishedFifth,
            token.pos as usize,
        ));
        self.ir.notes.push(NoteDescriptor::new(
            Interval::MinorThird,
            token.pos as usize,
        ));
        if self.expect_peek(TokenType::Extension("7".to_owned()), tokens) {
            tokens.next();
            self.ir.notes.push(NoteDescriptor::new(
                Interval::MinorSeventh,
                token.pos as usize,
            ));
        }
    }

    fn process_note(&mut self, n: &str, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        if self.ir.root.is_some() {
            self.errors.push(format!(
                "Error: multiple root ({}) at line {}",
                n, token.pos
            ));
            return;
        }
        self.ir
            .notes
            .push(NoteDescriptor::new(Interval::Unison, token.pos as usize));
        let literal = NoteLiteral::from_string(n);
        let mut modifier = None;

        if self.expect_peek(TokenType::Flat, tokens) {
            tokens.next();
            modifier = Some(Modifier::Flat);
        } else if self.expect_peek(TokenType::Sharp, tokens) {
            tokens.next();
            modifier = Some(Modifier::Sharp);
        }
        let modifier_str = match &modifier {
            Some(m) => m.to_string(),
            None => "".to_string(),
        };
        self.ir.descriptor = self.ir.name.replace(&format!("{}{}", n, modifier_str), "");
        self.ir.root = Some(Note::new(literal, modifier));
    }
    fn process_maj(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        let extensions = vec!["7", "9", "11", "13"];
        let mut is_used = false;
        for e in extensions {
            if self.expect_peek(TokenType::Extension(e.to_string()), tokens) {
                is_used = true;
                self.ir.notes.push(NoteDescriptor::new(
                    Interval::MajorSeventh,
                    token.pos as usize,
                ));
                if e == "7" {
                    // Skip the tension
                    tokens.next();
                }
                break;
            }
        }
        if !is_used {
            self.errors.push(format!(
                "Error: Maj and its varians alone are not allowed. Use it only followed by a 7, 11, 9 or 13 at position {}",
                token.pos
            ));
        }
    }

    fn process_maj7(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        self.ir.notes.push(NoteDescriptor::new(
            Interval::MajorSeventh,
            token.pos as usize,
        ));
        // Ignore the seventh if exists
        if self.expect_peek(TokenType::Extension("7".to_string()), tokens) {
            tokens.next();
        }
    }

    fn process_minor(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        self.ir.notes.push(NoteDescriptor::new(
            Interval::MinorThird,
            token.pos as usize,
        ));
        if self.expect_peek(TokenType::Extension("7".to_string()), tokens) {
            tokens.next();
            self.ir.notes.push(NoteDescriptor::new(
                Interval::MinorSeventh,
                token.pos as usize,
            ));
        }
    }

    fn process_modifier(
        &mut self,
        token: &Token,
        tokens: &mut Peekable<Iter<Token>>,
        modifier: Modifier,
    ) {
        if self.expect_extension(tokens) {
            let next_token = tokens.next().unwrap();
            let alt = &next_token.token_type.to_string();
            match alt.as_str() {
                "5" => match modifier {
                    Modifier::Sharp => {
                        self.ir.notes.push(NoteDescriptor::new(
                            Interval::AugmentedFifth,
                            token.pos as usize,
                        ));
                    }
                    Modifier::Flat => {
                        self.ir.notes.push(NoteDescriptor::new(
                            Interval::DiminishedFifth,
                            token.pos as usize,
                        ));
                    }
                    _ => (),
                },
                "6" => match modifier {
                    Modifier::Sharp => {
                        self.errors.push(format!(
                            "Error: A 6th cannot be sharp at position {}",
                            token.pos
                        ));
                    }
                    Modifier::Flat => {
                        self.ir.notes.push(NoteDescriptor::new(
                            Interval::MinorSixth,
                            token.pos as usize,
                        ));
                    }
                    _ => (),
                },
                "9" | "11" | "13" => self.add_tension(alt.as_str(), token, Some(modifier), false),
                _ => {
                    self.errors.push(format!(
                        "Error: Illegal alteration at position {}",
                        token.pos
                    ));
                }
            }
        } else {
            self.errors.push(format!(
                "Error: unexpected modifier at position {}",
                token.pos
            ));
        }
    }

    fn process_extension(&mut self, token: &Token, ext: &str) {
        if self.context == Context::Omit(true) && ext != "5" && ext != "3" {
            self.errors.push(format!(
                "Error: Illegal Omit target at position {}",
                token.pos
            ));
            return;
        }
        match ext {
            "3" => {
                if self.context == Context::Omit(true) {
                    self.ir.omits.third = true;
                    return;
                }
                if self.context == Context::Add(true) {
                    self.ir.notes.push(NoteDescriptor::new(
                        Interval::MajorThird,
                        token.pos as usize,
                    ));
                    self.ir.adds.push(Interval::MajorThird);
                    return;
                }
                self.errors.push(format!(
                    "Error: Illegal alteration at position {}",
                    token.pos
                ));
                return;
            }
            "5" => {
                if self.context == Context::Omit(true) {
                    self.ir.omits.five = true;
                    return;
                }
                if self.context == Context::Add(true) {
                    self.errors.push(format!(
                        "Error: Illegal add target at position {}",
                        token.pos
                    ));
                    return;
                }
                // This is the case for power chords, where the third is omited.
                // TODO: Since a power chord is just I + V, maybe we should error if next_token is anything or if this token is not following root
                self.ir.notes.push(NoteDescriptor::new(
                    Interval::PerfectFifth,
                    token.pos as usize,
                ));
                self.ir.omits.third = true
            }
            "6" => {
                if self.ir.has_sem_int(SemInterval::Sixth) {
                    self.errors
                        .push(format!("Error: Duplicate 6th at position {}", token.pos));
                    return;
                }
                if self.context == Context::Add(true) {
                    self.ir.adds.push(Interval::MajorSixth);
                    return;
                }
                self.ir.notes.push(NoteDescriptor::new(
                    Interval::MajorSixth,
                    token.pos as usize,
                ));
            }
            "7" => {
                if self.context == Context::Add(true) {
                    self.errors.push(format!(
                        "Error: Illegal add target at position {}",
                        token.pos
                    ));
                    return;
                }
                self.ir.notes.push(NoteDescriptor::new(
                    Interval::MinorSeventh,
                    token.pos as usize,
                ));
            }
            "9" | "11" | "13" => {
                let mut add = false;
                if self.context == Context::Add(true) {
                    add = true;
                }
                self.add_tension(ext, token, None, add);
            }
            _ => {
                self.errors.push(format!(
                    "Error: Illegal alteration at position {}",
                    token.pos
                ));
            }
        }
    }

    fn add_tension(
        &mut self,
        tension: &str,
        token: &Token,
        modifier: Option<Modifier>,
        is_add: bool,
    ) {
        match tension {
            "9" => match modifier {
                Some(m) => match m {
                    Modifier::Sharp => {
                        self.ir.notes.push(NoteDescriptor::new(
                            Interval::SharpNinth,
                            token.pos as usize,
                        ));
                        if is_add {
                            self.ir.adds.push(Interval::SharpNinth);
                        }
                    }
                    Modifier::Flat => {
                        self.ir
                            .notes
                            .push(NoteDescriptor::new(Interval::FlatNinth, token.pos as usize));
                        if is_add {
                            self.ir.adds.push(Interval::FlatNinth);
                        }
                    }
                    _ => (),
                },
                None => {
                    self.ir
                        .notes
                        .push(NoteDescriptor::new(Interval::Ninth, token.pos as usize));
                    if is_add {
                        self.ir.adds.push(Interval::Ninth);
                    }
                }
            },
            "11" => {
                if let Some(m) = &modifier {
                    match m {
                        Modifier::Sharp => {
                            self.ir.notes.push(NoteDescriptor::new(
                                Interval::SharpEleventh,
                                token.pos as usize,
                            ));
                            if is_add {
                                self.ir.adds.push(Interval::SharpEleventh);
                            }
                        }
                        Modifier::Flat => {
                            self.errors
                                .push(format!("Error: A 11th cannot be flat at pos {}", token.pos));
                        }
                        _ => (),
                    }
                } else {
                    self.ir
                        .notes
                        .push(NoteDescriptor::new(Interval::Eleventh, token.pos as usize));
                    if !self.ir.has_int(Interval::MinorThird) && !is_add {
                        self.ir.is_sus = true;
                    }
                    if is_add {
                        self.ir.adds.push(Interval::Eleventh);
                    }
                }
            }
            "13" => {
                if let Some(m) = &modifier {
                    match m {
                        Modifier::Flat => {
                            self.ir.notes.push(NoteDescriptor::new(
                                Interval::FlatThirteenth,
                                token.pos as usize,
                            ));
                            if is_add {
                                self.ir.adds.push(Interval::FlatThirteenth);
                            }
                        }
                        Modifier::Sharp => {
                            self.errors.push(format!(
                                "Error: A 13th cannot be sharp at pos {}",
                                token.pos
                            ));
                        }
                        _ => (),
                    }
                } else {
                    self.ir.notes.push(NoteDescriptor::new(
                        Interval::Thirteenth,
                        token.pos as usize,
                    ));
                    if is_add {
                        self.ir.adds.push(Interval::Thirteenth);
                    }
                }
            }
            _ => (),
        }
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
    use test_case::test_case;

    #[test_case("Bdim7Maj7119b6")]
    fn should_not_throw_errors(case: &str) {
        let mut parser = Parser::new();
        let _ = parser.parse(case);
        let literals = parser
            .ir
            .get_notes()
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>();
        println!("{:?}", literals);
    }
}
