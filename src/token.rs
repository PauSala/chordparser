#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Note(String),
    Sharp,
    Flat,
    Aug,
    Dim,
    HalfDim,
    Extension(String),
    Add,
    Omit,
    Alt,
    Sus,
    Minor,
    Maj,
    Maj7,
    Slash,
    LParent,
    RParent,
    Comma,
    Bass,
    Illegal,
    Eof,
}
impl TokenType {
    //TODO: Add support for M (major) and m (minor). This implies not converting i to uppercase and put here every case sepparetely
    pub fn from_string(i: &str) -> Option<TokenType> {
        match i {
            "BASS" | "Bass" | "bass" => Some(TokenType::Bass),
            "MAJ" | "Maj" | "maj" | "MAJOR" | "Major" | "major" | "MA" | "Ma" | "ma" | "M" => {
                Some(TokenType::Maj)
            }
            "MIN" | "Min" | "min" | "MINOR" | "Minor" | "minor" | "MI" | "Mi" | "mi" | "m" => {
                Some(TokenType::Minor)
            }
            "SUS" | "Sus" | "sus" => Some(TokenType::Sus),
            "DIM" | "Dim" | "dim" | "diminished" => Some(TokenType::Dim),
            "ALT" | "Alt" | "alt" => Some(TokenType::Alt),
            "AUG" | "Aug" | "aug" => Some(TokenType::Aug),
            "ADD" | "Add" | "add" => Some(TokenType::Add),
            "O" | "o" | "°" => Some(TokenType::Dim),
            "OMIT" | "Omit" | "omit" | "NO" | "No" | "no" => Some(TokenType::Omit),
            "A" => Some(TokenType::Note("A".to_string())),
            "B" => Some(TokenType::Note("B".to_string())),
            "C" => Some(TokenType::Note("C".to_string())),
            "D" => Some(TokenType::Note("D".to_string())),
            "E" => Some(TokenType::Note("E".to_string())),
            "F" => Some(TokenType::Note("F".to_string())),
            "G" => Some(TokenType::Note("G".to_string())),
            _ => None,
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Note(note) => f.write_str(note)?,
            TokenType::Sharp => f.write_str("#")?,
            TokenType::Flat => f.write_str("b")?,
            TokenType::Aug => f.write_str("+")?,
            TokenType::Dim => f.write_str("°")?,
            TokenType::HalfDim => f.write_str("ø")?,
            TokenType::Extension(ext) => f.write_str(ext)?,
            TokenType::Add => f.write_str("Add")?,
            TokenType::Sus => f.write_str("Sus")?,
            TokenType::Minor => f.write_str("-")?,
            TokenType::Maj => f.write_str("△")?,
            TokenType::Maj7 => f.write_str("△")?,
            TokenType::Slash => f.write_str("/")?,
            TokenType::Alt => f.write_str("Alt")?,
            TokenType::Illegal => f.write_str("ILLEGAL")?,
            TokenType::Eof => f.write_str("EOF")?,
            TokenType::LParent => f.write_str("(")?,
            TokenType::RParent => f.write_str(")")?,
            TokenType::Omit => f.write_str("Omit")?,
            TokenType::Comma => f.write_str(",")?,
            TokenType::Bass => f.write_str("Bass")?,
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub pos: u8,
}

impl Token {
    pub fn new(token_type: TokenType, pos: u8) -> Token {
        Token { token_type, pos }
    }
}

use std::fmt::Display;

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.token_type))?;
        Ok(())
    }
}
