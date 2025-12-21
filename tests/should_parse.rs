use chordparser::{
    chord::note::{Modifier, Note, NoteLiteral},
    parsing::Parser,
};

use test_case::test_case;

/// This test suite covers both popular chords and some  really weird ones, as well as edge cases found during development.

#[test_case("C5", vec!["C", "G"])]
#[test_case("C(omit3)", vec!["C", "G"])]
#[test_case("Csus", vec!["C", "F", "G"])]
#[test_case("C(b5)", vec!["C", "E", "Gb"])]
#[test_case("C", vec!["C", "E", "G"])]
#[test_case("C^", vec!["C", "E", "G", "B"]; "Cmaj7(C^)")]
#[test_case("C△", vec!["C", "E", "G", "B"]; "CMaj7")]
#[test_case("Cmajor7", vec!["C", "E", "G", "B"]; "Cmajor7")]
#[test_case("C△7", vec!["C", "E", "G", "B"]; "CMaj7 II")]
#[test_case("C##5", vec!["C#", "E#", "G𝄪"])]
#[test_case("C+", vec!["C", "E", "G#"]; "C+")]
#[test_case("C+-5", vec!["C", "E", "Gb", "G#"]; "C+-5")]
#[test_case("C(-5#5)", vec!["C", "E", "Gb", "G#"]; "C(-5#5)")]
#[test_case("C+#11", vec!["C", "E", "G#", "F#"]; "C+#11")]
#[test_case("C+Maj7", vec!["C", "E", "G#", "B"]; "C+Maj7")]
#[test_case("C+Maj76omit5", vec!["C", "E", "A", "B"]; "C+Maj76")]
#[test_case("C+Maj9", vec!["C", "E", "G#", "B", "D"]; "C+Maj9")]
#[test_case("C6(b5)", vec!["C", "E", "Gb", "A"])]
#[test_case("C6", vec!["C", "E", "G", "A"])]
#[test_case("C6(#5)", vec!["C", "E", "G#", "A"])]
#[test_case("C69", vec!["C", "E", "G", "A", "D"])]
#[test_case("C69(#11)", vec!["C", "E", "G", "A", "D", "F#"])]
#[test_case("Cma7(b5)", vec!["C", "E", "Gb", "B"])]
#[test_case("CMAJ713", vec!["C", "E", "G", "B", "D", "A"])]
#[test_case("CM7", vec!["C", "E", "G", "B"])]
#[test_case("Cma7", vec!["C", "E", "G", "B"])]
#[test_case("Cma7(#5)", vec!["C", "E", "G#", "B"])]
#[test_case("Cadd9(omit3)", vec!["C", "G", "D"])]
#[test_case("Cadd9(omit3,5)", vec!["C", "D"])]
#[test_case("Cadd9(no3)", vec!["C", "G", "D"])]
#[test_case("Cadd9", vec!["C", "E", "G", "D"])]
#[test_case("C(add9)", vec!["C", "E", "G", "D"])]
#[test_case("Cma9", vec!["C", "E", "G", "B", "D"])]
#[test_case("Cma9(no3)", vec!["C", "G", "B", "D"])]
#[test_case("Cma9(no3,5)", vec!["C", "B", "D"])]
#[test_case("Cma9no3", vec!["C", "G", "B", "D"])]
#[test_case("Cma9(#11)", vec!["C", "E", "G", "B", "D", "F#"])]
#[test_case("Cma9(omit3)", vec!["C", "G", "B", "D"])]
#[test_case("Cma13", vec!["C", "E", "G", "B", "D", "A"])]
#[test_case("Cma13(#11)", vec!["C", "E", "G", "B", "D", "F#", "A"])]
#[test_case("C°", vec!["C", "Eb", "Gb"]; "C° is dim")]
#[test_case("Cmi", vec!["C", "Eb", "G"])]
#[test_case("Cmib6", vec!["C", "Eb", "G", "Ab"])]
#[test_case("Cmi add9", vec!["C", "Eb", "G", "D"]; "Cmi add9 is minor")]
#[test_case("Cmi(add9)", vec!["C", "Eb", "G", "D"])]
#[test_case("Cmiadd9", vec!["C", "Eb", "G", "D"])]
#[test_case("Cmi6", vec!["C", "Eb", "G", "A"])]
#[test_case("Cmi69", vec!["C", "Eb", "G", "A", "D"])]
#[test_case("Cmi6/9", vec!["C", "Eb", "G", "A", "D"])]
#[test_case("C-6", vec!["C", "Eb", "G", "A"])]
#[test_case("C--5", vec!["C", "Eb", "Gb"])]
#[test_case("C--56/9", vec!["C", "Eb", "Gb", "A", "D"])]
#[test_case("C-69", vec!["C", "Eb", "G", "A", "D"])]
#[test_case("Cminor69", vec!["C", "Eb", "G", "A", "D"])]
#[test_case("Cminor611", vec!["C", "Eb", "G", "A", "D", "F"])]
#[test_case("Cminor613", vec!["C", "Eb", "G", "A", "D", "F", "A"])]
#[test_case("C-6/9", vec!["C", "Eb", "G", "A", "D"])]
#[test_case("Cmi69add11", vec!["C", "Eb", "G", "A", "D", "F"])]
#[test_case("Cmi(#5)", vec!["C", "Eb", "G#"])]
#[test_case("Cmi7", vec!["C", "Eb", "G", "Bb"])]
#[test_case("Cmi7(b5)", vec!["C", "Eb", "Gb", "Bb"])]
#[test_case("Cmi7(#5)", vec!["C", "Eb", "G#", "Bb"])]
#[test_case("Cmi7(b5,add11)", vec!["C", "Eb", "Gb", "Bb", "F"])]
#[test_case("Cmi7(add11)", vec!["C", "Eb", "G", "Bb", "F"])]
#[test_case("Cmi9", vec!["C", "Eb", "G", "Bb", "D"])]
#[test_case("Cmi9(b5)", vec!["C", "Eb", "Gb", "Bb", "D"])]
#[test_case("Cmi11", vec!["C", "Eb", "G", "Bb", "D", "F"])]
#[test_case("Cmi11(b5,no3)", vec!["C", "Gb", "Bb", "D", "F"])]
#[test_case("Cmi7(b5,#5)", vec!["C", "Eb", "Gb", "G#", "Bb"])]
#[test_case("Cmi11(b5,#5)", vec!["C", "Eb", "Gb", "G#", "Bb", "D", "F"])]
#[test_case("Cmi11(b5,b13)", vec!["C", "Eb", "Gb", "Bb", "D", "F", "Ab"])]
#[test_case("Cmi13", vec!["C", "Eb", "G", "Bb", "D", "F", "A"])]
#[test_case("Cminomit5maj7", vec!["C", "Eb", "B"])]
#[test_case("Cminomit5", vec!["C", "Eb"])]
#[test_case("C#minomit5maj7add9#11", vec!["C#", "E", "B#", "D#", "F𝄪"])]
#[test_case("Csus4(b5#5)", vec!["C", "F", "Gb", "G#"])]
#[test_case("C7", vec!["C", "E", "G", "Bb" ])]
#[test_case("CMaj", vec!["C", "E", "G" ])]
#[test_case("Cadd2", vec!["C", "D", "E", "G"])]
#[test_case("CMaj13#9#11#5", vec!["C", "E", "G#", "B", "D#", "F#", "A" ])]
#[test_case("CMaj713#9#11#5", vec!["C", "E", "G#", "B", "D#", "F#", "A" ])]
#[test_case("CMaj713#9#11", vec!["C", "E", "G", "B", "D#", "F#", "A" ])]
#[test_case("CM713#9#11#5", vec!["C", "E", "G#", "B", "D#", "F#", "A" ])]
#[test_case("C△13#9#11#5", vec!["C", "E", "G#", "B", "D#", "F#", "A" ])]
#[test_case("Calt", vec!["C", "E", "Bb", "Db", "D#", "F#", "Ab"])]
#[test_case("C7(b5,#5,b9)", vec!["C", "E", "Gb", "G#", "Bb", "Db"])]
#[test_case("C7sus", vec!["C", "F", "G", "Bb"])]
#[test_case("C7sus2", vec!["C", "G", "Bb", "D"])]
#[test_case("C7(b5,#5,#9)", vec!["C", "E", "Gb", "G#", "Bb", "D#"])]
#[test_case("C7(b5,#5,b9,#9)", vec!["C", "E", "Gb", "G#", "Bb", "Db", "D#"])]
#[test_case("C7b5#5b9#9b13", vec!["C", "E", "Gb", "G#", "Bb", "Db", "D#", "Ab"])]
#[test_case("C7(b13)", vec!["C", "E", "Bb", "Ab"])]
#[test_case("C9", vec!["C", "E", "G", "Bb", "D"])]
#[test_case("C9(13)", vec!["C", "E", "G", "Bb", "D", "A"])]
#[test_case("C9(add13)", vec!["C", "E", "G", "Bb", "D", "A"])]
#[test_case("C9sus", vec!["C", "F", "G", "Bb", "D"])]
#[test_case("C9(b5)", vec!["C", "E", "Gb", "Bb", "D"])]
#[test_case("C9(b5,#5)", vec!["C", "E", "Gb", "G#", "Bb", "D"])]
#[test_case("C9(b5,b13)", vec!["C", "E", "Gb", "Bb", "D", "Ab"])]
#[test_case("C9(#5,#11)", vec!["C", "E", "G#", "Bb", "D", "F#"])]
#[test_case("C9#11", vec!["C", "E", "G", "Bb", "D", "F#"])]
#[test_case("C11", vec!["C", "G", "Bb", "D", "F"])]
#[test_case("C11(b9)", vec!["C", "G", "Bb", "Db", "F"])]
#[test_case("C13", vec!["C", "E", "G", "Bb", "D", "A"])]
#[test_case("C(b13)", vec!["C", "E", "Ab"])]
#[test_case("C7b13", vec!["C", "E", "Bb", "Ab"])]
#[test_case("C13sus", vec!["C", "F", "G", "Bb", "D", "A"])]
#[test_case("C13(b5)", vec!["C", "E", "Gb", "Bb", "D", "A"])]
#[test_case("C13(b5,b9)", vec!["C", "E", "Gb", "Bb", "Db", "A"])]
#[test_case("C13(b5,#9)", vec!["C", "E", "Gb", "Bb", "D#", "A"])]
#[test_case("C13(b5,b9,#9)", vec!["C", "E", "Gb", "Bb", "Db", "D#", "A"])]
#[test_case("C13b9", vec!["C", "E", "G", "Bb", "Db", "A"])]
#[test_case("C13 b9 #9", vec!["C", "E", "G", "Bb", "Db", "D#", "A"])]
#[test_case("C13 b9 #11", vec!["C", "E", "G", "Bb", "Db", "F#", "A"])]
#[test_case("C13(b9,#9,#11)", vec!["C", "E", "G", "Bb", "Db", "D#", "F#", "A"])]
#[test_case("C13(#9)", vec!["C", "E", "G", "Bb", "D#", "A"])]
#[test_case("C13(#9#11)", vec!["C", "E", "G", "Bb", "D#", "F#", "A"])]
#[test_case("C13(#11)", vec!["C", "E", "G", "Bb", "D", "F#", "A"])]
#[test_case("Cdim", vec!["C", "Eb", "Gb"])]
#[test_case("Cdim7", vec!["C", "Eb", "Gb", "B𝄫"])]
#[test_case("Cdim7Maj7b13/Ab", vec!["C", "Eb", "Gb", "B𝄫", "B", "Ab"])]
#[test_case("Cdim7(add ma7)", vec!["C", "Eb", "Gb", "B𝄫", "B"])]
#[test_case("Cdim7(add Maj7, 9)", vec!["C", "Eb", "Gb", "B𝄫", "B", "D"])]
#[test_case("Cdim7(add Maj7, 9, 11)", vec!["C", "Eb", "Gb", "B𝄫", "B", "D", "F"])]
#[test_case("Cdim7addM711b13", vec!["C", "Eb", "Gb", "B𝄫", "B", "D", "F", "Ab"])]
#[test_case("Cdim7ma11", vec!["C", "Eb", "Gb", "B𝄫", "B", "D", "F"])]
#[test_case("Cdim7add ma7 b13", vec!["C", "Eb", "Gb", "B𝄫", "B", "Ab"])]
#[test_case("Cdim7addMaj7 b13", vec!["C", "Eb", "Gb", "B𝄫", "B", "Ab"])]
#[test_case("Cdim7add9", vec!["C", "Eb", "Gb", "B𝄫", "D"])]
#[test_case("Cdim7add911", vec!["C", "Eb", "Gb", "B𝄫", "D", "F"])]
#[test_case("Cøadd911", vec!["C", "Eb", "Gb", "Bb", "D", "F"])]
#[test_case("Cø11", vec!["C", "Eb", "Gb", "Bb", "D", "F"])]
#[test_case("Cø13", vec!["C", "Eb", "Gb", "Bb", "D", "F", "A"])]
#[test_case("Cdim7add911 b13", vec!["C", "Eb", "Gb", "B𝄫", "D", "F", "Ab"])]
#[test_case("Cdim7(add9,13)", vec!["C", "Eb", "Gb", "B𝄫", "D", "A"])]
#[test_case("Cdim7(add9,add13)", vec!["C", "Eb", "Gb", "B𝄫", "D", "A"])]
#[test_case("Cdim7(add9,b13)", vec!["C", "Eb", "Gb", "B𝄫", "D", "Ab"])]
#[test_case("Cdim7(add11,13)", vec!["C", "Eb", "Gb", "B𝄫", "F", "A"])]
#[test_case("Cdim7(add13)", vec!["C", "Eb", "Gb", "B𝄫","A"])]
#[test_case("C-7(add6)", vec!["C", "Eb", "G", "A","Bb"])]
#[test_case("Cminor7(add6)", vec!["C", "Eb", "G", "A","Bb"])]
#[test_case("Csus4(add9,3)", vec!["C", "E", "F", "G", "D"])]
#[test_case("C(add2,13)", vec!["C", "D", "E", "G", "A"])]
#[test_case("C(add2 13)", vec!["C", "D", "E", "G", "Bb", "D", "A"])]
#[test_case("C7(omit5,3 add9,13)", vec!["C", "Bb", "D", "A"])]
#[test_case("C7omit5,9", vec!["C", "E", "Bb", "D"])]
#[test_case("Cdimmaj7", vec!["C", "Eb", "Gb", "B"])]
#[test_case("CBass", vec!["C"])]
#[test_case("C7/9", vec!["C", "E", "G", "Bb", "D"])]
#[test_case("Cdim7/9", vec!["C", "Eb", "Gb", "B𝄫", "D"])]
#[test_case("C(add 9,omit 3)", vec!["C", "G", "D"])]
#[test_case("Fb(add 9,add b13)", vec!["Fb", "Ab", "Gb", "D𝄫"])]
#[test_case("Eb+(add b9,add #9)", vec!["Eb", "G", "B", "Fb", "F#"])]
#[test_case("C°7", vec!["C", "Eb", "Gb", "B𝄫"])]
#[test_case("C°7(add MA7)", vec!["C", "Eb", "Gb", "B𝄫", "B"])]
#[test_case("Csus(b9)", vec!["C", "F", "G", "Db"])]
#[test_case("Csus(b5)", vec!["C", "F", "Gb"])]
#[test_case("CMa", vec!["C", "E", "G"])]
#[test_case("F△7", vec!["F", "A", "C", "E"])]
#[test_case("C4", vec!["C", "F", "G"])]
#[test_case("C49", vec!["C", "F", "G", "Bb", "D"])]
#[test_case("C-5", vec!["C", "E", "Gb"])]
#[test_case("Cdim7maj711b13",  vec!["C", "Eb", "Gb", "B𝄫", "B", "D", "F", "Ab"])]
#[test_case("Cdim7omit3", vec!["C", "Gb", "B𝄫"])]
#[test_case("Cdimomit3", vec!["C", "Gb"])]
#[test_case("Cdimomit5", vec!["C", "Eb"])]
#[test_case("Cdim7omit5", vec!["C", "Eb", "B𝄫"])]
#[test_case("Cøomit5", vec!["C", "Eb", "Bb"])]
#[test_case("C+omit5",  vec!["C", "E"])]
#[test_case("C+b5omit5", vec!["C", "E"])]
#[test_case("C(#5b5omit5)", vec!["C", "E"])]
#[test_case("Csus2",vec!["C", "G", "D"])]
#[test_case("Csus#4", vec!["C", "G", "F#"])]
#[test_case("Cadd9omit3", vec!["C", "G", "D"])]
#[test_case("Cmin7sus2", vec!["C", "G", "Bb", "D"])]
#[test_case("Cadd9sus#4",vec!["C", "G", "D", "F#"])]
#[test_case("Cmin7sus#4", vec!["C", "G", "Bb", "F#"])]
#[test_case("Cmin7sus4", vec!["C", "F", "G", "Bb"])]
#[test_case("Cdimsus4", vec!["C", "F", "Gb"])]
#[test_case("Cdim7sus4", vec!["C", "F", "Gb", "B𝄫"])]
#[test_case("Csusdim7maj7", vec!["C", "F", "Gb", "B𝄫", "B"])]
#[test_case(" C+susMaj76", vec!["C", "F", "G#", "A", "B"])]
fn test_notes(i: &str, expected: Vec<&str>) {
    let mut parser = Parser::new();
    let res = parser.parse(i);
    let notes = vec![
        Note::new(NoteLiteral::C, Some(Modifier::Flat)),
        Note::new(NoteLiteral::C, Some(Modifier::Sharp)),
        Note::new(NoteLiteral::D, Some(Modifier::Flat)),
        Note::new(NoteLiteral::D, None),
        Note::new(NoteLiteral::D, Some(Modifier::Sharp)),
        Note::new(NoteLiteral::E, Some(Modifier::Flat)),
        Note::new(NoteLiteral::E, None),
        Note::new(NoteLiteral::E, Some(Modifier::Sharp)),
        Note::new(NoteLiteral::F, Some(Modifier::Flat)),
        Note::new(NoteLiteral::F, None),
        Note::new(NoteLiteral::F, Some(Modifier::Sharp)),
        Note::new(NoteLiteral::G, Some(Modifier::Flat)),
        Note::new(NoteLiteral::G, None),
        Note::new(NoteLiteral::G, Some(Modifier::Sharp)),
        Note::new(NoteLiteral::A, Some(Modifier::Flat)),
        Note::new(NoteLiteral::A, None),
        Note::new(NoteLiteral::A, Some(Modifier::Flat)),
        Note::new(NoteLiteral::B, None),
        Note::new(NoteLiteral::B, Some(Modifier::Flat)),
        Note::new(NoteLiteral::A, Some(Modifier::Sharp)),
        Note::new(NoteLiteral::B, Some(Modifier::Sharp)),
    ];
    match res {
        Ok(chord) => {
            let literals = &chord.note_literals;
            assert_eq!(literals, &expected);
            for n in notes {
                let t = chord.transpose_to(&n);
                assert_eq!(
                    chord.intervals, t.intervals,
                    "Error parsing chord {}",
                    chord.origin
                );

                let parsed = parser.parse(&t.origin);
                match parsed {
                    Ok(p) => assert_eq!(
                        t.intervals, p.intervals,
                        "Error transposing chord {} into {}",
                        &chord.origin, n
                    ),
                    Err(e) => panic!("{e}"),
                }
            }
        }
        Err(e) => {
            let a = e.errors.iter().fold("".to_owned(), |acc, e| {
                if acc.is_empty() {
                    e.to_string()
                } else {
                    format!("{acc} {e}")
                }
            });
            panic!("{}", a);
        }
    }
}
