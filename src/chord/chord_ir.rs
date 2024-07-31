use super::{
    semantics::{Note, NoteDescriptor, SemInterval},
    Chord,
};
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Omit {
    pub five: bool,
    pub third: bool,
}

impl Omit {
    pub fn new() -> Omit {
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChordIr {
    pub root: Option<Note>,
    pub bass: Option<Note>,
    pub notes: Vec<NoteDescriptor>,
    pub adds: Vec<SemInterval>,
    pub omits: Omit,
    pub is_sus: bool,
}

impl ChordIr {
    pub fn new() -> ChordIr {
        ChordIr {
            root: None,
            bass: None,
            notes: Vec::new(),
            adds: Vec::new(),
            omits: Omit::new(),
            is_sus: false,
        }
    }

    pub fn is_minor(&self) -> bool {
        self.notes.iter().any(|n| match n.sem_interval {
            // 3 is the minor third
            SemInterval::Third => n.semitone == 3,
            _ => false,
        })
    }

    pub fn has(&self, int: SemInterval) -> bool {
        self.notes.iter().any(|n| n.sem_interval == int)
    }
    pub fn has_add(&self, int: SemInterval) -> bool {
        self.adds.iter().any(|n| *n == int)
    }

    pub fn sort_notes(&mut self) {
        self.notes.sort_by(|a, b| a.semitone.cmp(&b.semitone))
    }

    pub fn get_notes(&mut self) -> Vec<Note> {
        let mut notes = Vec::new();
        self.sort_notes();
        if let Some(root) = &self.root {
            for n in &self.notes {
                let note = root.get_note(n.semitone, n.sem_interval.to_int());
                notes.push(note);
            }
        }
        notes
    }

    pub fn to_chord(&mut self) -> Chord {
        self.sort_notes();
        let mut semitones = Vec::new();
        let mut semantic_intervals = Vec::new();
        let mut real_intervals = Vec::new();
        let notes = self.get_notes();
        let note_literals = notes.iter().map(|a| a.to_string()).collect::<Vec<String>>();

        for e in &self.notes {
            semitones.push(e.semitone);
            semantic_intervals.push(e.sem_interval.to_int());
            real_intervals.push(Note::to_real_interval(e.sem_interval.to_int(), e.semitone))
        }

        Chord {
            root: self.root.clone().unwrap(),
            bass: self.bass.clone(),
            notes,
            note_literals,
            semantic_intervals,
            real_intervals,
            is_sus: self.is_sus,
            semitones,
        }
    }
}

impl Default for ChordIr {
    fn default() -> Self {
        Self::new()
    }
}
