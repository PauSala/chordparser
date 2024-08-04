use core::panic;

use chordparser::parser::Parser;

use test_case::test_case;

#[test_case("Cb-7add3", vec![])]
#[test_case("Cmin7Maj7", vec![])]
#[test_case("C#9b9", vec![])]
#[test_case("C9#9", vec![])]
#[test_case("C11#11", vec![])]
#[test_case("C#11#11", vec![])]
#[test_case("Db13#13", vec![])]
#[test_case("Db137min7", vec![])]
#[test_case("Cma6(b5)", vec![])]
#[test_case("Cma69", vec![])]
#[test_case("Cma6(#5)", vec![])]
#[test_case("Cmano3", vec![])]
#[test_case("CMaj", vec![])]
#[test_case("CAltb9", vec![])]
#[test_case("CAltb13", vec![])]
#[test_case("CAlt#11", vec![])]
#[test_case("C(#11", vec![])]
#[test_case("c-9", vec![])]
#[test_case("C(add9,7)", vec![])]
#[test_case("C(omit3,7)", vec![])]
fn should_error(i: &str, _expected: Vec<&str>) {
    let mut parser = Parser::new();
    let res = parser.parse(i);
    match res {
        Ok(chord) => panic!("Expected an error, got {:?}", chord),
        Err(e) => {
            assert!(e.errors.len() > 0);
        }
    }
}
