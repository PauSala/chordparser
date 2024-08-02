//! # [Overview](#overview)
//! chordparser is a library for parsing musical chords from a string representation.
//! Once parsed the [Chord](chord/struct.Chord.html) struct can be used to get information about the chord.  
//! This includes:
//! - Root note of the chord
//! - Bass note of the chord if any
//! - Parsed descriptor of the chord
//! - Note literals in the chord
//! - Intervals relative to root note
//! - Semitones relative to root note
//! - Whether the chord is a sus chord
//!
//! The [Chord](chord/struct.Chord.html) is also serializable into JSON and a MIDI file.  
//!
//! # [Parsing rules](#parsing_rules)
pub mod chord;
pub(crate) mod lexer;
pub mod midi;
pub mod parser;
pub mod parser_error;
pub(crate) mod token;
pub(crate) mod transformer;
pub(crate) mod validator;
