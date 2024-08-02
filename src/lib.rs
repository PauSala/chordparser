//! # [Overview](#overview)
//! chordparser is a library for parsing musical chords from a string representation inspired by chordSymbol: <https://www.npmjs.com/package/chord-symbol.>  
//! Said that, it implements its own rules and conventions, which can change in the future since this it is a work in progress project.  
//! For now, the scope of the library is to parse chords from Pop, Rock, and Jazz music in English _relatively standard_ notation.  
//! Classical notation, as well as Latin or German notation, is not supported.  
//!
//! # [Chord struct](#chord_struct)
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
//! The [Chord](chord/struct.Chord.html) is also serializable into JSON and MIDI.  
//!
//! # [Parsing rules](#parsing_rules)
//! Since there isn't a full consensus on how chords should be written, any chord parser is by definition opinionated.  
//! The current  rules and conventions maybe are too much restrictive, but it should allways be  a way to express a chord in a way that the parser (and a human) can understand.  
//! The main spirit for now is to reject non-standard representations rather than attempting to interpret everything and to focus on parsing the most standarized forms.  
//!  
//! For example,  we decided to ban the use of `M | Ma | | MA | Maj` or any similar variant (except `Δ`) if is not followed by a 7th or a tension (9, 11, 13, etc. ) because it leads to
//! some really weird notations like `CMaj(omit5)min7`, which are far away from being standard. This could be written much more clearly as as `C-Maj7(omit5)`.  
//! As another example, the symbol `Δ` can be used to express a `maj7` chord, regardless of whether it is followed by a 7th.
//! So in this case we are more permissive since both are common interpretations.
//! As said, this is a work in progress project and the rules can change in the future.  
//!
//! # [Limitations and Todos](#limitations)
//! - Parsed chord notes have enharmonically correct names when possible. For example, a `B#9` chord will have `C𝄪` as the ninth instead of D. Said that,
//! triple flat/sharps are not suported since they add an unnecesasry complexity for very rare use cases.
//! - The parser is not customizable for now, but it is expected to be in the future, for example:
//!     - Include or remove both custom and default validators.
//!     - Include or remove sets of allowed symbols.
//!     - Maybe allow other notations like Latin or German.
//!

pub mod chord;
pub(crate) mod lexer;
pub mod midi;
pub mod parser;
pub mod parser_error;
pub(crate) mod token;
pub(crate) mod transformer;
pub(crate) mod validator;
