//! Useful abstractions to work with intervals

use Interval::*;
use enum_bitset::EnumBitset;
use serde::Deserialize;
use serde::ser::{Serialize, Serializer};
use std::fmt::Display;

pub const THIRDS_SET: IntervalSet =
    IntervalSet::from_array([Interval::MinorThird, Interval::MajorThird]);

pub const FIFTHS_SET: IntervalSet = IntervalSet::from_array([
    Interval::DiminishedFifth,
    Interval::PerfectFifth,
    Interval::AugmentedFifth,
]);

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
    pub fn to_chord_notation(&self) -> &'static str {
        match self {
            Unison => "1",
            MinorSecond => "b2",
            MajorSecond => "2",
            MinorThird => "b3",
            MajorThird => "3",
            PerfectFourth => "4",
            AugmentedFourth => "#4",
            DiminishedFifth => "b5",
            PerfectFifth => "5",
            AugmentedFifth => "#5",
            MinorSixth => "b6",
            MajorSixth => "6",
            DiminishedSeventh => "bb7",
            MinorSeventh => "7",
            MajorSeventh => "Ma7",
            Octave => "8",
            FlatNinth => "b9",
            Ninth => "9",
            SharpNinth => "#9",
            Eleventh => "11",
            SharpEleventh => "#11",
            FlatThirteenth => "b13",
            Thirteenth => "13",
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
        serializer.serialize_str(&format!("{:?}", self))
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
