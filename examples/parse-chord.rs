use chordparser::parser::Parser;

pub fn main() {
    let mut parser = Parser::new();
    let result = parser.parse("Cmaj7");
    match result {
        Ok(chord) => {
            dbg!(chord);
        }
        Err(e) => {
            dbg!(e);
        }
    }
}
