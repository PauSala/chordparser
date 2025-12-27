use enum_bitset::EnumBitset;

use crate::chord::intervals::Interval;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ChordQuality {
    Major,
    Major6,
    Major7,
    Dominant7,

    Minor,
    Minor6,
    Minor7,
    MinorMaj7,

    Augmented,
    Diminished,
    Diminished7,

    Power,
    Bass,
}

const POW_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc7]);
const MAJ_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc4]);
const MAJ6_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc4, PitchClass::Pc9]);
const MAJ7_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc4, PitchClass::Pc11]);
const DOM7_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc4, PitchClass::Pc10]);

const MIN_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc3]);
const MIN6_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc3, PitchClass::Pc9]);
const MIN7_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc3, PitchClass::Pc10]);
const MIMA7SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc3, PitchClass::Pc11]);

const AUG_SET: PitchClassSet =
    PitchClassSet::from_array([PitchClass::Pc0, PitchClass::Pc4, PitchClass::Pc8]);
const DIM_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc3, PitchClass::Pc6]);
const DIM7_SET: PitchClassSet =
    PitchClassSet::from_array([PitchClass::Pc3, PitchClass::Pc6, PitchClass::Pc9]);

const QUALITY_SETS: &[(ChordQuality, PitchClassSet)] = &[
    (ChordQuality::Dominant7, DOM7_SET),
    (ChordQuality::Diminished7, DIM7_SET),
    (ChordQuality::Diminished, DIM_SET),
    (ChordQuality::MinorMaj7, MIMA7SET),
    (ChordQuality::Minor7, MIN7_SET),
    (ChordQuality::Minor6, MIN6_SET),
    (ChordQuality::Minor, MIN_SET),
    (ChordQuality::Major6, MAJ6_SET),
    (ChordQuality::Major7, MAJ7_SET),
    (ChordQuality::Major, MAJ_SET),
    (ChordQuality::Power, POW_SET),
];

impl From<&PitchClassSet> for ChordQuality {
    fn from(value: &PitchClassSet) -> Self {
        if value.difference(&AUG_SET).is_empty() {
            return ChordQuality::Augmented;
        }
        for (quality, set) in QUALITY_SETS {
            if value.is_superset_of(set) {
                return *quality;
            }
        }
        ChordQuality::Bass
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, EnumBitset)]
pub enum PitchClass {
    Pc0,  // Root (1)
    Pc1,  // m2
    Pc2,  // M2
    Pc3,  // m3
    Pc4,  // M3
    Pc5,  // P4
    Pc6,  // #4 / b5
    Pc7,  // P5
    Pc8,  // #5 / b6
    Pc9,  // M6
    Pc10, // m7
    Pc11, // M7

    Pc12, // Octave (8)
    Pc13, // b9
    Pc14, // 9
    Pc15, // #9 / ♭3
    Pc16, // M10 / 3
    Pc17, // 11
    Pc18, // #11
    Pc19, // 12 / 5
    Pc20, // b13
    Pc21, // 13
    Pc22, // m7
    Pc23, // M7
}

impl From<&[Interval]> for PitchClassSet {
    fn from(value: &[Interval]) -> Self {
        value.iter().fold(PitchClassSet::new(), |mut acc, int| {
            let pc = match int {
                Interval::Unison => PitchClass::Pc0,
                Interval::MinorSecond => PitchClass::Pc1,
                Interval::MajorSecond => PitchClass::Pc2,
                Interval::MinorThird => PitchClass::Pc3,
                Interval::MajorThird => PitchClass::Pc4,
                Interval::PerfectFourth => PitchClass::Pc5,
                Interval::AugmentedFourth | Interval::DiminishedFifth => PitchClass::Pc6,
                Interval::PerfectFifth => PitchClass::Pc7,
                Interval::AugmentedFifth | Interval::MinorSixth => PitchClass::Pc8,
                Interval::MajorSixth | Interval::DiminishedSeventh => PitchClass::Pc9,
                Interval::MinorSeventh => PitchClass::Pc10,
                Interval::MajorSeventh => PitchClass::Pc11,
                Interval::Octave => PitchClass::Pc12,
                Interval::FlatNinth => PitchClass::Pc13,
                Interval::Ninth => PitchClass::Pc14,
                Interval::SharpNinth => PitchClass::Pc15,
                Interval::Eleventh => PitchClass::Pc18,
                Interval::SharpEleventh => PitchClass::Pc19,
                Interval::FlatThirteenth => PitchClass::Pc21,
                Interval::Thirteenth => PitchClass::Pc22,
            };
            acc.insert(pc);
            acc
        })
    }
}

