//! # Error handling for the parser module.
use std::{
    error::Error,
    fmt::{self},
};

use serde::Serialize;

/// Errors that can occur when parsing a chord.
/// Includes a list of string messages with a reason an the position in the input string when possible.
/// The position is 1-based.
/// The error messages are meant to be user-friendly.
#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
pub enum ParserError {
    IllegalToken(usize),
    UnexpectedNote(usize),
    DuplicateModifier(String),
    InconsistentExtension(String),
    DuplicateExtension(usize),
    InvalidExtension(usize),
    WrongExpressionTarget(usize),
    UnexpectedModifier(usize),
    MissingRootNote,
    ThreeConsecutiveSemitones(Vec<String>),
    MissingAddTarget((usize, usize)),
    IllegalOrMissingOmitTarget((usize, usize)),
    IllegalAddTarget((usize, usize)),
    IllegalSlashNotation(usize),
    UnexpectedClosingParenthesis(usize),
    MissingClosingParenthesis(usize),
    NestedParenthesis(usize),
}

impl ParserError {
    fn surround_element_at_index(&self, s: &str, index: usize) -> String {
        let index = index - 1;
        if index >= s.len() {
            let mut res = s.to_string();
            res.push_str("{}");
            return res;
        }

        let before = &s[..index];
        let element = s.chars().nth(index).unwrap();
        let after = &s[index + element.len_utf8()..];

        format!("{}{{{}}}{}", before, element, after)
    }

    fn surround_element_at_index_with_span(&self, s: &str, index: usize, len: usize) -> String {
        let index = index - 1 + len;
        if index >= s.len() {
            let mut res = s.to_string();
            res.push_str("{}");
            return res;
        }

        let before = &s[..index];
        let element = s.chars().nth(index).unwrap();
        let after = &s[index + element.len_utf8()..];

        format!("{}{{{}}}{}", before, element, after)
    }

    /// Returns the position in the input string where the error occurred.
    /// If the error is not related to a specific position or is not possible to know the position, returns None.
    /// The position is 1-based.
    ///
    pub fn error_position(&self) -> Option<usize> {
        match self {
            ParserError::ThreeConsecutiveSemitones(_)
            | ParserError::DuplicateModifier(_)
            | ParserError::InconsistentExtension(_) => None,
            ParserError::IllegalToken(pos) | ParserError::UnexpectedNote(pos) => Some(*pos),
            ParserError::DuplicateExtension(pos) | ParserError::InvalidExtension(pos) => Some(*pos),
            ParserError::UnexpectedModifier(pos) | ParserError::IllegalSlashNotation(pos) => {
                Some(*pos)
            }
            ParserError::UnexpectedClosingParenthesis(pos)
            | ParserError::NestedParenthesis(pos)
            | ParserError::WrongExpressionTarget(pos)
            | ParserError::MissingClosingParenthesis(pos) => Some(*pos),
            ParserError::MissingRootNote => Some(1),
            ParserError::IllegalAddTarget((pos, len))
            | ParserError::IllegalOrMissingOmitTarget((pos, len))
            | ParserError::MissingAddTarget((pos, len)) => Some(*pos + *len),
        }
    }

    /// Returns a verbose display of the error, including the element at the position where the error occurred.
    /// The position is 1-based.
    pub fn verbose_display(&self, origin: &str) -> String {
        match self {
            ParserError::IllegalToken(pos)
            | ParserError::UnexpectedNote(pos)
            | ParserError::DuplicateExtension(pos)
            | ParserError::InvalidExtension(pos)
            | ParserError::UnexpectedModifier(pos)
            | ParserError::IllegalSlashNotation(pos)
            | ParserError::UnexpectedClosingParenthesis(pos)
            | ParserError::MissingClosingParenthesis(pos)
            | ParserError::WrongExpressionTarget(pos)
            | ParserError::NestedParenthesis(pos) => {
                let mut res = format!("{} → ", self);
                res.push_str(&self.surround_element_at_index(origin, *pos));
                res
            }
            ParserError::DuplicateModifier(_)
            | ParserError::InconsistentExtension(_)
            | ParserError::MissingRootNote
            | ParserError::ThreeConsecutiveSemitones(_) => {
                format!("{}", self)
            }
            ParserError::MissingAddTarget((pos, len))
            | ParserError::IllegalOrMissingOmitTarget((pos, len))
            | ParserError::IllegalAddTarget((pos, len)) => {
                let mut res = format!("{} → ", self);
                res.push_str(&self.surround_element_at_index_with_span(origin, *pos, *len));
                res
            }
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::IllegalToken(pos) => write!(f, "Illegal token at position {}", pos),
            ParserError::UnexpectedNote(pos) => write!(f, "Unexpected note at position {}", pos),
            ParserError::DuplicateModifier(modifier) => {
                write!(f, "Duplicate modifier: {}", modifier)
            }
            ParserError::InconsistentExtension(extension) => {
                write!(f, "Inconsistent extension: {}", extension)
            }
            ParserError::DuplicateExtension(pos) => {
                write!(f, "Duplicate extension at position {}", pos)
            }
            ParserError::InvalidExtension(pos) => {
                write!(f, "Invalid extension at position {}", pos)
            }
            ParserError::WrongExpressionTarget(expr) => {
                write!(f, "Invalid expression target at position: {}", expr)
            }
            ParserError::UnexpectedModifier(pos) => {
                write!(f, "Unexpected modifier at position {}", pos)
            }
            ParserError::MissingRootNote => write!(f, "Missing root note"),
            ParserError::ThreeConsecutiveSemitones(notes) => {
                write!(f, "Three consecutive semitones: {:?}", notes)
            }
            ParserError::MissingAddTarget((pos, len)) => {
                write!(f, "Missing add target at position {}", pos + len)
            }
            ParserError::IllegalOrMissingOmitTarget((pos, len)) => {
                write!(
                    f,
                    "Illegal or missing omit target at position {}",
                    pos + len
                )
            }
            ParserError::IllegalAddTarget((pos, len)) => {
                write!(f, "Illegal add target at position {}", pos + len)
            }
            ParserError::IllegalSlashNotation(pos) => {
                write!(f, "Illegal slash notation at position {}", pos)
            }
            ParserError::UnexpectedClosingParenthesis(pos) => {
                write!(f, "Unexpected closing parenthesis at position {}", pos)
            }
            ParserError::MissingClosingParenthesis(pos) => {
                write!(f, "Missing closing parenthesis at position {}", pos)
            }
            ParserError::NestedParenthesis(pos) => {
                write!(f, "Nested parenthesis at position {}", pos)
            }
        }
    }
}

/// Error returned when multiple errors occur during parsing.
/// Contains a list of ParserError.
#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
pub struct ParserErrors {
    pub errors: Vec<ParserError>,
}

impl ParserErrors {
    pub fn new(messages: Vec<ParserError>) -> ParserErrors {
        ParserErrors { errors: messages }
    }
}

impl fmt::Display for ParserErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Errors: {:?}", self.errors)
    }
}

impl Error for ParserErrors {}
