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
    (ChordQuality::MinorMaj7, MIMA7SET),
    (ChordQuality::Minor7, MIN7_SET),
    (ChordQuality::Minor6, MIN6_SET),
    (ChordQuality::Minor, MIN_SET),
    (ChordQuality::Major6, MAJ6_SET),
    (ChordQuality::Major7, MAJ7_SET),
    (ChordQuality::Major, MAJ_SET),
    (ChordQuality::Power, POW_SET),
];

impl From<&[Interval]> for ChordQuality {
    fn from(value: &[Interval]) -> Self {
        let pc: PitchClassSet = value.into();
        (&pc).into()
    }
}

impl From<&PitchClassSet> for ChordQuality {
    fn from(value: &PitchClassSet) -> Self {
        use ChordQuality::*;

        const RULES: &[QualityRule] = &[
            QualityRule {
                quality: Augmented,
                matches: is_augmented,
            },
            QualityRule {
                quality: Diminished,
                matches: is_diminished,
            },
        ];

        for rule in RULES {
            if (rule.matches)(value) {
                return rule.quality;
            }
        }

        for (quality, set) in QUALITY_SETS {
            if value.is_superset_of(set) {
                return *quality;
            }
        }

        Bass
    }
}

fn is_augmented(value: &PitchClassSet) -> bool {
    value.difference(&AUG_SET).is_empty() // Must match exactly, otherwise +5 will be handled as alteration.
}

fn is_diminished(value: &PitchClassSet) -> bool {
    value.is_superset_of(&DIM_SET) && !value.contains(PitchClass::Pc10) // no m7, otherwise b6 will be handled as alteration. 
}

struct QualityRule {
    quality: ChordQuality,
    matches: fn(&PitchClassSet) -> bool,
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
