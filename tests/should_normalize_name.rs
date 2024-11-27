use chordparser::parsing::Parser;
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
#[test_case("C7sus#4", "C7(#11,omit3)")]
#[test_case("C7sus", "C7sus")]
#[test_case("C13", "C13")]
#[test_case("C9add13", "C13")]
#[test_case("CAlt", "C7(b9,#9,#11,b13)")]
#[test_case("C7#5", "C7(#5)")]
#[test_case("C7#5,b5", "C7(b5,#5)")]
#[test_case("Cmin13add11", "Cmin13")]
#[test_case("Cminb13", "Cmin(b13)")]
#[test_case("Cminb13add9", "Cmin(b13,add9)")]
#[test_case("Cminb139", "Cmin9(b13)")]
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
#[test_case("C/A", "C/A")]
#[test_case("Cm6/A", "Cmin6/A")]
#[test_case("C(bass)", "CBass")]
#[test_case("C9", "C9")]
#[test_case("C11add13", "C13sus")]
#[test_case("C11", "C9sus")]
#[test_case("C7(add9,11)", "C9sus")]
#[test_case("Cmaj7(add9,11)", "CMaj9sus")]
#[test_case("C-(add9,13)", "Cmin(add9,13)")]
#[test_case("C-11(add13)", "Cmin13")]
#[test_case("C-b511(add9,b6)", "Cmin11(b5,b6)")]
#[test_case("C-9add11", "Cmin11")]
#[test_case("CBass", "CBass")]
#[test_case("C119b5+-7", "Cmin11(b5,#5)")]
#[test_case("C4", "Csus")]
#[test_case("C94", "C9sus")]
#[test_case("C49", "C9sus")]
#[test_case("Cm7#11add9,add13", "Cmin13(#11)")]
#[test_case("Cm#11(add9,13)", "Cmin(#11,add9,13)")]
#[test_case("C-7add11add13", "Cmin7(add13,11)")]
#[test_case("C-add11add13", "Cmin(add11,13)")]
#[test_case("C-7add#11add9add13", "Cmin13(#11)")]
#[test_case("C+dim", "Cdim(#5)")]
#[test_case("C+dim7", "Cdim7(#5)")]
#[test_case("Cdim9", "Cmin9(b5)")]
#[test_case("Cdim6", "Cdim7")]
#[test_case("Cdimadd9", "Cdim(add9)")]
#[test_case("Cdim7maj711b13", "Cdim7(b13,addMaj7,9,11)")]
#[test_case("Cdim7omit3", "Cdim7(omit3)")]
#[test_case("Cdimomit3", "C(b5,omit3)")]
#[test_case("Cdimomit5", "Cmin(omit5)")]
#[test_case("Cdim7omit5", "Cmin6(omit5)")]
#[test_case("Cøomit5", "Cmin7(omit5)")]
#[test_case("C+omit5", "C(omit5)")]
#[test_case("C+b5omit5", "C(omit5)")]
#[test_case("Cb#5b5omit5", "Cb(omit5)")]
#[test_case("Csus2", "C(add9,omit3)")]
#[test_case("Csus#4", "C(#11,omit3)")]
#[test_case("Cadd9omit3", "C(add9,omit3)")]
#[test_case("Cadd9sus#4", "C(#11,add9,omit3)")]
#[test_case("Cmin7sus2", "C9(omit3)")]
#[test_case("Cmin7sus#4", "C7(#11,omit3)")]
#[test_case("Cmin7sus4", "C7sus")]
#[test_case("Cmin7omit3", "C7(omit3)")]
#[test_case("Csusdim", "Csus(b5)")]
#[test_case("Csusdim7", "Cdim7sus")]
#[test_case("Csusdim7omit5", "C6sus(omit5)")] //Cdim67
#[test_case("Cdim67", "Cmin7(b5,add6)")]
#[test_case("Csusdim7maj7", "Cdim7sus(addMaj7)")]
#[test_case("C+susMaj76", "C6sus(#5,addMaj7)")]
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
