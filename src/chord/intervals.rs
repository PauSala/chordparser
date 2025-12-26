//! Useful abstractions to work with intervals

use enum_bitset::EnumBitset;
use serde::Deserialize;
use serde::ser::{Serialize, Serializer};
use std::fmt::Display;

/// Enum representing all possible intervals of a chord
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deserialize, Hash, EnumBitset)]
#[repr(u8)]
pub enum Interval {
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

impl Interval {
    /// Returns the semitone representation of the interval
    /// # Arguments
    /// * `self` - The interval
    /// # Returns
    /// * `u8` - The semitone representation of the interval
    pub fn st(&self) -> u8 {
        match self {
            Interval::Unison => 0,
            Interval::MinorSecond => 1,
            Interval::MajorSecond => 2,
            Interval::MinorThird => 3,
            Interval::MajorThird => 4,
            Interval::PerfectFourth => 5,
            Interval::AugmentedFourth => 6,
            Interval::DiminishedFifth => 6,
            Interval::PerfectFifth => 7,
            Interval::AugmentedFifth => 8,
            Interval::MinorSixth => 8,
            Interval::MajorSixth => 9,
            Interval::DiminishedSeventh => 9,
            Interval::MinorSeventh => 10,
            Interval::MajorSeventh => 11,
            Interval::Octave => 12,
            Interval::FlatNinth => 13,
            Interval::Ninth => 14,
            Interval::SharpNinth => 15,
            Interval::Eleventh => 17,
            Interval::SharpEleventh => 18,
            Interval::FlatThirteenth => 20,
            Interval::Thirteenth => 21,
        }
    }

    /// Transforms the interval into its semantic form, i.e,. for any interval returns its natural form.
    /// # Arguments
    /// * `self` - The interval
    /// # Returns
    /// * `SemInterval` - The semantic interval
    pub fn to_semantic_interval(&self) -> SemInterval {
        match self {
            Interval::Unison => SemInterval::Root,
            Interval::MinorSecond | Interval::MajorSecond => SemInterval::Second,
            Interval::MinorThird | Interval::MajorThird => SemInterval::Third,
            Interval::PerfectFourth | Interval::AugmentedFourth => SemInterval::Fourth,
            Interval::DiminishedFifth | Interval::PerfectFifth | Interval::AugmentedFifth => {
                SemInterval::Fifth
            }
            Interval::MinorSixth | Interval::MajorSixth => SemInterval::Sixth,
            Interval::DiminishedSeventh | Interval::MinorSeventh | Interval::MajorSeventh => {
                SemInterval::Seventh
            }
            Interval::Octave => SemInterval::Root,
            Interval::FlatNinth | Interval::Ninth | Interval::SharpNinth => SemInterval::Ninth,
            Interval::Eleventh | Interval::SharpEleventh => SemInterval::Eleventh,
            Interval::FlatThirteenth | Interval::Thirteenth => SemInterval::Thirteenth,
        }
    }

    /// Transforms given interval into its chord notation form
    /// # Arguments
    /// * `self` - The interval
    /// # Returns
    /// * `String` - The chord notation form for this interval
    pub fn to_chord_notation(&self) -> String {
        match self {
            Interval::Unison => "1".to_string(),
            Interval::MinorSecond => "b2".to_string(),
            Interval::MajorSecond => "2".to_string(),
            Interval::MinorThird => "b3".to_string(),
            Interval::MajorThird => "3".to_string(),
            Interval::PerfectFourth => "4".to_string(),
            Interval::AugmentedFourth => "#4".to_string(),
            Interval::DiminishedFifth => "b5".to_string(),
            Interval::PerfectFifth => "5".to_string(),
            Interval::AugmentedFifth => "#5".to_string(),
            Interval::MinorSixth => "b6".to_string(),
            Interval::MajorSixth => "6".to_string(),
            Interval::DiminishedSeventh => "bb7".to_string(),
            Interval::MinorSeventh => "7".to_string(),
            Interval::MajorSeventh => "Maj7".to_string(),
            Interval::Octave => "8".to_string(),
            Interval::FlatNinth => "b9".to_string(),
            Interval::Ninth => "9".to_string(),
            Interval::SharpNinth => "#9".to_string(),
            Interval::Eleventh => "11".to_string(),
            Interval::SharpEleventh => "#11".to_string(),
            Interval::FlatThirteenth => "b13".to_string(),
            Interval::Thirteenth => "13".to_string(),
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

/// Enum representing semantic intervals, meaning that every interval can be any of its possible values.  
/// It is used to calculate the correct enharmonic notes from given root.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SemInterval {
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

impl SemInterval {
    /// Numeric representation of the semantic interval
    /// # Arguments
    /// * `self` - The semantic interval
    /// # Returns
    /// * `u8` - The int representation of the semantic interval
    pub fn numeric(&self) -> u8 {
        *self as u8
    }
}
