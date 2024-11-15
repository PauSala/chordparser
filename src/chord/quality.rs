use serde::{Deserialize, Serialize};

use super::{intervals::Interval, Chord};

/// Describes the quality of a chord
#[derive(Debug, PartialEq, Default, Eq, Clone, Serialize, Deserialize)]
#[repr(u8)]
pub enum Quality {
    Power,
    #[default]
    Major,
    Major6,
    Major7,
    Minor,
    Minor6,
    Minor7,
    MinorMaj7,
    Dominant,
    Diminished,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum BaseQuality {
    #[default]
    Major,
    Minor,
    Dominant,
    Diminished,
    Augmented,
    Power,
}

impl BaseQuality {
    pub fn quality(rbs: &[bool; 24]) -> BaseQuality {
        if BaseQuality::is_dim(rbs) {
            return BaseQuality::Diminished;
        }
        if BaseQuality::is_aug(rbs) {
            return BaseQuality::Augmented;
        }
        if BaseQuality::is_dom(rbs) {
            return BaseQuality::Dominant;
        }
        if BaseQuality::is_min(rbs) {
            return BaseQuality::Minor;
        }
        if BaseQuality::is_power(rbs) {
            return BaseQuality::Power;
        }
        BaseQuality::Major
    }

    fn is_power(rbs: &[bool; 24]) -> bool {
        rbs[0]
            && !rbs[1]
            && !rbs[2]
            && !rbs[3]
            && !rbs[4]
            && !rbs[5]
            && !rbs[6]
            && rbs[7]
            && !rbs[8]
            && !rbs[9]
            && !rbs[10]
            && !rbs[11]
    }

    fn is_min(rbs: &[bool; 24]) -> bool {
        !(!rbs[3] || rbs[4] || BaseQuality::is_dim(rbs) || rbs[6] && rbs[9])
    }
    fn is_aug(rbs: &[bool; 24]) -> bool {
        !rbs[3] && !rbs[6] && rbs[8] && !rbs[10]
    }
    fn is_dim(rbs: &[bool; 24]) -> bool {
        rbs[6] && !rbs[10] && !rbs[4] && (!(!rbs[3] || !rbs[9] && !rbs[6]) || (rbs[6] && rbs[9]))
    }
    fn is_dom(rbs: &[bool; 24]) -> bool {
        !rbs[3] && rbs[10]
    }
}

impl Quality {
    /// Given a chord, returns its quality
    pub fn from_chord(ch: &Chord) -> Quality {
        let maj6 = ch.has(Interval::MajorSixth);
        let maj7 = ch.has(Interval::MajorSeventh);
        let min7 = ch.has(Interval::MinorSeventh);

        match BaseQuality::quality(&ch.rbs) {
            BaseQuality::Major | BaseQuality::Augmented => {
                if maj6 {
                    return Quality::Major6;
                }
                if maj7 {
                    return Quality::Major7;
                }
                Quality::Major
            }
            BaseQuality::Minor => {
                if maj7 {
                    return Quality::MinorMaj7;
                }
                if min7 {
                    return Quality::Minor7;
                }
                if maj6 {
                    return Quality::Minor6;
                }
                Quality::Minor
            }
            BaseQuality::Dominant => Quality::Dominant,
            BaseQuality::Diminished => Quality::Diminished,
            BaseQuality::Power => {
                // Because it can have extensions (9, 11, etc)
                if ch.semitones.len() == 2 {
                    return Quality::Power;
                }
                Quality::Major
            }
        }
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
    #[test_case("CMin13", Quality::Minor7)]
    #[test_case("CMinb13", Quality::Minor)]
    #[test_case("C-Maj7", Quality::MinorMaj7)]
    #[test_case("CMaj7-", Quality::MinorMaj7)]
    #[test_case("C-7add6", Quality::Minor7)]
    #[test_case("C-69", Quality::Minor6)]
    #[test_case("C-11add6", Quality::Minor7)]
    #[test_case("C-b5", Quality::Diminished)]
    #[test_case("C-7b5", Quality::Minor7)]
    #[test_case("Cdim13", Quality::Minor7)]
    #[test_case("Cdim7", Quality::Diminished)]
    #[test_case("Cdim7Maj7", Quality::Diminished)]
    #[test_case("CdimMaj7", Quality::Diminished)]
    #[test_case("CdimMaj9", Quality::Diminished)]
    fn test_qualities(input: &str, expected: Quality) {
        let mut parser = Parser::new();
        let res = parser.parse(input);
        match res {
            Ok(chord) => {
                // assert_eq!(chord.quality, expected)
                if chord.complete_quality != expected {
                    println!("{}", chord.origin);
                    assert_eq!(chord.complete_quality, expected)
                }
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
