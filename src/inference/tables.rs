use crate::chord::note::Note;

use crate::chord::note::NoteLiteral::*;
use crate::chord::note::NoteModifier;

#[derive(Copy, Clone)]
pub(crate) struct NoteEntry {
    pub notes: [Note; 2],
    pub len: usize,
}

pub(crate) static MIDI_NOTE_TABLE: [NoteEntry; 12] = [
    NoteEntry {
        // 0: C
        notes: [
            Note {
                literal: C,
                modifier: None,
            },
            Note {
                literal: C,
                modifier: None,
            },
        ],
        len: 1,
    },
    NoteEntry {
        // 1: C# / Db
        notes: [
            Note {
                literal: C,
                modifier: Some(NoteModifier(1)),
            },
            Note {
                literal: D,
                modifier: Some(NoteModifier(-1)),
            },
        ],
        len: 2,
    },
    NoteEntry {
        // 2: D
        notes: [
            Note {
                literal: D,
                modifier: None,
            },
            Note {
                literal: D,
                modifier: None,
            },
        ],
        len: 1,
    },
    NoteEntry {
        // 3: D# / Eb
        notes: [
            Note {
                literal: D,
                modifier: Some(NoteModifier(1)),
            },
            Note {
                literal: E,
                modifier: Some(NoteModifier(-1)),
            },
        ],
        len: 2,
    },
    NoteEntry {
        // 4: E
        notes: [
            Note {
                literal: E,
                modifier: None,
            },
            Note {
                literal: E,
                modifier: None,
            },
        ],
        len: 1,
    },
    NoteEntry {
        // 5: F
        notes: [
            Note {
                literal: F,
                modifier: None,
            },
            Note {
                literal: F,
                modifier: None,
            },
        ],
        len: 1,
    },
    NoteEntry {
        // 6: F# / Gb
        notes: [
            Note {
                literal: F,
                modifier: Some(NoteModifier(1)),
            },
            Note {
                literal: G,
                modifier: Some(NoteModifier(-1)),
            },
        ],
        len: 2,
    },
    NoteEntry {
        // 7: G
        notes: [
            Note {
                literal: G,
                modifier: None,
            },
            Note {
                literal: G,
                modifier: None,
            },
        ],
        len: 1,
    },
    NoteEntry {
        // 8: G# / Ab
        notes: [
            Note {
                literal: G,
                modifier: Some(NoteModifier(1)),
            },
            Note {
                literal: A,
                modifier: Some(NoteModifier(-1)),
            },
        ],
        len: 2,
    },
    NoteEntry {
        // 9: A
        notes: [
            Note {
                literal: A,
                modifier: None,
            },
            Note {
                literal: A,
                modifier: None,
            },
        ],
        len: 1,
    },
    NoteEntry {
        // 10: A# / Bb
        notes: [
            Note {
                literal: A,
                modifier: Some(NoteModifier(1)),
            },
            Note {
                literal: B,
                modifier: Some(NoteModifier(-1)),
            },
        ],
        len: 2,
    },
    NoteEntry {
        // 11: B
        notes: [
            Note {
                literal: B,
                modifier: None,
            },
            Note {
                literal: B,
                modifier: None,
            },
        ],
        len: 1,
    },
];

pub fn notes_from_midi(midi: u8) -> &'static [Note] {
    let entry = &MIDI_NOTE_TABLE[(midi % 12) as usize];
    &entry.notes[..entry.len]
}
