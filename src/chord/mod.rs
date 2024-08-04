//! # Chords, notes and intervals
use std::vec;

use intervals::{Interval, SemInterval};
use quality::Quality;
use serde::{Deserialize, Serialize};
use serde_json;

use note::Note;

pub(crate) mod chord_ir;
pub mod intervals;
pub mod note;
pub mod quality;

/// Chord representation of a successfully parsed string.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Chord {
    /// The string that originated the chord.
    pub origin: String,
    /// The descriptor of the chord (all beyond its root).
    pub descriptor: String,
    /// The root note of the chord.
    pub root: Note,
    /// The bass note of the chord if any is added with a slash.
    pub bass: Option<Note>,
    /// The quality of the chord.
    pub quality: Quality,
    /// The notes of the chord.
    pub notes: Vec<Note>,
    /// The notes of the chord as string literals.
    pub note_literals: Vec<String>,
    /// The semitones of the notes relative to root.
    pub semitones: Vec<u8>,
    /// The semantic intervals of the notes, meaning non altered intervals.
    pub semantic_intervals: Vec<u8>,
    /// The real intervals of the notes, the actual intervals.
    pub real_intervals: Vec<Interval>,
    pub is_sus: bool,
    /// Intervals added through the add modifier.
    pub adds: Vec<Interval>,
}

impl Chord {
    pub fn builder(origin: &str, root: Note) -> ChordBuilder {
        ChordBuilder::new(origin, root)
    }

    pub fn transpose_to_root(&self, transpose_to: &Note) -> Chord {
        let bass = self
            .bass
            .as_ref()
            .map(|bass| self.root.transpose_to(bass, transpose_to));

        let mut notes = Vec::new();
        let semitones = self.semitones.clone();
        let semantic_intervals = self.semantic_intervals.clone();

        for (st, sem_int) in semitones.iter().zip(&semantic_intervals) {
            let note = transpose_to.get_note(*st, *sem_int);
            notes.push(note);
        }

        let note_literals = notes.iter().map(|a| a.to_string()).collect::<Vec<String>>();

        let mut origin = transpose_to.to_string();
        origin.push_str(&self.descriptor);

        Chord::builder(&origin, transpose_to.clone())
            .descriptor(&self.descriptor)
            .bass(bass)
            .notes(notes)
            .note_literals(note_literals)
            .semitones(semitones)
            .semantic_intervals(semantic_intervals)
            .real_intervals(self.real_intervals.clone())
            .adds(self.adds.clone())
            .is_sus(self.is_sus)
            .build()
    }

    /// Returns the MIDI codes for the chord, centered around central C (60 midi code).
    /// # Arguments
    /// * `self` - The chord to get the MIDI codes from.
    /// # Returns
    /// * A vector of MIDI codes.
    pub fn to_midi_codes(&self) -> Vec<u8> {
        let root = self.root.to_midi_code();
        let mut codes = vec![];
        if let Some(bass) = &self.bass {
            codes.push(bass.to_midi_code() - 12);
            codes.push(root);
        } else {
            codes.push(root - 12);
        }
        for note in self.real_intervals.iter().skip(1) {
            codes.push(note.st() + root);
        }
        codes
    }

    /// Returns the JSON representation of the chord.
    /// # Arguments
    /// * `self` - The chord to get the JSON representation from.
    /// # Returns
    /// * A JSON string.
    pub fn to_json(&self) -> String {
        let a = serde_json::to_string(self);
        match a {
            Ok(v) => v,
            Err(_) => "{{}}".to_string(),
        }
    }

    pub(crate) fn has(&self, int: Interval) -> bool {
        self.real_intervals.iter().any(|n| *n == int)
    }
}

/// Builder for the Chord struct.
pub struct ChordBuilder {
    origin: String,
    descriptor: String,
    root: Note,
    bass: Option<Note>,
    quality: Quality,
    notes: Vec<Note>,
    note_literals: Vec<String>,
    semitones: Vec<u8>,
    semantic_intervals: Vec<u8>,
    real_intervals: Vec<Interval>,
    is_sus: bool,
    adds: Vec<Interval>,
}

impl ChordBuilder {
    pub fn new(origin: &str, root: Note) -> ChordBuilder {
        ChordBuilder {
            origin: origin.to_string(),
            descriptor: String::new(),
            root,
            bass: None,
            quality: Quality::Major,
            notes: Vec::new(),
            note_literals: Vec::new(),
            semitones: Vec::new(),
            semantic_intervals: Vec::new(),
            real_intervals: Vec::new(),
            is_sus: false,
            adds: Vec::new(),
        }
    }

    pub fn real_intervals(mut self, real_intervals: Vec<Interval>) -> ChordBuilder {
        self.real_intervals = real_intervals;
        self
    }

