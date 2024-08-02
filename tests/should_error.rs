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
#[test_case("Cma6(b5)", vec!["C", "E", "Gb", "A"])]
#[test_case("Cma69", vec!["C", "E", "G", "A", "D"])]
#[test_case("Cma6(#5)", vec!["C", "E", "G#", "A"])]
#[test_case("Cmano3", vec!["C", "G"])]
#[test_case("CMaj", vec!["C", "E", "G" ])]
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
