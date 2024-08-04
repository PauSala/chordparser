//! # Chords, notes and intervals
use std::vec;

use intervals::Interval;
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
        Quality::Major7 => {
            let mut ext = Vec::new();
            let (main_t, other) = get_this_thing(ch);
            dbg!(&main_t, &other);
            res.push_str("Maj");
            if let Some(t) = main_t {
                if t == Interval::Eleventh {
                    res.push_str("9sus")
                } else if ch.is_sus {
                    res.push_str(&t.to_human_readable());
                    res.push_str("sus");
                } else {
                    res.push_str(&t.to_human_readable());
                }
            } else {
                res.push('7');
            }
            if ch.has(Interval::DiminishedFifth) {
                ext.push("b5".to_owned())
            }
            for (i, e) in other.into_iter().enumerate() {
                let mut s = String::from("");
                if i == 0 {
                    s.push_str("add");
                }
                s.push_str(&e.to_human_readable());
                ext.push(s);
            }
            if !ext.is_empty() {
                res.push('(');
                res.push_str(&ext.join(","));
                res.push(')');
            }
        }
        Quality::Power => {
            res.push('5');
        }
        Quality::Minor => todo!(),
        Quality::Dominant => todo!(),
        Quality::SemiDiminished => todo!(),
        Quality::Diminished => todo!(),
        Quality::Augmented => todo!(),
        Quality::Major6 => todo!(),
        Quality::Major => todo!(),
    }
    res
}

fn get_this_thing(ch: &Chord) -> (Option<Interval>, Vec<Interval>) {
    let mut master = Vec::new();
    let mut other = Vec::new();

    for i in &ch.real_intervals {
        match i {
            Interval::FlatNinth
            | Interval::SharpNinth
            | Interval::SharpEleventh
            | Interval::FlatThirteenth => master.push(*i),
            Interval::Ninth => master.push(*i),
            Interval::Eleventh => {
                if ch.real_intervals.contains(&Interval::Ninth)
                    || ch.real_intervals.contains(&Interval::FlatNinth)
                    || ch.real_intervals.contains(&Interval::SharpNinth)
                {
                    master.retain(|a| *a != Interval::Ninth);
                    master.push(*i);
                } else {
                    other.push(*i)
                }
            }
            Interval::Thirteenth => {
                if ch.has(Interval::MinorThird) {
                    if (ch.real_intervals.contains(&Interval::Ninth)
                        || ch.real_intervals.contains(&Interval::FlatNinth)
                        || ch.real_intervals.contains(&Interval::SharpNinth))
                        && (ch.real_intervals.contains(&Interval::Eleventh)
                            || ch.real_intervals.contains(&Interval::SharpEleventh))
                    {
                        master.push(*i);
                        master.retain(|a| *a != Interval::Ninth && *a != Interval::Eleventh);
                    } else {
                        other.push(*i)
                    }
                } else if ch.real_intervals.contains(&Interval::Ninth)
                    || ch.real_intervals.contains(&Interval::FlatNinth)
                    || ch.real_intervals.contains(&Interval::SharpNinth)
                {
                    master.retain(|a| *a != Interval::Ninth && *a != Interval::Eleventh);
                    master.push(*i);
                } else {
                    other.push(*i)
                }
            }
            _ => (),
        }
    }
    let high = master.pop();
    for i in master {
        other.push(i)
    }

    other.sort_by_key(|a| a.st());
    (high, other)
}

#[cfg(test)]
mod test {
    use crate::{chord::normalize, parser::Parser};

    #[test]
    fn shoudl_work() {
        let mut parser = Parser::new();
        let res = parser.parse("Cmaj9b5add13add11");
        match res {
            Ok(c) => {
                dbg!(&c);
                let adds = normalize(&c);
                dbg!(adds);
            }
            Err(_) => todo!(),
        }
    }
}
