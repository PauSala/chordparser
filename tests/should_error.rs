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
fn should_error(i: &str, _expected: Vec<&str>) {
    let mut parser = Parser::new();
    let res = parser.parse(i);
    match res {
        Ok(chord) => panic!("Expected an error, got {:?}", chord),
        Err(e) => {
            let a = e.errors.iter().fold("".to_owned(), |acc, e| {
                if acc.is_empty() {
                    e.to_string()
                } else {
                    format!("{acc} {e}")
                }
            });
            dbg!(a);
        }
    }
}
