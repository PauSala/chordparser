//! Useful abstractions to work with notes
//!
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// All possible note literals.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum NoteLiteral {
    C = 0,
    D = 1,
    E = 2,
    F = 3,
    G = 4,
    A = 5,
    B = 6,
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

    pub(crate) fn numeric(&self) -> u8 {
        *self as u8
    }

    /// Returns the matcher for given root and interval.  
    /// It is used to find the enharmonically correct note given an interval and a root
    /// # Arguments
    /// * `root` - The root note
    /// * `interval` - The interval
    /// # Returns
    /// A NoteMatcher that contains all possible notes at a distance of interval from root.
    pub(crate) fn get_matcher(&self, root: u8, interval: u8) -> NoteMatcher {
        let interval = (root + interval) % 12;
        let i = interval % 12;
        match i {
            0 => vec![
                (NoteLiteral::C, None),
                (NoteLiteral::B, Some(Modifier::Sharp)),
                (NoteLiteral::D, Some(Modifier::DFlat)),
            ],
            1 => vec![
                (NoteLiteral::D, Some(Modifier::Flat)),
                (NoteLiteral::C, Some(Modifier::Sharp)),
                (NoteLiteral::B, Some(Modifier::DSharp)),
            ],
            2 => vec![
                (NoteLiteral::D, None),
                (NoteLiteral::E, Some(Modifier::DFlat)),
                (NoteLiteral::C, Some(Modifier::DSharp)),
            ],
            3 => vec![
                (NoteLiteral::E, Some(Modifier::Flat)),
                (NoteLiteral::D, Some(Modifier::Sharp)),
                (NoteLiteral::F, Some(Modifier::DFlat)),
            ],

            4 => vec![
                (NoteLiteral::E, None),
                (NoteLiteral::F, Some(Modifier::Flat)),
                (NoteLiteral::D, Some(Modifier::DSharp)),
            ],
            5 => vec![
                (NoteLiteral::F, None),
                (NoteLiteral::E, Some(Modifier::Sharp)),
                (NoteLiteral::G, Some(Modifier::DFlat)),
            ],
            6 => vec![
                (NoteLiteral::G, Some(Modifier::Flat)),
                (NoteLiteral::F, Some(Modifier::Sharp)),
                (NoteLiteral::E, Some(Modifier::DSharp)),
            ],

            7 => vec![
                (NoteLiteral::G, None),
                (NoteLiteral::F, Some(Modifier::DSharp)),
                (NoteLiteral::A, Some(Modifier::DFlat)),
            ],
            8 => vec![
                (NoteLiteral::A, Some(Modifier::Flat)),
                (NoteLiteral::G, Some(Modifier::Sharp)),
            ],

            9 => vec![
                (NoteLiteral::A, None),
                (NoteLiteral::G, Some(Modifier::DSharp)),
                (NoteLiteral::B, Some(Modifier::DFlat)),
            ],
            10 => vec![
                (NoteLiteral::B, Some(Modifier::Flat)),
                (NoteLiteral::A, Some(Modifier::Sharp)),
                (NoteLiteral::C, Some(Modifier::DFlat)),
            ],
            11 => vec![
                (NoteLiteral::B, None),
                (NoteLiteral::A, Some(Modifier::DSharp)),
                (NoteLiteral::C, Some(Modifier::Flat)),
            ],
            _ => vec![],
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

/// All possible matches for a given semitone from a root note.
pub type NoteMatcher = Vec<(NoteLiteral, Option<Modifier>)>;

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

/// Represents a musical note, splited into its literal and its modifier if any.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Note {
    pub literal: NoteLiteral,
    pub modifier: Option<Modifier>,
}

impl Note {
    pub fn new(literal: NoteLiteral, modifier: Option<Modifier>) -> Note {
        Note { literal, modifier }
    }

    fn get_difference(&self, to: &Note) -> u8 {
        let o = self.to_semitone();
        let n = to.to_semitone();
        (n + 12 - o) % 12
    }

    /// Transpose a note to another note taking as refference the distance between `self` and `to`.  
    /// For example:
    /// If `self` is C, and `to` is E, `note` D will be transposed to F#.  
    /// # Arguments
    /// * `note` - The note to transpose
    /// * `to` - The note took as refference to calculate the transposing interval
    /// # Returns
    /// The transposed note
    pub fn transpose(&self, note: &Note, to: &Note) -> Note {
        let diff = self.get_difference(to);
        let m = self.literal.get_matcher(note.to_semitone(), diff);
        Note::new(m[0].0, m[0].1)
    }

    /// Returns the semitone distance taking C as reference.
    /// # Returns
    /// The semitone distance from C
    pub fn to_semitone(&self) -> u8 {
        match self.literal {
            NoteLiteral::C => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 1,
                    Modifier::Flat => 11,
                    _ => 0,
                },
                None => 0,
            },
            NoteLiteral::D => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 3,
                    Modifier::Flat => 1,
                    _ => 0,
                },
                None => 2,
            },
            NoteLiteral::E => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 5,
                    Modifier::Flat => 3,
                    _ => 0,
                },
                None => 4,
            },
            NoteLiteral::F => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 6,
                    Modifier::Flat => 4,
                    _ => 0,
                },
                None => 5,
            },
            NoteLiteral::G => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 8,
                    Modifier::Flat => 6,
                    _ => 0,
                },
                None => 7,
            },
            NoteLiteral::A => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 10,
                    Modifier::Flat => 8,
                    _ => 0,
                },
                None => 9,
            },
            NoteLiteral::B => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 0,
                    Modifier::Flat => 10,
                    _ => 0,
                },
                None => 11,
            },
        }
    }

    /// Given a semitone distance from root and a semantic interval, returns the enharmonically correct note.
    /// # Arguments
    /// * `semitone` - The semitone distance from root
    /// * `semantic_interval` - The semantic interval
    /// # Returns
    /// The enharmonically correct note relative to root
    pub fn get_note(&self, semitone: u8, semantic_interval: u8) -> Note {
        let m = self.literal.get_matcher(self.to_semitone(), semitone);
        let root_index = &self.literal.numeric();
        let interval_index = (root_index + (semantic_interval - 1)) % 7;
        let f = m.iter().find(|m| m.0.numeric() == interval_index);

        // If the note is triple flat/sharp return the first note, it is wrong anyway
        // Maybe in the future we can try to get the most reasonable note and return some kind of warning attached to the chord
        let (literal, modifier) = f.unwrap_or(&m[0]).to_owned();
        Note::new(literal, modifier)
    }

    /// Returns the MIDI code of the note.
    /// # Returns
    /// The MIDI code of the note centered around central C
    pub fn to_midi_code(&self) -> u8 {
        let central_c = 60;
        let mut code = central_c - 12;
        if let Some(m) = &self.modifier {
            match m {
                Modifier::Sharp => code += 1,
                Modifier::Flat => code -= 1,
                Modifier::DSharp => code += 2,
                Modifier::DFlat => code -= 2,
            }
        }
        match self.literal {
            NoteLiteral::C => (),
            NoteLiteral::D => code += 2,
            NoteLiteral::E => code += 4,
            NoteLiteral::F => code += 5,
            NoteLiteral::G => code += 7,
            NoteLiteral::A => code += 9,
            NoteLiteral::B => code += 11,
        }
        code
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

#[cfg(test)]
mod test {
    use crate::chord::intervals::IntDegree;

    use super::*;

    #[test]
    fn enharmonies() {
        let cases = vec![
            (
                Note::new(NoteLiteral::C, None),
                3,
                IntDegree::Third,
                Note {
                    literal: NoteLiteral::E,
                    modifier: Some(Modifier::Flat),
                },
            ),
            (
                Note::new(NoteLiteral::C, Some(Modifier::Flat)),
                3,
                IntDegree::Third,
                Note {
                    literal: NoteLiteral::E,
                    modifier: Some(Modifier::DFlat),
                },
            ),
            (
                Note::new(NoteLiteral::D, Some(Modifier::Flat)),
                1,
                IntDegree::Ninth,
                Note {
                    literal: NoteLiteral::E,
                    modifier: Some(Modifier::DFlat),
                },
            ),
            (
                Note::new(NoteLiteral::D, Some(Modifier::Sharp)),
                15,
                IntDegree::Ninth,
                Note {
                    literal: NoteLiteral::E,
                    modifier: Some(Modifier::DSharp),
                },
            ),
            (
                Note::new(NoteLiteral::B, None),
                9,
                IntDegree::Seventh,
                Note {
                    literal: NoteLiteral::A,
                    modifier: Some(Modifier::Flat),
                },
            ),
        ];
        for (note, dist, sem_interval, expect) in cases {
            assert_eq!(expect, note.get_note(dist, sem_interval.numeric()))
        }
    }
}
