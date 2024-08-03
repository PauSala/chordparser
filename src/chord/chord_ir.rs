use super::{
    intervals::{Interval, SemInterval},
    note::{Note, NoteDescriptor},
    Chord,
};
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Omit {
    pub five: bool,
    pub third: bool,
}

impl Omit {
    pub(crate) fn new() -> Omit {
        Omit {
            five: false,
            third: false,
        }
    }
}
impl Default for Omit {
    fn default() -> Self {
        Self::new()
    }
}

/// Intermediate representation of a chord used by the parser
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct ChordIr {
    pub name: String,
    pub descriptor: String,
    pub bass: Option<Note>,
    pub root: Option<Note>,
    pub notes: Vec<NoteDescriptor>,
    pub adds: Vec<Interval>,
    pub omits: Omit,
    pub is_sus: bool,
}

impl ChordIr {
    pub(crate) fn new() -> ChordIr {
        ChordIr {
            name: String::new(),
            descriptor: String::new(),
            root: None,
            bass: None,
            notes: Vec::new(),
            adds: Vec::new(),
            omits: Omit::new(),
            is_sus: false,
        }
    }

    pub(crate) fn has_minor_third(&self) -> bool {
        self.notes.iter().any(|n| match n.sem_interval {
            SemInterval::Third => n.semitone == Interval::MinorThird.st(),
            _ => false,
        })
    }

    pub(crate) fn has(&self, int: SemInterval) -> bool {
        self.notes.iter().any(|n| n.sem_interval == int)
    }

    pub(crate) fn has_add(&self, int: Interval) -> bool {
        self.adds.iter().any(|n| *n == int)
    }

    pub(crate) fn sort_by_semitone(&mut self) {
        self.notes.sort_by(|a, b| a.semitone.cmp(&b.semitone))
    }

    /// Get the notes of the chord
    pub(crate) fn get_notes(&mut self) -> Vec<Note> {
        let mut notes = Vec::new();
        self.sort_by_semitone();
        if let Some(root) = &self.root {
            for n in &self.notes {
                let note = root.get_note(n.semitone, n.sem_interval.numeric());
                notes.push(note);
            }
        }
        notes
    }

    pub(crate) fn create_chord(&mut self) -> Chord {
        self.sort_by_semitone();
        let mut semitones = Vec::new();
        let mut semantic_intervals = Vec::new();
        let mut real_intervals = Vec::new();
        let notes = self.get_notes();
        let note_literals = notes.iter().map(|a| a.to_string()).collect::<Vec<String>>();

        for e in &self.notes {
            semitones.push(e.semitone);
            semantic_intervals.push(e.sem_interval.numeric());
            real_intervals.push(Note::to_real_interval(e.sem_interval.numeric(), e.semitone))
        }

        Chord::builder(&self.name, self.root.clone().unwrap())
            .descriptor(&self.descriptor)
            .bass(self.bass.clone())
            .notes(notes)
            .note_literals(note_literals)
            .semitones(semitones)
            .semantic_intervals(semantic_intervals)
            .real_intervals(real_intervals)
            .is_sus(self.is_sus)
            .adds(self.adds.clone())
            .build()
    }
}

impl Default for ChordIr {
    fn default() -> Self {
        Self::new()
    }
}