    pub fn semantic_intervals(mut self, semantic_intervals: Vec<u8>) -> ChordBuilder {
        self.semantic_intervals = semantic_intervals;
        self
    }

    pub fn semitones(mut self, semitones: Vec<u8>) -> ChordBuilder {
        self.semitones = semitones;
        self
    }

    pub fn note_literals(mut self, note_literals: Vec<String>) -> ChordBuilder {
        self.note_literals = note_literals;
        self
    }

    pub fn is_sus(mut self, is_sus: bool) -> ChordBuilder {
        self.is_sus = is_sus;
        self
    }

    pub fn notes(mut self, notes: Vec<Note>) -> ChordBuilder {
        self.notes = notes;
        self
    }

    pub fn bass(mut self, bass: Option<Note>) -> ChordBuilder {
        self.bass = bass;
        self
    }

    pub fn descriptor(mut self, descriptor: &str) -> ChordBuilder {
        self.descriptor = descriptor.to_string();
        self
    }

    pub fn adds(mut self, adds: Vec<Interval>) -> ChordBuilder {
        self.adds = adds;
        self
    }

    pub fn build(self) -> Chord {
        let mut chord = Chord {
            origin: self.origin,
            descriptor: self.descriptor,
            root: self.root,
            bass: self.bass,
            quality: self.quality,
            notes: self.notes,
            note_literals: self.note_literals,
            semantic_intervals: self.semantic_intervals,
            real_intervals: self.real_intervals,
            is_sus: self.is_sus,
            semitones: self.semitones,
            adds: self.adds,
        };
        chord.quality = Quality::from_chord(&chord);
        chord
    }
}

pub fn normalize(ch: &Chord) -> String {
    let mut res = ch.root.to_string();
    match ch.quality {
        Quality::Power => {
            res.push('5');
        }
        Quality::Major6 => {
            res.push_str("6");
            let mmod = get_main_mod(ch);
            if let Some(mo) = mmod {
                res.push_str(&mo.to_string());
            }
            return _normalize(ch, res);
        }
        Quality::Major7 => {
            res.push_str("Maj");
            let mut mmod = get_main_mod(ch).unwrap();
            if mmod == Interval::Eleventh && ch.is_sus {
                mmod = Interval::Ninth;
            }
            res.push_str(&mmod.to_string());
            if ch.is_sus {
                res.push_str("sus");
            }
            return _normalize(ch, res);
        }
        Quality::Minor6 => {
            res.push_str("min6");
            let mmod = get_main_mod(ch);
            if let Some(mo) = mmod {
                res.push_str(&mo.to_string());
            }
            return _normalize(ch, res);
        }
        Quality::Minor7 => {
            res.push_str("min");
            let mmod = get_main_mod(ch).unwrap();
            let to_str = {
                if mmod == Interval::MinorSeventh {
                    "7".to_string()
                } else {
                    mmod.to_string()
                }
            };
            res.push_str(&to_str);
            return _normalize(ch, res);
        }
        Quality::MinorMaj7 => todo!(),
        Quality::Minor => todo!(),
        Quality::Dominant => todo!(),
        Quality::SemiDiminished => todo!(),
        Quality::Diminished => todo!(),
        Quality::Augmented => todo!(),
        Quality::Major => todo!(),
    }
    res
}

fn _normalize(ch: &Chord, mut base: String) -> String {
    let mut ext = Vec::new();
    let alter = get_alt_notes(ch);
    for a in alter {
        ext.push(a.to_human_readable());
    }
    let adds = get_adds(ch);
    for (i, a) in adds.iter().enumerate() {
        let mut r = String::new();
        if i == 0 {
            r.push_str("add");
        }
        r.push_str(&a.to_human_readable());
        ext.push(r);
    }
    let omits = get_omits(ch);
    for (i, o) in omits.iter().enumerate() {
        let mut r = String::new();
        if i == 0 {
            r.push_str("omit");
        }
        r.push_str(&o);
        ext.push(r);
    }
    if !ext.is_empty() {
        base.push('(');
        base.push_str(&ext.join(","));
        base.push(')');
    }
    base
}

fn get_omits(ch: &Chord) -> Vec<String> {
    let mut res = Vec::new();
    if !ch
        .semantic_intervals
        .iter()
        .any(|i| *i == SemInterval::Third.numeric() || *i == SemInterval::Fourth.numeric())
        && !ch.has(Interval::Eleventh)
    {
        res.push("3".to_string());
    }
    if !ch
        .semantic_intervals
        .iter()
        .any(|i| *i == SemInterval::Fifth.numeric())
        && !ch.has(Interval::FlatThirteenth)
    {
        res.push("5".to_string());
    }
    res
}

