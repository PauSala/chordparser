use core::panic;

use chordparser::parsing::Parser;
use test_case::test_case;

#[test_case("FbG△7", vec![])]
#[test_case("F/G/C", vec![])]
#[test_case("Cb-7add3", vec![])]
#[test_case("C-add3", vec![])]
#[test_case("Cmin7Maj7", vec![])]
#[test_case("Cminb2", vec![])]
#[test_case("Cminb", vec![])]
#[test_case("Cmin#", vec![])]
#[test_case("C#9b9", vec![])]
#[test_case("C9#9", vec![])]
#[test_case("C11#11", vec![])]
#[test_case("C#11#11", vec![])]
#[test_case("Db13#13", vec![])]
#[test_case("Db1313", vec![])]
#[test_case("Db1sus3", vec![])]
#[test_case("Dbadd#3", vec![])]
#[test_case("Gomit", vec![])]
#[test_case("Gomit7", vec![])]
#[test_case("Gsus4sus2", vec![])]
#[test_case("C6/11", vec![])]
#[test_case("C(#11", vec![])]
#[test_case("c-9", vec![])]
#[test_case("C(add9,7)", vec![])]
#[test_case("C(omit3,7)", vec![])]
#[test_case("C13(#5,b5)", vec![])]
#[test_case("Csus4#11", vec![])]
#[test_case("C-9(add13)b5#5",  vec![])]
#[test_case("C-b513(add9,b6)", vec![])]
#[test_case("CMaj7b9", vec![])]
#[test_case("maj7b9", vec![])]
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
