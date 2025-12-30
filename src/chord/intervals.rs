//! Useful abstractions to work with intervals

use Interval::*;
use enum_bitset::EnumBitset;
use serde::Deserialize;
use serde::ser::{Serialize, Serializer};
use std::fmt::Display;

/// Enum representing intervals of a chord
#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deserialize, Hash, EnumBitset, Default,
)]
#[repr(u8)]
pub enum Interval {
    #[default]
    Unison,
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectFourth,
    AugmentedFourth,
    DiminishedFifth,
    PerfectFifth,
    AugmentedFifth,
    MinorSixth,
    MajorSixth,
    DiminishedSeventh,
    MinorSeventh,
    MajorSeventh,
    Octave,
    FlatNinth,
    Ninth,
    SharpNinth,
    Eleventh,
    SharpEleventh,
    FlatThirteenth,
    Thirteenth,
}

impl IntervalSet {
    /// Removes a specific interval from the set and adds another.
    ///
    /// **Unconditionally** removes `remove` and inserts `add`.
    pub fn remove_then_add(&mut self, remove: Interval, add: Interval) {
        self.remove(remove);
        self.insert(add);
    }

    /// Returns a new `IntervalSet` where a specific interval is replaced with another.
    ///
    /// Produces a **new set** with the following behavior:
    /// - Every occurrence of `target` in the original set is replaced by `dest`.
    /// - The original set is **not modified**.
    pub fn replace(&self, target: Interval, dest: Interval) -> IntervalSet {
        self.iter()
            .map(|a| if a == target { dest } else { a })
            .collect()
    }
}

impl Interval {
    /// Returns the semitone representation of the interval
    /// ## Arguments
    /// * `self` - The interval
    /// ## Returns
    /// * `u8` - The semitone representation of the interval
    pub fn st(&self) -> u8 {
        match self {
            Unison => 0,
            MinorSecond => 1,
            MajorSecond => 2,
            MinorThird => 3,
            MajorThird => 4,
            PerfectFourth => 5,
            AugmentedFourth => 6,
            DiminishedFifth => 6,
            PerfectFifth => 7,
            AugmentedFifth => 8,
            MinorSixth => 8,
            MajorSixth => 9,
            DiminishedSeventh => 9,
            MinorSeventh => 10,
            MajorSeventh => 11,
            Octave => 12,
            FlatNinth => 13,
            Ninth => 14,
            SharpNinth => 15,
            Eleventh => 17,
            SharpEleventh => 18,
            FlatThirteenth => 20,
            Thirteenth => 21,
        }
    }

    /// Transforms the interval into its degree, i.e,. for any interval returns its natural form.
    /// # Arguments
    /// * `self` - The interval
    /// # Returns
    /// * `SemInterval` - The interval degree
    pub fn to_degree(&self) -> IntDegree {
        match self {
            Unison => IntDegree::Root,
            MinorSecond | MajorSecond => IntDegree::Second,
            MinorThird | MajorThird => IntDegree::Third,
            PerfectFourth | AugmentedFourth => IntDegree::Fourth,
            DiminishedFifth | PerfectFifth | AugmentedFifth => IntDegree::Fifth,
            MinorSixth | MajorSixth => IntDegree::Sixth,
            DiminishedSeventh | MinorSeventh | MajorSeventh => IntDegree::Seventh,
            Octave => IntDegree::Root,
            FlatNinth | Ninth | SharpNinth => IntDegree::Ninth,
            Eleventh | SharpEleventh => IntDegree::Eleventh,
            FlatThirteenth | Thirteenth => IntDegree::Thirteenth,
        }
    }

    /// Transforms given interval into its chord notation form
    /// # Arguments
    /// * `self` - The interval
    /// # Returns
    /// * `String` - The chord notation form for this interval
    pub fn to_chord_notation(&self) -> String {
        match self {
            Unison => "1".to_string(),
            MinorSecond => "b2".to_string(),
            MajorSecond => "2".to_string(),
            MinorThird => "b3".to_string(),
            MajorThird => "3".to_string(),
            PerfectFourth => "4".to_string(),
            AugmentedFourth => "#4".to_string(),
            DiminishedFifth => "b5".to_string(),
            PerfectFifth => "5".to_string(),
            AugmentedFifth => "#5".to_string(),
            MinorSixth => "b6".to_string(),
            MajorSixth => "6".to_string(),
            DiminishedSeventh => "bb7".to_string(),
            MinorSeventh => "7".to_string(),
            MajorSeventh => "Ma7".to_string(),
            Octave => "8".to_string(),
            FlatNinth => "b9".to_string(),
            Ninth => "9".to_string(),
            SharpNinth => "#9".to_string(),
            Eleventh => "11".to_string(),
            SharpEleventh => "#11".to_string(),
            FlatThirteenth => "b13".to_string(),
            Thirteenth => "13".to_string(),
        }
    }

