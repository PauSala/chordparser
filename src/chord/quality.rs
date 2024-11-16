use serde::{Deserialize, Serialize};

use super::{intervals::Interval, Chord};

/// Describes the quality of a chord
#[derive(Debug, PartialEq, Default, Eq, Clone, Serialize, Deserialize)]
#[repr(u8)]
pub enum InnerQuality {
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
pub enum Quality {
    #[default]
    Major,
    Minor,
    Dominant,
    Diminished,
    Augmented,
    Power,
}

impl Quality {
    pub fn quality(rbs: &[bool; 24]) -> Quality {
        if Quality::is_dim(rbs) {
            return Quality::Diminished;
        }
        if Quality::is_aug(rbs) {
            return Quality::Augmented;
        }
        if Quality::is_dom(rbs) {
            return Quality::Dominant;
        }
        if Quality::is_min(rbs) {
            return Quality::Minor;
        }
        if Quality::is_power(rbs) {
            return Quality::Power;
        }
        Quality::Major
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
        !(!rbs[3] || rbs[4] || Quality::is_dim(rbs) || rbs[6] && rbs[9])
            || (rbs[3] && (rbs[10] || rbs[11]))
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

impl InnerQuality {
    /// Given a chord, returns its quality
    pub fn from_chord(ch: &Chord) -> InnerQuality {
        dbg!(&ch.real_intervals);
        let maj6 = ch.has(Interval::MajorSixth);
        let maj7 = ch.has(Interval::MajorSeventh);
        let min7 = ch.has(Interval::MinorSeventh);

        match Quality::quality(&ch.rbs) {
            Quality::Major | Quality::Augmented => {
                if maj6 {
                    return InnerQuality::Major6;
                }
                if maj7 {
                    return InnerQuality::Major7;
                }
                InnerQuality::Major
            }
            Quality::Minor => {
                if maj7 {
                    return InnerQuality::MinorMaj7;
                }
                if min7 {
                    return InnerQuality::Minor7;
                }
                if maj6 {
                    return InnerQuality::Minor6;
                }
                InnerQuality::Minor
            }
            Quality::Dominant => InnerQuality::Dominant,
            Quality::Diminished => InnerQuality::Diminished,
            Quality::Power => {
                // Because it can have extensions (9, 11, etc)
                if ch.semitones.len() == 2 {
                    return InnerQuality::Power;
                }
                InnerQuality::Major
            }
        }
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::{chord::quality::InnerQuality, parsing::Parser};

    #[test_case("C5", InnerQuality::Power)]
    #[test_case("C6Maj7", InnerQuality::Major6)]
    #[test_case("Cmaj7no3", InnerQuality::Major7)]
    #[test_case("Cno3", InnerQuality::Power)]
    #[test_case("Cma9omit3", InnerQuality::Major7)]
    #[test_case("C", InnerQuality::Major)]
    #[test_case("CM7", InnerQuality::Major7)]
    #[test_case("CM13", InnerQuality::Major7)]
    #[test_case("CMaj7sus", InnerQuality::Major7)]
    #[test_case("Csus", InnerQuality::Major)]
    #[test_case("CMaj7#5", InnerQuality::Major7)]
    #[test_case("C(#5)", InnerQuality::Major)]
    #[test_case("Cadd9(#5)", InnerQuality::Major)]
    #[test_case("C7sus2", InnerQuality::Dominant)]
    #[test_case("C7sus", InnerQuality::Dominant)]
    #[test_case("C13", InnerQuality::Dominant)]
    #[test_case("CAlt", InnerQuality::Dominant)]
    #[test_case("C7#5", InnerQuality::Dominant)]
    #[test_case("C7(#5,b5)", InnerQuality::Dominant)]
    #[test_case("CMin13", InnerQuality::Minor7)]
    #[test_case("CMinb13", InnerQuality::Minor)]
    #[test_case("C-Maj7", InnerQuality::MinorMaj7)]
    #[test_case("CMaj7-", InnerQuality::MinorMaj7)]
    #[test_case("C-7add6", InnerQuality::Minor7)]
    #[test_case("C-69", InnerQuality::Minor6)]
    #[test_case("C-11add6", InnerQuality::Minor7)]
    #[test_case("C-b5", InnerQuality::Diminished)]
    #[test_case("C-7b5", InnerQuality::Minor7)]
    #[test_case("Cdim13", InnerQuality::Minor7)]
    #[test_case("Cdim7", InnerQuality::Diminished)]
    #[test_case("Cdim7Maj7", InnerQuality::Diminished)]
    #[test_case("CdimMaj7", InnerQuality::Diminished)]
    #[test_case("CdimMaj9", InnerQuality::Diminished)]
    fn test_qualities(input: &str, expected: InnerQuality) {
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
