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
    Augmented,
}

impl Quality {
    /// Given a chord, returns its quality
    pub fn from_chord(chord: &Chord) -> Quality {
        let maj3 = chord.has(Interval::MajorThird);
        let min3 = chord.has(Interval::MinorThird);
        let dim5 = chord.has(Interval::DiminishedFifth);
        let aug5 = chord.has(Interval::AugmentedFifth);
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
        } else if aug5 {
            if min7 {
                return Quality::Dominant;
            }
            return Quality::Augmented;
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
