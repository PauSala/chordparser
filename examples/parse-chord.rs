use chordparser::parser::Parser;
//
pub fn main() {
    let mut parser = Parser::new();
    let result = parser.parse("Abmaj7#11");
    match result {
        Ok(chord) => {
            dbg!(chord);
        }
        Err(e) => {
            dbg!(e);
        }
    }
}
