use serde::{Deserialize, Serialize};

use super::{Chord, intervals::Interval};

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
    pub fn new(rbs: &[bool; 24]) -> Quality {
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
        rbs[3] && ((!rbs[4] && !Quality::is_dim(rbs) && !(rbs[6] && rbs[9])) || rbs[10] || rbs[11])
    }

    fn is_aug(rbs: &[bool; 24]) -> bool {
        !rbs[3] && !rbs[6] && rbs[8] && !rbs[10]
    }
    fn is_dim(rbs: &[bool; 24]) -> bool {
        rbs[6] && !rbs[10] && !rbs[4] && (rbs[3] || rbs[9])
    }
    fn is_dom(rbs: &[bool; 24]) -> bool {
        !rbs[3] && rbs[10]
    }
}

impl InnerQuality {
    /// Given a chord, returns its quality
    pub fn from_chord(ch: &Chord) -> InnerQuality {
        let maj6 = ch.has(Interval::MajorSixth);
        let maj7 = ch.has(Interval::MajorSeventh);
        let min7 = ch.has(Interval::MinorSeventh);

        match Quality::new(&ch.rbs) {
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
