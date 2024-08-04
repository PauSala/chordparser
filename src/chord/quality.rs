use serde::{Deserialize, Serialize};

use super::{intervals::Interval, Chord};

/// Describes the quality of a chord
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Quality {
    Power,
    Major,
    Minor,
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

        if min3 {
            if dim5 && (dim7 || !min7) {
                return Quality::Diminished;
            } else if dim5 {
                return Quality::SemiDiminished;
            }
            return Quality::Minor;
        } else if aug5 {
            if min7 {
                return Quality::Dominant;
            }
            return Quality::Augmented;
        } else if min7 {
            return Quality::Dominant;
        } else if maj3 || maj7 || p4 {
            return Quality::Major;
        }
        Quality::Power
    }
}
