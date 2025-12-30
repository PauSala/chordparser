//! Useful abstractions to work with notes
//!
use std::{fmt::Display, str::FromStr};

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

/// Any note modifier, including double, triple or n sharp/flat modifiers.
///
/// Used to handle any transposition while maintaining the correct enharmonic names.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct NoteModifier(pub i8);

impl NoteModifier {
    pub fn serialize_as_string<S>(
        modifier: &Option<NoteModifier>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match modifier {
            Some(m) => serializer.serialize_str(&m.to_string()),
            None => serializer.serialize_none(),
        }
    }
    pub fn deserialize_from_string<'de, D>(
        deserializer: D,
    ) -> Result<Option<NoteModifier>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        match s {
            Some(s_str) => {
                if s_str.is_empty() {
                    Ok(None)
                } else {
                    NoteModifier::from_str(&s_str)
                        .map(Some)
                        .map_err(serde::de::Error::custom)
                }
            }
            None => Ok(None),
        }
    }
}

impl FromStr for NoteModifier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(NoteModifier(1)),
            "b" => Ok(NoteModifier(-1)),
            "𝄪" => Ok(NoteModifier(2)),
            "𝄫" => Ok(NoteModifier(-2)),
            "" => Ok(NoteModifier(0)),
            // Handle the (n)# and (n)b cases
            _ if s.starts_with('(') => parse_extended_modifier(s),
            _ => Err(format!("Invalid modifier format: {}", s)),
        }
    }
}
/// Helper to parse formats like "(3)#" or "(-4)b"
fn parse_extended_modifier(s: &str) -> Result<NoteModifier, String> {
    let end_paren = s
        .find(')')
        .ok_or_else(|| "Missing closing parenthesis".to_string())?;

    let num_str = &s[1..end_paren];

    let n = num_str
        .parse::<i8>()
        .map_err(|_| format!("Invalid number in modifier: {}", num_str))?;

    let suffix = &s[end_paren + 1..];
    match suffix {
        "#" => Ok(NoteModifier(n)),
        "b" => Ok(NoteModifier(n)),
        _ => Err(format!("Invalid suffix after parenthesis: {}", suffix)),
    }
}

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

impl From<RootModifier> for NoteModifier {
    fn from(value: RootModifier) -> Self {
        match value {
            RootModifier::Sharp => NoteModifier(1),
            RootModifier::Flat => NoteModifier(-1),
        }
    }
}

/// Represents a musical note, splited into its literal and its modifier if any.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Note {
    pub literal: NoteLiteral,
    #[serde(
        serialize_with = "NoteModifier::serialize_as_string",
        deserialize_with = "NoteModifier::deserialize_from_string"
    )]
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

/// A simplified accidental used specifically for identifying chord roots.
///
/// Unlike [`NoteModifier`], which supports complex accidentals (double sharps,
/// double flats, or parenthesized offsets), `RootModifier` is restricted
/// to the primary accidentals found in standard chord root notation.
///
/// For musical calculations or representing intervals where double-sharps/flats
/// may occur, use [`NoteModifier`] instead.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum RootModifier {
    Sharp,
    Flat,
}

impl Display for RootModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RootModifier::Sharp => f.write_str("#"),
            RootModifier::Flat => f.write_str("b"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_serialization_format() {
        let note = Note::new(NoteLiteral::C, Some(NoteModifier(1)));
        let expected_json = json!({
            "literal": "C",
            "modifier": "#"
        });

        let actual_json = serde_json::to_value(&note).unwrap();
        assert_eq!(actual_json, expected_json);
    }

    #[test]
    fn test_round_trip() {
        let cases = [
            (NoteLiteral::A, Some(NoteModifier(-1)), "Ab"),
            (NoteLiteral::F, Some(NoteModifier(2)), "F𝄪"),
            (NoteLiteral::E, None, "E"),
            (NoteLiteral::E, Some(NoteModifier(-2)), "E𝄫"),
            (NoteLiteral::B, Some(NoteModifier(-4)), "B(-4)b"),
            (NoteLiteral::D, Some(NoteModifier(5)), "D(5)#"),
        ];

        for (lit, modif, display_str) in cases {
            let note = Note::new(lit, modif);

            assert_eq!(note.to_string(), display_str);
            let serialized = serde_json::to_string(&note).unwrap();
            let deserialized: Note = serde_json::from_str(&serialized).unwrap();
            assert_eq!(note, deserialized, "Failed to round-trip: {}", display_str);
        }
    }

    #[test]
    fn test_deserializing_custom_strings() {
        let input = r#"
        {
            "literal": "G",
            "modifier": "(3)#"
        }
        "#;

        let note: Note = serde_json::from_str(input).expect("Should parse (3)#");
        assert_eq!(note.literal, NoteLiteral::G);
        assert_eq!(note.modifier.unwrap().0, 3);
    }
}
