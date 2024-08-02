//! Useful abstractions to work with notes
//!
use core::panic;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::intervals::{Interval, SemInterval};

/// All possible note literals.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
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
            _ => panic!("Unknown note literal"),
        }
    }

    pub(crate) fn to_int(l: &NoteLiteral) -> u8 {
        match l {
            NoteLiteral::C => 0,
            NoteLiteral::D => 1,
            NoteLiteral::E => 2,
            NoteLiteral::F => 3,
            NoteLiteral::G => 4,
            NoteLiteral::A => 5,
            NoteLiteral::B => 6,
        }
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
            0 => NoteMatcher(vec![
                (NoteLiteral::C, None),
                (NoteLiteral::B, Some(Modifier::Sharp)),
                (NoteLiteral::D, Some(Modifier::DFlat)),
            ]),
            1 => NoteMatcher(vec![
                (NoteLiteral::D, Some(Modifier::Flat)),
                (NoteLiteral::C, Some(Modifier::Sharp)),
                (NoteLiteral::B, Some(Modifier::DSharp)),
            ]),
            2 => NoteMatcher(vec![
                (NoteLiteral::D, None),
                (NoteLiteral::E, Some(Modifier::DFlat)),
                (NoteLiteral::C, Some(Modifier::DSharp)),
            ]),
            3 => NoteMatcher(vec![
                (NoteLiteral::E, Some(Modifier::Flat)),
                (NoteLiteral::D, Some(Modifier::Sharp)),
                (NoteLiteral::F, Some(Modifier::DFlat)),
            ]),

            4 => NoteMatcher(vec![
                (NoteLiteral::E, None),
                (NoteLiteral::F, Some(Modifier::Flat)),
                (NoteLiteral::D, Some(Modifier::DSharp)),
            ]),
            5 => NoteMatcher(vec![
                (NoteLiteral::F, None),
                (NoteLiteral::E, Some(Modifier::Sharp)),
                (NoteLiteral::G, Some(Modifier::DFlat)),
            ]),
            6 => NoteMatcher(vec![
                (NoteLiteral::G, Some(Modifier::Flat)),
                (NoteLiteral::F, Some(Modifier::Sharp)),
                (NoteLiteral::E, Some(Modifier::DSharp)),
            ]),

            7 => NoteMatcher(vec![
                (NoteLiteral::G, None),
                (NoteLiteral::F, Some(Modifier::DSharp)),
                (NoteLiteral::A, Some(Modifier::DFlat)),
            ]),
            8 => NoteMatcher(vec![
                (NoteLiteral::A, Some(Modifier::Flat)),
                (NoteLiteral::G, Some(Modifier::Sharp)),
            ]),

            9 => NoteMatcher(vec![
                (NoteLiteral::A, None),
                (NoteLiteral::G, Some(Modifier::DSharp)),
                (NoteLiteral::B, Some(Modifier::DFlat)),
            ]),
            10 => NoteMatcher(vec![
                (NoteLiteral::B, Some(Modifier::Flat)),
                (NoteLiteral::A, Some(Modifier::Sharp)),
                (NoteLiteral::C, Some(Modifier::DFlat)),
            ]),
            11 => NoteMatcher(vec![
                (NoteLiteral::B, None),
                (NoteLiteral::A, Some(Modifier::DSharp)),
                (NoteLiteral::C, Some(Modifier::Flat)),
            ]),
            _ => panic!("Dont call this with a number greater than 6"),
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
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NoteMatcher(Vec<(NoteLiteral, Option<Modifier>)>);

/// Represents a note modifier. It can be sharp, flat, double sharp or double flat.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
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
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
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
    pub fn transpose_to(&self, note: &Note, to: &Note) -> Note {
        let diff = self.get_difference(to);
        let m = self.literal.get_matcher(note.to_semitone(), diff);
        Note::new(m.0[0].clone().0, m.0[0].clone().1)
    }

    /// Returns the real interval given a semantic interval and a semitone distance from root.
    /// # Arguments
    /// * `semantic_interval` - The semantic interval, meaning an interval with no alterations (e.g. 2nd, 3rd, 4th, etc).
    /// * `interval` - The semitone distance from root
    /// # Returns
    /// The real interval
    pub fn to_real_interval(semantic_interval: u8, st: u8) -> Interval {
        match semantic_interval {
            1 => Interval::Unison,
            2 => match st {
                1 => Interval::MinorSecond,
                2 => Interval::MajorSecond,
                _ => panic!("Invalid interval"),
            },
            3 => match st {
                3 => Interval::MinorThird,
                4 => Interval::MajorThird,
                _ => panic!("Invalid interval"),
            },
            4 => match st {
                5 => Interval::PerfectFourth,
                6 => Interval::AugmentedFourth,
                _ => panic!("Invalid interval"),
            },
            5 => match st {
                6 => Interval::DiminishedFifth,
                7 => Interval::PerfectFifth,
                8 => Interval::AugmentedFifth,
                _ => panic!("Invalid interval"),
            },
            6 => match st {
                8 => Interval::MinorSixth,
                9 => Interval::MajorSixth,
                _ => panic!("Invalid interval"),
            },
            7 => match st {
                9 => Interval::DiminishedSeventh,
                10 => Interval::MinorSeventh,
                11 => Interval::MajorSeventh,
                _ => panic!("Invalid interval"),
            },
            9 => match st {
                13 => Interval::FlatNinth,
                14 => Interval::Ninth,
                15 => Interval::SharpNinth,
                _ => panic!("Invalid interval"),
            },

            11 => match st {
                17 => Interval::Eleventh,
                18 => Interval::SharpEleventh,
                _ => panic!("Invalid interval"),
            },

            13 => match st {
                20 => Interval::FlatThirteenth,
                21 => Interval::Thirteenth,
                _ => panic!("Invalid interval"),
            },
            _ => panic!("Invalid interval"),
        }
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
                    _ => panic!("Double flat/sharp roots are not allowed"),
                },
                None => 0,
            },
            NoteLiteral::D => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 3,
                    Modifier::Flat => 1,
                    _ => panic!("Double flat/sharp roots are not allowed"),
                },
                None => 2,
            },
            NoteLiteral::E => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 5,
                    Modifier::Flat => 3,
                    _ => panic!("Double flat/sharp roots are not allowed"),
                },
                None => 4,
            },
            NoteLiteral::F => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 6,
                    Modifier::Flat => 4,
                    _ => panic!("Double flat/sharp roots are not allowed"),
                },
                None => 5,
            },
            NoteLiteral::G => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 8,
                    Modifier::Flat => 6,
                    _ => panic!("Double flat/sharp roots are not allowed"),
                },
                None => 7,
            },
            NoteLiteral::A => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 10,
                    Modifier::Flat => 8,
                    _ => panic!("Double flat/sharp roots are not allowed"),
                },
                None => 9,
            },
            NoteLiteral::B => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 0,
                    Modifier::Flat => 10,
                    _ => panic!("Double flat/sharp roots are not allowed"),
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
        let root_index = NoteLiteral::to_int(&self.literal);
        let interval_index = (root_index + (semantic_interval - 1)) % 7;
        let mut f =
            m.0.iter()
                .find(|m| NoteLiteral::to_int(&m.0) == interval_index);
        f = {
            // If the note is triple flat/sharp return the first note, it is wrong anyway
            // Maybe in the future we can try to get the most reasonable note and return some kind of warning attached to the chord
            if f.is_none() {
                Some(&m.0[0])
            } else {
                f
            }
        };
        let (literal, modifier) = f.unwrap().to_owned();
        Note::new(literal, modifier)
    }

    /// Returns the MIDI code of the note.
    /// # Returns
    /// The MIDI code of the note centered around central C
    pub fn to_midi_code(&self) -> u8 {
        let central_c = 60;
        let mut code = central_c - 12;
        match &self.modifier {
            Some(m) => match m {
                Modifier::Sharp => code += 1,
                Modifier::Flat => code -= 1,
                Modifier::DSharp => code += 2,
                Modifier::DFlat => code -= 2,
            },
            None => (),
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

/// Intermediate representation of a note used by the parser.
/// It contains the semitone distance from root, the semantic interval and the position in the string input.
/// # Fields
/// * `semitone` - The semitone distance from root
/// * `sem_interval` - The semantic interval with no alterations (e.g. 2nd, 3rd, 4th, etc)
/// * `pos` - The position in the string input
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct NoteDescriptor {
    pub sem_interval: SemInterval,
    pub semitone: u8,
    pub pos: usize,
}

impl NoteDescriptor {
    pub(crate) fn new(sem_interval: SemInterval, semitone: u8, pos: usize) -> NoteDescriptor {
        NoteDescriptor {
            sem_interval,
            semitone,
            pos,
        }
    }
}