fn get_main_mod(ch: &Chord) -> Option<Interval> {
    match ch.quality {
        Quality::Power => None,
        Quality::Major => None,
        Quality::Minor => None,
        Quality::Major6 | Quality::Minor6 => {
            if ch.has(Interval::Ninth) {
                return Some(Interval::Ninth);
            }
            None
        }
        Quality::Major7 => {
            if ch.has(Interval::Thirteenth)
                && ch
                    .semantic_intervals
                    .iter()
                    .any(|i| i == &SemInterval::Ninth.numeric())
            {
                return Some(Interval::Thirteenth);
            }
            if ch.has(Interval::Eleventh)
                && ch
                    .semantic_intervals
                    .iter()
                    .any(|i| i == &SemInterval::Ninth.numeric())
            {
                return Some(Interval::Eleventh);
            }
            if ch.has(Interval::Ninth) {
                return Some(Interval::Ninth);
            }
            Some(Interval::MajorSeventh)
        }
        Quality::Minor7 => {
            if ch.has(Interval::Thirteenth)
                && ch
                    .semantic_intervals
                    .iter()
                    .any(|i| i == &SemInterval::Ninth.numeric())
                && ch
                    .semantic_intervals
                    .iter()
                    .any(|i| i == &SemInterval::Eleventh.numeric())
            {
                return Some(Interval::Thirteenth);
            }
            if ch.has(Interval::Eleventh)
                && ch
                    .semantic_intervals
                    .iter()
                    .any(|i| i == &SemInterval::Ninth.numeric())
            {
                return Some(Interval::Eleventh);
            }
            if ch.has(Interval::Ninth) {
                return Some(Interval::Ninth);
            }
            Some(Interval::MinorSeventh)
        }
        _ => None,
    }
}

fn get_adds(ch: &Chord) -> Vec<Interval> {
    let mut adds = Vec::new();
    match ch.quality {
        Quality::Power => adds,
        Quality::Major7 => {
            if ch.has(Interval::Thirteenth)
                && !ch
                    .real_intervals
                    .iter()
                    .any(|i| *i == Interval::Eleventh || *i == Interval::Ninth)
            {
                adds.push(Interval::Thirteenth);
            }
            if ch.has(Interval::Eleventh)
                && !ch.real_intervals.iter().any(|i| *i == Interval::Ninth)
            {
                adds.push(Interval::Eleventh);
            }
            adds
        }
        Quality::Minor7 => {
            if ch.has(Interval::Thirteenth)
                && (!ch.real_intervals.iter().any(|i| *i == Interval::Eleventh)
                    || !ch.real_intervals.iter().any(|i| *i == Interval::Ninth))
            {
                adds.push(Interval::Thirteenth);
            }
            if ch.has(Interval::Eleventh)
                && !ch.real_intervals.iter().any(|i| *i == Interval::Ninth)
            {
                adds.push(Interval::Eleventh);
            }
            adds
        }
        Quality::Major6 | Quality::Minor6 => {
            if ch.has(Interval::Eleventh) {
                adds.push(Interval::Eleventh);
            }
            adds
        }
        Quality::Major => todo!(),
        Quality::Minor => todo!(),
        Quality::MinorMaj7 => todo!(),
        Quality::Dominant => todo!(),
        Quality::SemiDiminished => todo!(),
        Quality::Diminished => todo!(),
        Quality::Augmented => todo!(),
    }
}

fn get_alt_notes(ch: &Chord) -> Vec<Interval> {
    let res = Vec::new();
    let altered = [
        Interval::DiminishedFifth,
        Interval::AugmentedFifth,
        Interval::MinorSixth,
        Interval::FlatNinth,
        Interval::SharpNinth,
        Interval::SharpEleventh,
        Interval::FlatThirteenth,
    ];
    let dim: Vec<Interval> = altered
        .iter()
        .filter(|i| *i != &Interval::DiminishedFifth)
        .cloned()
        .collect();
    let aug: Vec<Interval> = altered
        .iter()
        .filter(|i| *i != &Interval::AugmentedFifth)
        .cloned()
        .collect();
    match ch.quality {
        Quality::Power => res,
        Quality::SemiDiminished | Quality::Diminished => ch
            .real_intervals
            .iter()
            .filter(|i| dim.contains(i))
            .cloned()
            .collect(),
        Quality::Augmented => ch
            .real_intervals
            .iter()
            .filter(|i| aug.contains(i))
            .cloned()
            .collect(),
        _ => ch
            .real_intervals
            .iter()
            .filter(|i| altered.contains(i))
            .cloned()
            .collect(),
    }
}

#[cfg(test)]
mod test {
    use crate::{chord::normalize, parser::Parser};

    #[test]
    fn shoudl_work() {
        let mut parser = Parser::new();
        // let res = parser.parse("CMaj7add13b5");
        let res = parser.parse("C-9add11add13");
        match res {
            Ok(c) => {
                dbg!(&c);
                let adds = normalize(&c);
                dbg!(adds);
            }
            Err(e) => {
                dbg!(e);
                panic!();
            }
        }
    }
}
