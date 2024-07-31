use chordparser::{chord::semantics::{Modifier, Note, NoteLiteral}, parser::Parser};

use test_case::test_case;

#[test_case("C5", vec!["C", "G"])]
#[test_case("C(omit3)", vec!["C", "G"])]
#[test_case("Csus", vec!["C", "F", "G"])]
#[test_case("C(b5)", vec!["C", "E", "Gb"])]
#[test_case("C", vec!["C", "E", "G"])]
#[test_case("C+", vec!["C", "E", "G#"]; "C+")]
#[test_case("C6(b5)", vec!["C", "E", "Gb", "A"])]
#[test_case("C6", vec!["C", "E", "G", "A"])]
#[test_case("C6(#5)", vec!["C", "E", "G#", "A"])]
#[test_case("C69", vec!["C", "E", "G", "A", "D"])]
#[test_case("C69(#11)", vec!["C", "E", "G", "A", "D", "F#"])]
#[test_case("Cma6(b5)", vec!["C", "E", "Gb", "A"])]
#[test_case("Cma69", vec!["C", "E", "G", "A", "D"])]
#[test_case("Cma6(#5)", vec!["C", "E", "G#", "A"])]
#[test_case("Cma7(b5)", vec!["C", "E", "Gb", "B"])]
#[test_case("Cma7", vec!["C", "E", "G", "B"])]
#[test_case("Cma7(#5)", vec!["C", "E", "G#", "B"])]
#[test_case("Cadd9(omit3)", vec!["C", "G", "D"])]
#[test_case("Cadd9(no3)", vec!["C", "G", "D"])]
#[test_case("Cadd9", vec!["C", "E", "G", "D"])]
#[test_case("C(add9)", vec!["C", "E", "G", "D"])]
#[test_case("Cma9", vec!["C", "E", "G", "Bb", "D"])]
#[test_case("Cma9(no3)", vec!["C", "G", "Bb", "D"])]
#[test_case("Cma9no3", vec!["C", "G", "Bb", "D"])]
#[test_case("Cmano3", vec!["C", "G"])]
#[test_case("Cma9(#11)", vec!["C", "E", "G", "Bb", "D", "F#"])]
#[test_case("Cma9(omit3)", vec!["C", "G", "Bb", "D"])]
#[test_case("Cma13", vec!["C", "E", "G", "Bb", "D", "A"])]
#[test_case("Cma13(#11)", vec!["C", "E", "G", "Bb", "D", "F#", "A"])]
#[test_case("C°", vec!["C", "Eb", "Gb"]; "C° is dim")]
#[test_case("Cmi", vec!["C", "Eb", "G"])]
#[test_case("Cmi add9", vec!["C", "Eb", "G", "D"]; "Cmi add9 is minor")]
#[test_case("Cmi(add9)", vec!["C", "Eb", "G", "D"])]
#[test_case("Cmiadd9", vec!["C", "Eb", "G", "D"])]
#[test_case("Cmi6", vec!["C", "Eb", "G", "A"])]
#[test_case("Cmi69", vec!["C", "Eb", "G", "A", "D"])]
#[test_case("Cmi6/9", vec!["C", "Eb", "G", "A", "D"])]
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
#[test_case("Csus4(b5#5)", vec!["C", "F", "Gb", "G#"])]
#[test_case("Bdim7Maj7b13", vec!["B", "D", "F", "Ab", "A#", "C#", "E", "G"])]
#[test_case("CMaj", vec!["C", "E", "G" ])]
fn test_notes(i: &str, expected: Vec<&str>) {
    let mut parser = Parser::new();
    let res = parser.parse(i);
    match res {
        Ok(chord) => {
            //dbg!(&chord);
            let t = chord.transpose_to_root(&Note::new(NoteLiteral::E, Some(Modifier::Flat)));
            dbg!(t);
            dbg!(&chord.note_literals);
            dbg!(&chord.real_intervals);
            let literals = chord.note_literals;
            assert_eq!(literals, expected);
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
