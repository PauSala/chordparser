//! # [Overview](#overview)
//! ChordParser is a library for parsing musical chords from a human readable string representation.  
//! It is inspired by chordSymbol: <https://www.npmjs.com/package/chord-symbol.>  
//! Said that, it implements its own rules and conventions, which can change in the future since this it is a work in progress project.  
//! For now, the scope of the library is to parse chords from Pop, Rock, and Jazz music in _relatively standard_ English  notation.  
//! Classical notation, as well as Latin or German notation, is not supported.  
//!
//! # [Chord struct](#chord_struct)
//! Once parsed the [Chord](chord/struct.Chord.html) struct can be used to get information about the chord.  
//! This includes:
//! - Root note of the chord
//! - Bass note of the chord if any
//! - Parsed descriptor of the chord
//! - A normalized version of the input
//! - Note literals
//! - Intervals relative to root note
//! - Semitones relative to root note
//!
//! The [Chord](chord/struct.Chord.html) is also serializable into JSON, can generate MIDI codes for its notes, and allows transposition from one key to another.
//!
//!
//! # [Parser rules](#parsing_rules)
//! Since there isn't a full consensus on how chords should be written, any chord parser is by definition opinionated.  
//! We try to get a good balance between rejecting all invalid notations and accept any possible chord representation.  
//! Check the test cases in the /test folder to have a grasp of what chords can and cannot be parsed.
//!
//! # [Voicing generation](#voicing_generation)
//! The voicings module exposes a function to generate a set of MIDI notes from a [Chord](chord/struct.Chord.html) representing a voicing for it.
//! The voicing is generated in a range from C1 to G4.  The generator function accepts a lead note to generate the voicings around it, which allows chaining distinct chords smoothly.
//!
//!
//! # [Limitations](#limitations)
//! - Parsed chord notes have enharmonically correct names when possible (for example, a `B#9` chord will have `C𝄪` as the ninth instead of D), but triple flat/sharps are not suported.
//! - When transposed, slash-bass notes (like C in Ab/C) may not be enharmonically correct.
//! - The parser is not customizable for now, but it is expected to be in the future, for example:
//!     - Include or remove both custom and default validators.
//!     - Include or remove sets of allowed symbols.
//!     - Maybe allow other notations like Latin or German.
//!
//! # [Examples](#examples)
//! ```rust
//! use chordparser::parsing::Parser;
//!
//! let mut parser = Parser::new();
//! let result = parser.parse("AbMaj7#11");
//! match result {
//!     Ok(chord) => {
//!         dbg!(&chord);
//!         dbg!(&chord.to_json());
//!     }
//!     Err(e) => {
//!         dbg!(e);
//!     }
//! }
//!
//! ```

pub mod chord;
pub mod parsing;
pub mod voicings;
