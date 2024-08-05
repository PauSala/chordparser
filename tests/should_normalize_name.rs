use chordparser::parser::Parser;
use test_case::test_case;

#[test_case("C5", "C5")]
#[test_case("C6Maj7", "C6(addMaj7)")]
#[test_case("CMaj7#9omit3", "CMaj7(#9,omit3)")]
#[test_case("Cmaj7no3", "CMaj7(omit3)")]
#[test_case("Cmaj7sus4", "CMaj7sus")]
#[test_case("Cmaj7sus2", "CMaj9(omit3)")]
#[test_case("Cno3", "C5")]
#[test_case("Cma9omit3", "CMaj9(omit3)")]
#[test_case("C", "C")]
#[test_case("CM7", "CMaj7")]
#[test_case("CM13", "CMaj13")]
#[test_case("Csus", "Csus")]
#[test_case("CMaj7#5", "CMaj7(#5)")]
#[test_case("C(#5)", "C(#5)")]
#[test_case("Cadd9(#5)", "C(#5,add9)")]
#[test_case("C7sus2", "C9(omit3)")]
#[test_case("C7susb2", "C7(b9,omit3)")]
#[test_case("C7sus#4", "C9(#11,omit3)")]
#[test_case("C7sus", "C7sus")]
#[test_case("C13", "C13")]
#[test_case("C9add13", "C13")]
#[test_case("C11add13", "C13sus")]
#[test_case("CAlt", "C7(b5,b9,#9,#11,b13)")]
#[test_case("C7#5", "C7(#5)")]
#[test_case("C7#5,b5", "C7(b5,#5)")]
#[test_case("C13(#5,b5)", "C13(b5,#5)")]
#[test_case("CMin13add11", "Cmin13")]
#[test_case("CMinb13", "Cmin(b13)")]
#[test_case("CMinb13add9", "Cmin(b13,add9)")]
#[test_case("CMinb139", "Cmin9(b13)")]
#[test_case("C-Maj7", "CminMaj7")]
#[test_case("CMaj7-", "CminMaj7")]
#[test_case("C-7add6", "Cmin7(add6)")]
#[test_case("C-69", "Cmin69")]
#[test_case("C-11add6", "Cmin11(add6)")]
#[test_case("Cminor9", "Cmin9")]
#[test_case("Cminor6add11omit5", "Cmin6(add11,omit5)")]
#[test_case("C-b5", "Cdim")]
#[test_case("C-7b5", "Cmin7(b5)")]
#[test_case("Cdim13", "Cmin13(b5)")]
#[test_case("Cdim7", "Cdim7")]
#[test_case("Cdim7Maj7", "Cdim7(addMaj7)")]
#[test_case("CdimMaj7", "Cdim(addMaj7)")]
#[test_case("CdimMaj9", "Cdim(addMaj7,9)")]
#[test_case("Csus4#11", "Csus(#11,add9)")]
#[test_case("C/A", "C/A")]
#[test_case("Cm6/A", "Cmin6/A")]
#[test_case("C(bass)", "CBass")]
fn test_normalize(input: &str, expected: &str) {
    let mut parser = Parser::new();
    let res = parser.parse(input);
    match res {
        Ok(chord) => {
            assert_eq!(chord.normalized, expected)
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
