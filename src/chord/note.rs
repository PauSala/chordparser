//! Useful abstractions to work with notes
//!
use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// All possible note literals.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum NoteLiteral {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl NoteLiteral {
    pub fn from_string(i: &str) -> NoteLiteral {
        match i {
            "C" => NoteLiteral::C,
            "D" => NoteLiteral::D,
            "E" => NoteLiteral::E,
            "F" => NoteLiteral::F,
            "G" => NoteLiteral::G,
            "A" => NoteLiteral::A,
            "B" => NoteLiteral::B,
            _ => NoteLiteral::C,
        }
    }
    pub fn from_u8(n: u8) -> NoteLiteral {
        match n % 7 {
            0 => NoteLiteral::C,
            1 => NoteLiteral::D,
            2 => NoteLiteral::E,
            3 => NoteLiteral::F,
            4 => NoteLiteral::G,
            5 => NoteLiteral::A,
            6 => NoteLiteral::B,
            _ => unreachable!(),
        }
    }

    /// Returns the semitones from C in a natural major scale (C=0, D=2, E=4...)
    pub fn natural_semitone(&self) -> u8 {
        match self {
            NoteLiteral::C => 0,
            NoteLiteral::D => 2,
            NoteLiteral::E => 4,
            NoteLiteral::F => 5,
            NoteLiteral::G => 7,
            NoteLiteral::A => 9,
            NoteLiteral::B => 11,
        }
    }
}

impl Display for NoteLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoteLiteral::C => f.write_str("C"),
            NoteLiteral::D => f.write_str("D"),
            NoteLiteral::E => f.write_str("E"),
            NoteLiteral::F => f.write_str("F"),
            NoteLiteral::G => f.write_str("G"),
            NoteLiteral::A => f.write_str("A"),
            NoteLiteral::B => f.write_str("B"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct NoteModifier(pub i8);

impl Display for NoteModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => Ok(()),
            1 => f.write_str("#"),
            -1 => f.write_str("b"),
            2 => f.write_str("𝄪"),
            -2 => f.write_str("𝄫"),
            // Handle other accidentals for the sake of completeness
            n if n > 2 => write!(f, "({})#", n),
            n if n < -2 => write!(f, "({})b", n),
            _ => Ok(()),
        }
    }
}

impl From<Modifier> for NoteModifier {
    fn from(value: Modifier) -> Self {
        match value {
            Modifier::Sharp => NoteModifier(1),
            Modifier::Flat => NoteModifier(-1),
            Modifier::DSharp => NoteModifier(2),
            Modifier::DFlat => NoteModifier(-2),
        }
    }
}

/// Represents a musical note, splited into its literal and its modifier if any.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Note {
    pub literal: NoteLiteral,
    pub modifier: Option<NoteModifier>,
}

impl Note {
    pub fn new(literal: NoteLiteral, modifier: Option<NoteModifier>) -> Self {
        Self { literal, modifier }
    }

    pub fn to_semitone(&self) -> u8 {
        let base_semitone = self.literal.natural_semitone();
        let offset = self.modifier.map(|m| m.0).unwrap_or(0);
        ((base_semitone as i8 + offset + 12) % 12) as u8
    }

    pub fn get_note(&self, semitones_from_root: u8, interval_degree: u8) -> Note {
        let target_literal = NoteLiteral::from_u8(self.literal as u8 + (interval_degree - 1));
        let root_pitch = self.to_semitone();
        let goal_pitch = (root_pitch + semitones_from_root) % 12;
        let natural_pitch = target_literal.natural_semitone();
        let mut offset = goal_pitch as i8 - natural_pitch as i8;
        if offset > 6 {
            offset -= 12;
        } else if offset < -6 {
            offset += 12;
        }

        Note {
            literal: target_literal,
            modifier: if offset == 0 {
                None
            } else {
                Some(NoteModifier(offset))
            },
        }
    }

    pub fn transpose(&self, note_to_transpose: &Note, target_reference: &Note) -> Note {
        let semi_diff = (target_reference.to_semitone() as i8 - self.to_semitone() as i8 + 12) % 12;
        let letter_diff = (target_reference.literal as i8 - self.literal as i8 + 7) % 7;
        note_to_transpose.get_note(semi_diff as u8, (letter_diff + 1) as u8)
    }

    pub fn to_midi_code(&self) -> u8 {
        let octave = 3;
        let base_c = 12 * (octave + 1);
        let note_offset = self.to_semitone();
        (base_c as i8 + note_offset as i8) as u8
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m = match &self.modifier {
            Some(m) => m.to_string(),
            None => "".to_owned(),
        };
        f.write_str(&format!("{}{}", self.literal, m))?;
        Ok(())
    }
}

/// Represents a note modifier. It can be sharp, flat, double sharp or double flat.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum Modifier {
    Sharp,
    Flat,
    DSharp,
    DFlat,
}

impl Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Modifier::Sharp => f.write_str("#"),
            Modifier::Flat => f.write_str("b"),
            Modifier::DSharp => f.write_str("𝄪"),
            Modifier::DFlat => f.write_str("𝄫"),
        }
    }
}
