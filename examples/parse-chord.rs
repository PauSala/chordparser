use chordparser::{midi::to_midi_file, parser::Parser};
/// Parse a chord and generate a both json-string representation and the  MIDI file.
pub fn main() {
    let mut parser = Parser::new();
    let result = parser.parse("Ab°7(Maj7,9)");
    match result {
        Ok(chord) => {
            dbg!(&chord);
            dbg!(&chord.to_json());
            to_midi_file(&chord.to_midi_codes(), "midi_files/Abmaj7#11", 120, 4);
        }
        Err(e) => {
            dbg!(e);
        }
    }
}
