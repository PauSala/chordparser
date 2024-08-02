//! # Struct returned when parsing a chord fails
use std::{error::Error, fmt};

/// Errors that can occur when parsing a chord.  
/// Includes a list of string messages with a reason an the position in the input string when possible.
#[derive(Debug)]
pub struct ParserErrors {
    pub errors: Vec<String>,
}

impl ParserErrors {
    pub fn new(messages: Vec<String>) -> ParserErrors {
        ParserErrors { errors: messages }
    }
}

impl fmt::Display for ParserErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Errors: {:?}", self.errors)
    }
}

impl Error for ParserErrors {}
