use std::{error::Error, fmt};

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
