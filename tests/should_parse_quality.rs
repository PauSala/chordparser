use chordparser::{chord::quality::Quality, parser::Parser};
use test_case::test_case;

#[test_case("C5", Quality::Power)]
#[test_case("Cmaj7no3", Quality::Major)]
#[test_case("Cno3", Quality::Power)]
#[test_case("Cma9omit3", Quality::Major)]
#[test_case("C", Quality::Major)]
#[test_case("CM7", Quality::Major)]
#[test_case("CM13", Quality::Major)]
#[test_case("CMaj7sus", Quality::Major)]
#[test_case("Csus", Quality::Major)]
#[test_case("CMaj7#5", Quality::Augmented)]
#[test_case("C(#5)", Quality::Augmented)]
#[test_case("Cadd9(#5)", Quality::Augmented)]
#[test_case("C7sus2", Quality::Dominant)]
#[test_case("C7sus", Quality::Dominant)]
#[test_case("C13", Quality::Dominant)]
#[test_case("CAlt", Quality::Dominant)]
#[test_case("C7#5", Quality::Dominant)]
#[test_case("C7(#5,b5)", Quality::Dominant)]
#[test_case("C13(#5,b5)", Quality::Dominant)]
#[test_case("CMin13", Quality::Minor)]
#[test_case("CMinb13", Quality::Minor)]
#[test_case("C-Maj7", Quality::Minor)]
#[test_case("CMaj7-", Quality::Minor)]
#[test_case("C-b5", Quality::SemiDiminished)]
#[test_case("C-7b5", Quality::SemiDiminished)]
#[test_case("Cdim7", Quality::Diminished)]
#[test_case("Cdim7Maj7", Quality::Diminished)]
fn test_qualities(input: &str, expected: Quality) {
    let mut parser = Parser::new();
    let res = parser.parse(input);
    match res {
        Ok(chord) => {
            assert_eq!(chord.quality, expected)
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
