use serde::{Deserialize, Serialize};
use serde_json;

use semantics::Note;

pub mod chord_ir;
pub mod semantics;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Chord {
    pub origin: String,
    pub root: Note,
    pub bass: Option<Note>,
    pub notes: Vec<Note>,
    pub note_literals: Vec<String>,
    pub semitones: Vec<u8>,
    pub semantic_intervals: Vec<u8>,
    pub real_intervals: Vec<String>,
    pub is_sus: bool,
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
        let mut origin = String::from("transposed_from: ");
        origin.push_str(&self.origin);

        Chord {
            origin,
            root: transpose_to.clone(),
            bass,
            notes,
            note_literals,
            semitones,
            semantic_intervals,
            real_intervals: self.real_intervals.clone(),
            is_sus: self.is_sus,
        }
    }

    pub fn to_json(&self) -> String {
        let a = serde_json::to_string(self);
        match a {
            Ok(v) => v,
            Err(_) => "{{}}".to_string(),
        }
    }
}

pub struct ChordBuilder {
    origin: String,
    root: Note,
    bass: Option<Note>,
    notes: Vec<Note>,
    note_literals: Vec<String>,
    semitones: Vec<u8>,
    semantic_intervals: Vec<u8>,
    real_intervals: Vec<String>,
    is_sus: bool,
}

impl ChordBuilder {
    pub fn new(origin: &str, root: Note) -> ChordBuilder {
        ChordBuilder {
            origin: origin.to_string(),
            root,
            bass: None,
            notes: Vec::new(),
            note_literals: Vec::new(),
            semitones: Vec::new(),
            semantic_intervals: Vec::new(),
            real_intervals: Vec::new(),
            is_sus: false,
        }
    }

    pub fn real_intervals(mut self, real_intervals: Vec<String>) -> ChordBuilder {
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

    pub fn build(self) -> Chord {
        Chord {
            origin: self.origin,
            root: self.root,
            bass: self.bass,
            notes: self.notes,
            note_literals: self.note_literals,
            semantic_intervals: self.semantic_intervals,
            real_intervals: self.real_intervals,
            is_sus: self.is_sus,
            semitones: self.semitones,
        }
    }
}