// #[cfg(test)]
// mod tests {

//     use test_case::test_case;

//     use crate::parsing::{
//         Parser,
//         chord_quality::{ChordQuality, PitchClassSet},
//     };

//     #[test_case("C5", ChordQuality::Power)]
//     #[test_case("C6Maj7", ChordQuality::Major6)]
//     // #[test_case("Cmaj7no3", ChordQuality::Major7)]
//     #[test_case("Cno3", ChordQuality::Power)]
//     // #[test_case("Cma9omit3", ChordQuality::Major7)]
//     #[test_case("C", ChordQuality::Major)]
//     #[test_case("CM7", ChordQuality::Major7)]
//     #[test_case("CM13", ChordQuality::Major7)]
//     // #[test_case("CMaj7sus", ChordQuality::Major7)]
//     // #[test_case("Csus", ChordQuality::Major)]
//     #[test_case("CMaj7#5", ChordQuality::Major7)]
//     #[test_case("C(#5)", ChordQuality::Augmented)]
//     #[test_case("Cadd9(#5)", ChordQuality::Major)]
//     // #[test_case("C7sus2", ChordQuality::Dominant7)] // fails
//     // #[test_case("C7sus", ChordQuality::Dominant7)]
//     #[test_case("C13", ChordQuality::Dominant7)]
//     #[test_case("CAlt", ChordQuality::Dominant7)]
//     #[test_case("C7#5", ChordQuality::Dominant7)]
//     #[test_case("C7(#5,b5)", ChordQuality::Dominant7)]
//     #[test_case("CMin13", ChordQuality::Minor7)]
//     #[test_case("CMinb13", ChordQuality::Minor)]
//     #[test_case("C-Maj7", ChordQuality::MinorMaj7)]
//     #[test_case("CMaj7-", ChordQuality::MinorMaj7)]
//     #[test_case("C-7add6", ChordQuality::Minor7)]
//     #[test_case("C-69", ChordQuality::Minor6)]
//     #[test_case("C-11add6", ChordQuality::Minor7)]
//     #[test_case("C-b5", ChordQuality::Diminished)]
//     // #[test_case("C-7b5", ChordQuality::Minor7)]
//     // #[test_case("Cdim13", ChordQuality::Minor7)]
//     #[test_case("Cdim7", ChordQuality::Diminished7)]
//     #[test_case("Cdim7Maj7", ChordQuality::Diminished7)]
//     #[test_case("CdimMaj7", ChordQuality::Diminished)]
//     #[test_case("CdimMaj9", ChordQuality::Diminished)]
//     fn test_quality(input: &str, expected: ChordQuality) {
//         let mut parser = Parser::new();
//         let res = parser.parse(input);
//         match res {
//             Ok(chord) => {
//                 let intervals = chord.norm_intervals;
//                 let pitch_class_set: PitchClassSet = intervals.as_slice().into();
//                 let quality: ChordQuality = (&pitch_class_set).into();
//                 dbg!(intervals);
//                 dbg!(pitch_class_set);
//                 assert_eq!(quality, expected);
//             }
//             Err(e) => {
//                 let a = e.errors.iter().fold("".to_owned(), |acc, e| {
//                     if acc.is_empty() {
//                         e.to_string()
//                     } else {
//                         format!("{acc} {e}")
//                     }
//                 });
//                 panic!("{}", a);
//             }
//         }
//     }
// }
