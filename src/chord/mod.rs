use serde::{Deserialize, Serialize};
use serde_json;

use semantics::Note;

pub mod chord_ir;
pub mod semantics;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Chord {
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
    pub fn new(
        root: Note,
        bass: Option<Note>,
        notes: Vec<Note>,
        note_literals: Vec<String>,
        intervals: Vec<u8>,
        semantic_intervals: Vec<u8>,
        real_intervals: Vec<String>,
        is_sus: bool,
    ) -> Chord {
        Chord {
            root,
            bass,
            notes,
            note_literals,
            semitones: intervals,
            semantic_intervals,
            real_intervals,
            is_sus,
        }
    }

    pub fn transpose_to_root(&self, transpose_to: &Note) -> Chord {
        let bass = self.bass.as_ref().map(|bass| self.root.transpose_to(bass, transpose_to));

        let mut notes = Vec::new();
        let semitones = self.semitones.clone();
        let semantic_intervals = self.semantic_intervals.clone();

        for (st, sem_int) in semitones.iter().zip(&semantic_intervals) {
            let note = transpose_to.get_note(*st, *sem_int);
            notes.push(note);
        }

        let note_literals = notes.iter().map(|a| a.to_string()).collect::<Vec<String>>();

        Chord {
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

    pub fn to_json(&self) -> String{
        let a = serde_json::to_string(self);
        match a {
            Ok(v) => v,
            Err(_) => "{{}}".to_string(),
        }
    }
}
