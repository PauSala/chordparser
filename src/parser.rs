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

/// The parser is responsible fo reading and parsing the user input, transforming it into a [Chord] struct.  
/// Every time a chord is parsed the parser is cleared, so its recommended to rehuse the parser instead of creating new ones.  
pub struct Parser {
    scanner: Lexer,
    errors: Vec<String>,
    ir: ChordIr,
    transformers: Vec<Transformer>,
    validators: Vec<Validator>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            scanner: Lexer::new(),
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
        let binding = self.scanner.scan_tokens(input);
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
            TokenType::RParent => (),
            TokenType::Illegal => self
                .errors
                .push(format!("Illegal character at position {}", token.pos)),
            TokenType::Eof => (),
        }
    }

    fn process_omit(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        if self.expect_peek(TokenType::Extension("5".to_string()), tokens) {
            tokens.next();
            self.ir.omits.five = true;
            // omit 5,3
            if self.expect_peek(TokenType::Extension("3".to_string()), tokens) {
                tokens.next();
                self.ir.omits.third = true;
            }
        } else if self.expect_peek(TokenType::Extension("3".to_string()), tokens) {
            tokens.next();
            self.ir.omits.third = true;
            // omit 3,5
            if self.expect_peek(TokenType::Extension("5".to_string()), tokens) {
                tokens.next();
                self.ir.omits.five = true;
            }
        } else {
            self.errors.push(format!(
                "Error: Omit has no target at position {}",
                token.pos
            ));
        }
    }

    fn process_lparent(&mut self, tokens: &mut Peekable<Iter<Token>>) {
        while tokens.peek().is_some() {
            let token = tokens.next().unwrap();
            match token.token_type {
                TokenType::RParent => return,
                TokenType::Eof => {
                    todo!("Handle unmatched parenthesis")
                }
                _ => (),
            }
            // This will advance to next token
            self.process_token(token, tokens);
        }
    }

    fn process_aug(&mut self, token: &Token) {
        self.ir.notes.push(NoteDescriptor::new(
            SemInterval::Third,
            4,
            token.pos as usize,
        ));
        self.ir.notes.push(NoteDescriptor::new(
            SemInterval::Fifth,
            8,
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
        todo!();
    }

    fn process_add(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        let mut modifier = None;
        if self.expect_peek(TokenType::Flat, tokens) {
            tokens.next();
            modifier = Some(Modifier::Flat);
        }
        if self.expect_peek(TokenType::Sharp, tokens) {
            tokens.next();
            modifier = Some(Modifier::Sharp);
        }
        if self.expect_extension(tokens) {
            let next = tokens.next().unwrap();
            if let TokenType::Extension(t) = &next.token_type {
                match t.as_str() {
                    "9" | "11" | "13" => {
                        self.add_tension(t, token, modifier, true);
                    }
                    "2" => self.add_tension("9", token, modifier, true),
                    // Looks like add 3 appears in real book, but only as a mijor third
                    "3" => {
                        if modifier.is_some() {
                            self.errors.push(format!(
                                "Error: Add 3 cannot be sharp or flat at pos {}",
                                token.pos
                            ));
                            return;
                        }
                        self.ir.notes.push(NoteDescriptor::new(
                            SemInterval::Third,
                            4,
                            token.pos as usize,
                        ));
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
        self.ir.is_sus = true;
        if self.expect_peek(TokenType::Sharp, tokens) {
            tokens.next();
            if self.expect_peek(TokenType::Extension("4".to_string()), tokens) {
                tokens.next();
                self.ir.notes.push(NoteDescriptor::new(
                    SemInterval::Fourth,
                    Interval::AugmentedFourth.st(),
                    token.pos as usize,
                ));
                return;
            }
            self.errors.push(format!(
                "Error: Sus should be sus2, susb2, sus4 or sus#4 at pos {}",
                token.pos
            ));
            return;
        }
        if self.expect_peek(TokenType::Flat, tokens) {
            tokens.next();
            if self.expect_peek(TokenType::Extension("2".to_string()), tokens) {
                tokens.next();
                self.ir.notes.push(NoteDescriptor::new(
                    SemInterval::Second,
                    Interval::MinorSecond.st(),
                    token.pos as usize,
                ));
                return;
            }
            self.errors.push(format!(
                "Error: Sus should be sus2, susb2, sus4 or sus#4 at pos {}",
                token.pos
            ));
            return;
        }
        if self.expect_peek(TokenType::Extension("2".to_string()), tokens) {
            tokens.next();
            self.ir.notes.push(NoteDescriptor::new(
                SemInterval::Second,
                Interval::MajorSecond.st(),
                token.pos as usize,
            ));
            return;
        }
        if self.expect_peek(TokenType::Extension("4".to_string()), tokens) {
            tokens.next();
            self.ir.notes.push(NoteDescriptor::new(
                SemInterval::Fourth,
                Interval::PerfectFourth.st(),
                token.pos as usize,
            ));
            return;
        }
        self.ir.notes.push(NoteDescriptor::new(
            SemInterval::Fourth,
            Interval::PerfectFourth.st(),
            token.pos as usize,
        ));
    }

    fn process_dim(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        self.ir.notes.push(NoteDescriptor::new(
            SemInterval::Fifth,
            Interval::DiminishedFifth.st(),
            token.pos as usize,
        ));
        self.ir.notes.push(NoteDescriptor::new(
            SemInterval::Third,
            Interval::MinorThird.st(),
            token.pos as usize,
        ));
        if self.expect_peek(TokenType::Extension("7".to_owned()), tokens) {
            tokens.next();
            self.ir.notes.push(NoteDescriptor::new(
                SemInterval::Seventh,
                Interval::DiminishedSeventh.st(),
                token.pos as usize,
            ));
        }
    }

    fn process_halfdim(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        self.ir.notes.push(NoteDescriptor::new(
            SemInterval::Fifth,
            Interval::DiminishedFifth.st(),
            token.pos as usize,
        ));
        self.ir.notes.push(NoteDescriptor::new(
            SemInterval::Third,
            Interval::MinorThird.st(),
            token.pos as usize,
        ));
        if self.expect_peek(TokenType::Extension("7".to_owned()), tokens) {
            tokens.next();
            self.ir.notes.push(NoteDescriptor::new(
                SemInterval::Seventh,
                Interval::MinorSeventh.st(),
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
        self.ir.notes.push(NoteDescriptor::new(
            SemInterval::Root,
            0,
            token.pos as usize,
        ));
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
                    SemInterval::Seventh,
                    Interval::MajorSeventh.st(),
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
                "Error: Ma modifier is useless and confusing, use it only with a 7, 11, 9 or 13 at position {}",
                token.pos
            ));
        }
    }

    fn process_maj7(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        self.ir.notes.push(NoteDescriptor::new(
            SemInterval::Seventh,
            Interval::MajorSeventh.st(),
            token.pos as usize,
        ));
        // Ignore the seventh if exists
        if self.expect_peek(TokenType::Extension("7".to_string()), tokens) {
            tokens.next();
        }
    }

    fn process_minor(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        self.ir.notes.push(NoteDescriptor::new(
            SemInterval::Third,
            Interval::MinorThird.st(),
            token.pos as usize,
        ));
        if self.expect_peek(TokenType::Extension("7".to_string()), tokens) {
            tokens.next();
            self.ir.notes.push(NoteDescriptor::new(
                SemInterval::Seventh,
                Interval::MinorSeventh.st(),
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
                            SemInterval::Fifth,
                            Interval::AugmentedFifth.st(),
                            token.pos as usize,
                        ));
                    }
                    Modifier::Flat => {
                        self.ir.notes.push(NoteDescriptor::new(
                            SemInterval::Fifth,
                            Interval::DiminishedFifth.st(),
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
                            SemInterval::Sixth,
                            Interval::MinorSixth.st(),
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
        match ext {
            "5" => {
                // This is the case for power chords, where the third is omited.
                // TODO: Since a power chord is just I + V, maybe we should error if next_token is anything or if this token is not following root
                self.ir.notes.push(NoteDescriptor::new(
                    SemInterval::Fifth,
                    Interval::PerfectFifth.st(),
                    token.pos as usize,
                ));
                self.ir.omits.third = true
            }
            "6" => {
                self.ir.notes.push(NoteDescriptor::new(
                    SemInterval::Sixth,
                    Interval::MajorSixth.st(),
                    token.pos as usize,
                ));
            }
            "7" => {
                self.ir.notes.push(NoteDescriptor::new(
                    SemInterval::Seventh,
                    Interval::MinorSeventh.st(),
                    token.pos as usize,
                ));
            }
            "9" | "11" | "13" => self.add_tension(ext, token, None, false),
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
            "9" => {
                match modifier {
                    Some(m) => match m {
                        Modifier::Sharp => {
                            self.ir.notes.push(NoteDescriptor::new(
                                SemInterval::Ninth,
                                Interval::SharpNinth.st(),
                                token.pos as usize,
                            ));
                        }
                        Modifier::Flat => {
                            self.ir.notes.push(NoteDescriptor::new(
                                SemInterval::Ninth,
                                Interval::FlatNinth.st(),
                                token.pos as usize,
                            ));
                        }
                        _ => (),
                    },
                    None => {
                        self.ir.notes.push(NoteDescriptor::new(
                            SemInterval::Ninth,
                            Interval::Ninth.st(),
                            token.pos as usize,
                        ));
                    }
                }
                if is_add {
                    self.ir.adds.push(SemInterval::Ninth);
                }
            }
            "11" => {
                if let Some(m) = &modifier {
                    match m {
                        Modifier::Sharp => {
                            self.ir.notes.push(NoteDescriptor::new(
                                SemInterval::Eleventh,
                                Interval::SharpEleventh.st(),
                                token.pos as usize,
                            ));
                            if is_add {
                                self.ir.adds.push(SemInterval::Eleventh);
                            }
                        }
                        Modifier::Flat => {
                            self.errors
                                .push(format!("Error: A 11th cannot be flat at pos {}", token.pos));
                        }
                        _ => (),
                    }
                } else {
                    self.ir.notes.push(NoteDescriptor::new(
                        SemInterval::Eleventh,
                        Interval::Eleventh.st(),
                        token.pos as usize,
                    ));
                    if is_add {
                        self.ir.adds.push(SemInterval::Eleventh);
                    }
                }
            }
            "13" => {
                if let Some(m) = &modifier {
                    match m {
                        Modifier::Flat => {
                            self.ir.notes.push(NoteDescriptor::new(
                                SemInterval::Thirteenth,
                                Interval::FlatThirteenth.st(),
                                token.pos as usize,
                            ));
                            if is_add {
                                self.ir.adds.push(SemInterval::Thirteenth);
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
                        SemInterval::Thirteenth,
                        Interval::Thirteenth.st(),
                        token.pos as usize,
                    ));
                    if is_add {
                        self.ir.adds.push(SemInterval::Thirteenth);
                    }
                }
            }
            _ => (),
        }
    }

    //pub
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