    pub fn from_chord_notation(i: &str) -> Option<Interval> {
        match i {
            "1" => Some(Interval::Unison),
            "b2" => Some(Interval::MinorSecond),
            "2" => Some(Interval::MajorSecond),
            "b3" => Some(Interval::MinorThird),
            "3" => Some(Interval::MajorThird),
            "4" => Some(Interval::PerfectFourth),
            "#4" => Some(Interval::AugmentedFourth),
            "b5" => Some(Interval::DiminishedFifth),
            "5" => Some(Interval::PerfectFifth),
            "#5" => Some(Interval::AugmentedFifth),
            "b6" => Some(Interval::MinorSixth),
            "6" => Some(Interval::MajorSixth),
            "bb7" => Some(Interval::DiminishedSeventh),
            "7" => Some(Interval::MinorSeventh),
            "maj7" => Some(Interval::MajorSeventh),
            "8" => Some(Interval::Octave),
            "b9" => Some(Interval::FlatNinth),
            "9" => Some(Interval::Ninth),
            "#9" => Some(Interval::SharpNinth),
            "11" => Some(Interval::Eleventh),
            "#11" => Some(Interval::SharpEleventh),
            "b13" => Some(Interval::FlatThirteenth),
            "13" => Some(Interval::Thirteenth),
            _ => None,
        }
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_chord_notation())
    }
}

impl Serialize for Interval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_chord_notation().as_str())
    }
}

/// Enum representing interval degrees.
/// It is used to calculate the correct enharmonic notes from given root.
#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deserialize, Hash, EnumBitset, Default,
)]
pub enum IntDegree {
    #[default]
    Root = 1,
    Second = 2,
    Third = 3,
    Fourth = 4,
    Fifth = 5,
    Sixth = 6,
    Seventh = 7,
    Ninth = 9,
    Eleventh = 11,
    Thirteenth = 13,
}

impl From<Interval> for IntDegree {
    fn from(value: Interval) -> Self {
        match value {
            Unison => IntDegree::Root,
            MinorSecond => IntDegree::Second,
            MajorSecond => IntDegree::Second,
            MinorThird => IntDegree::Third,
            MajorThird => IntDegree::Third,
            PerfectFourth => IntDegree::Fourth,
            AugmentedFourth => IntDegree::Fourth,
            DiminishedFifth => IntDegree::Fourth,
            PerfectFifth => IntDegree::Fifth,
            AugmentedFifth => IntDegree::Fifth,
            MinorSixth => IntDegree::Sixth,
            MajorSixth => IntDegree::Sixth,
            DiminishedSeventh => IntDegree::Seventh,
            MinorSeventh => IntDegree::Seventh,
            MajorSeventh => IntDegree::Seventh,
            Octave => IntDegree::Root,
            FlatNinth => IntDegree::Ninth,
            Ninth => IntDegree::Ninth,
            SharpNinth => IntDegree::Ninth,
            Eleventh => IntDegree::Eleventh,
            SharpEleventh => IntDegree::Eleventh,
            FlatThirteenth => IntDegree::Thirteenth,
            Thirteenth => IntDegree::Thirteenth,
        }
    }
}

impl From<&IntervalSet> for IntDegreeSet {
    fn from(value: &IntervalSet) -> Self {
        value
            .iter()
            .map(<Interval as Into<IntDegree>>::into)
            .collect()
    }
}

impl IntDegree {
    /// Numeric representation of the interval degree
    /// # Arguments
    /// * `self` - The interval degree
    /// # Returns
    /// * `u8` - The int representation of the interval degree
    pub fn numeric(&self) -> u8 {
        *self as u8
    }
}
