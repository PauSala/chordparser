use enum_bitset::EnumBitset;

use crate::chord::intervals::{Interval, IntervalSet};

const POW_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc7]);
const MAJ_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc4]);
const MAJ6_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc4, PitchClass::Pc9]);
const MAJ7_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc4, PitchClass::Pc11]);
const DOM7_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc4, PitchClass::Pc10]);

const MIN_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc3]);
const MIN6_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc3, PitchClass::Pc9]);
const MIN7_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc3, PitchClass::Pc10]);
const MIMA7SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc3, PitchClass::Pc11]);

const AUG_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc4, PitchClass::Pc8]);
const DIM_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc3, PitchClass::Pc6]);
const DIM7_SET: PitchClassSet =
    PitchClassSet::from_array([PitchClass::Pc3, PitchClass::Pc6, PitchClass::Pc9]);
const SEVENTH_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc10, PitchClass::Pc11]);
const SUS_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc5, PitchClass::Pc17]);
const THIRDS_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc3, PitchClass::Pc4]);

const QUALITY_SETS: &[(ChordQuality, PitchClassSet)] = &[
    (ChordQuality::Dominant7, DOM7_SET),
    (ChordQuality::MinorMaj7, MIMA7SET),
    (ChordQuality::Minor7, MIN7_SET),
    (ChordQuality::Minor6, MIN6_SET),
    (ChordQuality::Minor, MIN_SET),
    (ChordQuality::Major6, MAJ6_SET),
    (ChordQuality::Major7, MAJ7_SET),
    (ChordQuality::Major, MAJ_SET),
    (ChordQuality::Power, POW_SET),
];

const EMPTY_INTERVAL_SET: IntervalSet = IntervalSet::from_array([]);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, EnumBitset)]
pub enum PitchClass {
    Pc0,  // Root
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

    Pc12, // Root
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

impl From<&Interval> for PitchClass {
    fn from(value: &Interval) -> Self {
        match value {
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
        }
    }
}

impl From<&[Interval]> for PitchClassSet {
    fn from(value: &[Interval]) -> Self {
        value.iter().fold(PitchClassSet::new(), |mut acc, int| {
            acc.insert(Into::<PitchClass>::into(int));
            acc
        })
    }
}

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

impl ChordQuality {
    pub(crate) fn is_sus(&self, ints: &PitchClassSet) -> bool {
        match self {
            ChordQuality::Power | ChordQuality::Bass => false,
            _ => {
                ints.intersection(&THIRDS_SET).is_empty() && !ints.intersection(&SUS_SET).is_empty()
            }
        }
    }

    pub(crate) fn alterations(&self, ints: &IntervalSet) -> IntervalSet {
        self.alteration_mask().intersection(ints)
    }

    pub(crate) fn extensions(&self, ints: &IntervalSet) -> IntervalSet {
        self.extension_mask().intersection(ints)
    }

    fn extension_mask(&self) -> &'static IntervalSet {
        const DEFAULT: IntervalSet =
            IntervalSet::from_array([Interval::Ninth, Interval::Thirteenth]);
        const M7: IntervalSet = IntervalSet::from_array([Interval::MajorSeventh]).union(&DEFAULT);
        const M11: IntervalSet = IntervalSet::from_array([Interval::Eleventh]).union(&DEFAULT);
        const M7_11: IntervalSet = M11.union(&M7);
        match self {
            ChordQuality::Power | ChordQuality::Bass => &EMPTY_INTERVAL_SET,
            ChordQuality::Diminished7 | ChordQuality::Diminished | ChordQuality::Minor6 => &M7_11,
            ChordQuality::Minor | ChordQuality::Minor7 | ChordQuality::MinorMaj7 => &M11,
            ChordQuality::Major6 => &M7,
            ChordQuality::Dominant7
            | ChordQuality::Major7
            | ChordQuality::Major
            | ChordQuality::Augmented => &DEFAULT,
        }
    }

    fn alteration_mask(&self) -> &'static IntervalSet {
        const DIM: IntervalSet = IntervalSet::from_array([
            Interval::AugmentedFifth,
            Interval::MinorSixth,
            Interval::FlatNinth,
            Interval::FlatThirteenth,
        ]);
        const AUG: IntervalSet = IntervalSet::from_array([
            Interval::DiminishedFifth,
            Interval::FlatNinth,
            Interval::SharpNinth,
            Interval::SharpEleventh,
            Interval::FlatThirteenth,
        ]);
        const DEFAULT: IntervalSet = IntervalSet::from_array([
            Interval::DiminishedFifth,
            Interval::AugmentedFifth,
            Interval::MinorSixth,
            Interval::FlatNinth,
            Interval::SharpNinth,
            Interval::SharpEleventh,
            Interval::FlatThirteenth,
        ]);
        match self {
            ChordQuality::Power | ChordQuality::Bass => &EMPTY_INTERVAL_SET,
            ChordQuality::Diminished | ChordQuality::Diminished7 => &DIM,
            ChordQuality::Augmented => &AUG,
            _ => &DEFAULT,
        }
    }
}

impl From<&[Interval]> for ChordQuality {
    fn from(value: &[Interval]) -> Self {
        let pc: PitchClassSet = value.into();
        (&pc).into()
    }
}

impl From<&PitchClassSet> for ChordQuality {
    fn from(value: &PitchClassSet) -> Self {
        use ChordQuality::*;
        struct Rule {
            quality: ChordQuality,
            matches: fn(&PitchClassSet) -> bool,
        }
        const RULES: &[Rule] = &[
            Rule {
                quality: Augmented,
                matches: is_augmented,
            },
            Rule {
                quality: Diminished,
                matches: is_diminished,
            },
            Rule {
                quality: Diminished7,
                matches: is_diminished7,
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
    // If it has a 7th ir 6th is not handled as aug.
    value.is_superset_of(&AUG_SET)
        && value.is_disjoint(&SEVENTH_SET)
        && !value.contains_const(&PitchClass::Pc9)
}
fn is_diminished(value: &PitchClassSet) -> bool {
    // no m7, otherwise b5 will be handled as alteration.
    value.is_superset_of(&DIM_SET) && !value.contains(PitchClass::Pc10)
}
fn is_diminished7(value: &PitchClassSet) -> bool {
    // no m7, otherwise b5 will be handled as alteration.
    value.is_superset_of(&DIM7_SET) && !value.contains(PitchClass::Pc10)
}
