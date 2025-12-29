use chordparser::parsing::Parser;
use test_case::test_case;

#[test_case("C5", "C5")]
#[test_case("C6Ma7", "C6(addMa7)")]
#[test_case("CMa7#9omit3", "CMa7(#9,omit3)")]
#[test_case("Cma7no3", "CMa7(omit3)")]
#[test_case("Cma7sus4", "CMa7sus")]
#[test_case("Cma7sus2", "CMa9(omit3)")]
#[test_case("Cno3", "C5")]
#[test_case("Cma9omit3", "CMa9(omit3)")]
#[test_case("C", "C")]
#[test_case("CM7", "CMa7")]
#[test_case("CM13", "CMa13")]
#[test_case("Csus", "Csus")]
#[test_case("CMa7#5", "CMa7(#5)")]
#[test_case("C(#5)", "C+")]
#[test_case("Cadd9(#5)", "C+(add9)")]
#[test_case("C7sus2", "C9(omit3)")]
#[test_case("C7susb2", "C7(b9,omit3)")]
#[test_case("C7sus#4", "C7(#11,omit3)")]
#[test_case("C7sus", "C7sus")]
#[test_case("C13", "C13")]
#[test_case("C9add13", "C13")]
#[test_case("CAlt", "C7(b9,#9,#11,b13)")]
#[test_case("C7#5", "C7(#5)")]
#[test_case("C7#5,b5", "C7(b5,#5)")]
#[test_case("Cmi13add11", "Cmi13")]
#[test_case("Cmib13", "Cmi(b13)")]
#[test_case("Cmib13add9", "Cmi(b13,add9)")]
#[test_case("Cmib139", "Cmi9(b13)")]
#[test_case("C-Ma7", "CmiMa7")]
#[test_case("CMa7-", "CmiMa7")]
#[test_case("C-7add6", "Cmi7(add13)")]
#[test_case("C-69", "Cmi69")]
#[test_case("C-11add6", "Cmi13")]
#[test_case("Cminor9", "Cmi9")]
#[test_case("Cminor6add11omit5", "Cmi6(add11,omit5)")]
#[test_case("C-b5", "Cdim")]
#[test_case("C-7b5", "Cmi7(b5)")]
#[test_case("Cdim13", "Cmi13(b5)")]
#[test_case("Cdim7", "Cdim7")]
#[test_case("Cdim7Ma7", "Cdim7(addMa7)")]
#[test_case("CdimMa7", "Cdim(addMa7)")]
#[test_case("CdimMa9", "Cdim(addMa7,9)")]
#[test_case("C/A", "C/A")]
#[test_case("Cm6/A", "Cmi6/A")]
#[test_case("C(bass)", "CBass")]
#[test_case("C9", "C9")]
#[test_case("C11add13", "C13sus")]
#[test_case("C11", "C9sus")]
#[test_case("C7(add9,11)", "C9sus")]
#[test_case("Cma7(add9,11)", "CMa9sus")]
#[test_case("C-(add9,13)", "Cmi(add9,13)")]
#[test_case("C-11(add13)", "Cmi13")]
#[test_case("C-b511(add9,b6)", "Cmi11(b5,b6)")]
#[test_case("C-9add11", "Cmi11")]
#[test_case("CBass", "CBass")]
#[test_case("C119b5+-7", "Cmi11(b5,#5)")]
#[test_case("C4", "Csus")]
#[test_case("C94", "C9sus")]
#[test_case("C49", "C9sus")]
#[test_case("Cm7#11add9,add13", "Cmi13(#11)")]
#[test_case("Cm#11(add9,13)", "Cmi(#11,add9,13)")]
#[test_case("C-7add11add13", "Cmi7(add11,13)")]
#[test_case("C-add11add13", "Cmi(add11,13)")]
#[test_case("C-7add#11add9add13", "Cmi13(#11)")]
#[test_case("C+dim", "Cdim(#5)")]
#[test_case("C+dim7", "Cdim7(#5)")]
#[test_case("Cdim9", "Cmi9(b5)")]
#[test_case("Cdim6", "Cdim7")]
#[test_case("Cdimadd9", "Cdim(add9)")]
#[test_case("Cdim7ma711b13", "Cdim7(b13,addMa7,9,11)")]
#[test_case("Cdim7omit3", "Cdim7(omit3)")]
#[test_case("Cdimomit3", "Cdim(omit3)")]
#[test_case("Cdimomit5", "Cmi(omit5)")]
#[test_case("Cdim7omit5", "Cmi6(omit5)")]
#[test_case("Cøomit5", "Cmi7(omit5)")]
#[test_case("C+omit5", "C(omit5)")]
#[test_case("C+b5omit5", "C(omit5)")]
#[test_case("Cb#5b5omit5", "Cb(omit5)")]
#[test_case("Csus2", "C(add9,omit3)")]
#[test_case("Csus#4", "C(#11,omit3)")]
#[test_case("Cadd9omit3", "C(add9,omit3)")]
#[test_case("Cadd9sus#4", "C(#11,add9,omit3)")]
#[test_case("Cmi7sus2", "Cmi9(omit3)")]
#[test_case("Cmi7sus#4", "Cmi7(#11,omit3)")]
#[test_case("Cmi7sus4", "Cmi7sus")]
#[test_case("Cmi7omit3", "Cmi7(omit3)")]
#[test_case("Csusdim", "Cdimsus")]
#[test_case("Csusdim7", "Cdim7sus")]
#[test_case("Csusdim7omit5", "Cmi6sus(omit5)")]
#[test_case("Cdim67", "Cdim7")]
#[test_case("Csusdim7ma7", "Cdim7sus(addMa7)")]
#[test_case("C+susMa76", "C6sus(#5,addMa7)")]
fn test_normalize(input: &str, expected: &str) {
    let mut parser = Parser::new();

    let chord = parser.parse(input).unwrap_or_else(|e| {
        let msg = e
            .errors
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        panic!("Failed to parse `{}`: {}", input, msg);
    });

    assert_eq!(
        chord.normalized, expected,
        "Normalization mismatch for `{}`",
        input
    );
}
