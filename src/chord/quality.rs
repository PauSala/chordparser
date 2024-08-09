use serde::{Deserialize, Serialize};

use super::{intervals::Interval, Chord};

/// Describes the quality of a chord
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Quality {
    Power,
    Major,
    Major6,
    Major7,
    Minor,
    Minor6,
    Minor7,
    MinorMaj7,
    Dominant,
    SemiDiminished,
    Diminished,
}

impl Quality {
    /// Given a chord, returns its quality
    pub fn from_chord(chord: &Chord) -> Quality {
        let maj3 = chord.has(Interval::MajorThird);
        let min3 = chord.has(Interval::MinorThird);
        let dim5 = chord.has(Interval::DiminishedFifth);
        let dim7 = chord.has(Interval::DiminishedSeventh);
        let min7 = chord.has(Interval::MinorSeventh);
        let maj7 = chord.has(Interval::MajorSeventh);
        let p4 = chord.has(Interval::PerfectFourth);
        let maj6 = chord.has(Interval::MajorSixth);

        if min3 {
            if dim5 && (dim7 || !min7) {
                return Quality::Diminished;
            } else if dim5 && !maj7 {
                return Quality::SemiDiminished;
            } else if maj7 && !maj6 {
                return Quality::MinorMaj7;
            } else if min7 {
                return Quality::Minor7;
            } else if maj6 {
                return Quality::Minor6;
            }
            return Quality::Minor;
        } else if min7 {
            return Quality::Dominant;
        } else if maj6 {
            return Quality::Major6;
        } else if maj7 {
            return Quality::Major7;
        } else if maj3 || p4 {
            return Quality::Major;
        }
        Quality::Power
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::{chord::quality::Quality, parsing::Parser};

    #[test_case("C5", Quality::Power)]
    #[test_case("C6Maj7", Quality::Major6)]
    #[test_case("Cmaj7no3", Quality::Major7)]
    #[test_case("Cno3", Quality::Power)]
    #[test_case("Cma9omit3", Quality::Major7)]
    #[test_case("C", Quality::Major)]
    #[test_case("CM7", Quality::Major7)]
    #[test_case("CM13", Quality::Major7)]
    #[test_case("CMaj7sus", Quality::Major7)]
    #[test_case("Csus", Quality::Major)]
    #[test_case("CMaj7#5", Quality::Major7)]
    #[test_case("C(#5)", Quality::Major)]
    #[test_case("Cadd9(#5)", Quality::Major)]
    #[test_case("C7sus2", Quality::Dominant)]
    #[test_case("C7sus", Quality::Dominant)]
    #[test_case("C13", Quality::Dominant)]
    #[test_case("CAlt", Quality::Dominant)]
    #[test_case("C7#5", Quality::Dominant)]
    #[test_case("C7(#5,b5)", Quality::Dominant)]
    #[test_case("C13(#5,b5)", Quality::Dominant)]
    #[test_case("CMin13", Quality::Minor7)]
    #[test_case("CMinb13", Quality::Minor)]
    #[test_case("C-Maj7", Quality::MinorMaj7)]
    #[test_case("CMaj7-", Quality::MinorMaj7)]
    #[test_case("C-7add6", Quality::Minor7)]
    #[test_case("C-69", Quality::Minor6)]
    #[test_case("C-11add6", Quality::Minor7)]
    #[test_case("C-b5", Quality::Diminished)]
    #[test_case("C-7b5", Quality::SemiDiminished)]
    #[test_case("Cdim13", Quality::SemiDiminished)]
    #[test_case("Cdim7", Quality::Diminished)]
    #[test_case("Cdim7Maj7", Quality::Diminished)]
    #[test_case("CdimMaj7", Quality::Diminished)]
    #[test_case("CdimMaj9", Quality::Diminished)]
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
}
