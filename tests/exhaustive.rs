const ALL_NOTES: [&str; 21] = [
    "Cb", "C#", "Db", "D", "D#", "Eb", "E", "E#", "Fb", "F", "F#", "Gb", "G", "G#", "Ab", "A",
    "Ab", "B", "Bb", "A#", "B#",
];

/// This test suite exhaustively covers chords that should not produce errors during parsing.  
/// The main purpose is to validate that all symbols can be parsed and to detect some corner cases derived from its combinations.
/// Tests in `should_parse.rs` contains checked results and is the place where to put both corner cases and main use cases.
#[cfg(test)]
mod tests {
    use chordparser::parsing::Parser;

    use crate::ALL_NOTES;

    #[test]
    fn test_should_parse_major() {
        let mut parser = Parser::new();
        let symbols = [
            "", "MAJ", "Maj", "maj", "MAJOR", "Major", "major", "MA", "Ma", "ma", "M", "△", "^",
        ];
        let descriptors = [
            "", "7", "79", "79#11", "79#1113", "79#11b13", "9", "#11", "13", "b13", "6", "b6",
            "69", "6/9", "/9", "sus", "sus2", "sus4", "sus#4", "+", "+5", "#5", "b5", "-5",
        ];

        for note in ALL_NOTES {
            for sym in &symbols {
                for desc in &descriptors {
                    let mut base = String::new();
                    base.push_str(note);
                    base.push_str(&sym);
                    base.push_str(&desc);
                    assert!(parser.parse(&base).is_ok(), "Failed to parse `{}`", base);
                }
            }
        }
    }

    #[test]
    fn test_should_parse_minor() {
        let mut parser = Parser::new();
        let symbols = [
            "MIN", "Min", "min", "MINOR", "Minor", "minor", "MI", "Mi", "mi", "m",
        ];
        let sevenths = ["", "△", "^", "△7", "^7", "7"];
        let descriptors = [
            "911", "91113", "9", "11", "#11", "13", "b13", "6", "b6", "69", "6/9", "/9", "sus",
            "sus2", "sus4", "sus#4", "+", "+5", "#5", "b5", "-5",
        ];

        for note in ALL_NOTES {
            for sym in &symbols {
                for seven in &sevenths {
                    for desc in &descriptors {
                        let mut base = String::new();
                        base.push_str(note);
                        base.push_str(sym);
                        base.push_str(seven);
                        base.push_str(desc);
                        assert!(parser.parse(&base).is_ok(), "Failed to parse `{}`", base);
                    }
                }
            }
        }
    }

    #[test]
    fn test_should_parse_dominant() {
        let mut parser = Parser::new();
        let symbols = ["7"];
        let descriptors = [
            "9", "9#11", "9#1113", "b9b13", "9", "#11", "13", "b13", "6", "b6", "sus", "sus2",
            "susb2", "sus#4", "sus4", "+", "+5", "#5", "b5", "-5",
        ];

        for note in ALL_NOTES {
            for sym in &symbols {
                for desc in &descriptors {
                    let mut base = String::new();
                    base.push_str(note);
                    base.push_str(&sym);
                    base.push_str(&desc);
                    assert!(parser.parse(&base).is_ok(), "Failed to parse `{}`", base);
                }
            }
        }
    }

    #[test]
    fn test_should_parse_other() {
        let mut parser = Parser::new();
        let symbols = [
            "SUS",
            "Sus",
            "sus",
            "DIM",
            "Dim",
            "dim",
            "diminished",
            "ALT",
            "Alt",
            "alt",
            "AUG",
            "Aug",
            "aug",
            "ADD9",
            "Add11",
            "add13",
            "O",
            "o",
            "°",
            "OMIT3",
            "Omit5",
            "omit3",
            "NO5",
            "No3",
            "no3",
            "Bass",
        ];

        for note in ALL_NOTES {
            for sym in &symbols {
                let mut base = String::new();
                base.push_str(note);
                base.push_str(&sym);
                assert!(parser.parse(&base).is_ok(), "Failed to parse `{}`", base);
            }
        }
    }
}
