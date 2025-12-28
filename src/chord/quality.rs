use enum_bitset::EnumBitset;
use serde::{Deserialize, Serialize};

use crate::chord::intervals::{IntDegree, IntDegreeSet, Interval, IntervalSet};
use ChordQuality::*;
use Interval::*;
use Pc::*;

// Quality sets
const POW_SET: PcSet = PcSet::from_array([Pc7]);
pub(crate) const EXACT_POW_SET: PcSet = PcSet::from_array([Pc::Pc0, Pc::Pc7]);
const MAJ_SET: PcSet = PcSet::from_array([Pc4]);
const MAJ6_SET: PcSet = PcSet::from_array([Pc4, Pc9]);
const MAJ7_SET: PcSet = PcSet::from_array([Pc4, Pc11]);
const DOM7_SET: PcSet = PcSet::from_array([Pc4, Pc10]);

const MIN_SET: PcSet = PcSet::from_array([Pc3]);
const MIN6_SET: PcSet = PcSet::from_array([Pc3, Pc9]);
const MIN7_SET: PcSet = PcSet::from_array([Pc3, Pc10]);
const MIMA7SET: PcSet = PcSet::from_array([Pc3, Pc11]);

const AUG_SET: PcSet = PcSet::from_array([Pc4, Pc8]);
const DIM_SET: PcSet = PcSet::from_array([Pc3, Pc6]);
const DIM7_SET: PcSet = PcSet::from_array([Pc3, Pc6, Pc9]);

// Other convenient sets
const SEVENTH_SET: PcSet = PcSet::from_array([Pc10, Pc11]);
const SUS_SET: PcSet = PcSet::from_array([Pc5, Pc17]);
pub(crate) const THIRDS_SET: PcSet = PcSet::from_array([Pc3, Pc4]);
pub(crate) const FIFTHS_SET: PcSet = PcSet::from_array([Pc6, Pc7, Pc8]);

const QUALITY_SETS: &[(ChordQuality, PcSet)] = &[
    (Dom, DOM7_SET),
    (MiMaj7, MIMA7SET),
    (Mi7, MIN7_SET),
    (Mi6, MIN6_SET),
    (Mi, MIN_SET),
    (Maj6, MAJ6_SET),
    (Maj7, MAJ7_SET),
    (Maj, MAJ_SET),
    (Pow, POW_SET),
];

// Interval sets
const EMPTY_INTERVAL_SET: IntervalSet = IntervalSet::from_array([]);

/// PitchClass: semitones in two octaves
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, EnumBitset)]
pub enum Pc {
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

impl From<&Interval> for Pc {
    fn from(value: &Interval) -> Self {
        match value {
            Unison => Pc0,
            MinorSecond => Pc1,
            MajorSecond => Pc2,
            MinorThird => Pc3,
            MajorThird => Pc4,
            PerfectFourth => Pc5,
            AugmentedFourth | Interval::DiminishedFifth => Pc6,
            PerfectFifth => Pc7,
            AugmentedFifth | Interval::MinorSixth => Pc8,
            MajorSixth | Interval::DiminishedSeventh => Pc9,
            MinorSeventh => Pc10,
            MajorSeventh => Pc11,
            Octave => Pc12,
            FlatNinth => Pc13,
            Ninth => Pc14,
            SharpNinth => Pc15,
            Eleventh => Pc17,
            SharpEleventh => Pc18,
            FlatThirteenth => Pc20,
            Thirteenth => Pc21,
        }
    }
}

impl From<&[Interval]> for PcSet {
    fn from(value: &[Interval]) -> Self {
        value.iter().fold(PcSet::new(), |mut acc, int| {
            acc.insert(Into::<Pc>::into(int));
            acc
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum ChordQuality {
    #[default]
    Maj,
    Maj6,
    Maj7,
    Dom,

    Mi,
    Mi6,
    Mi7,
    MiMaj7,

    Augmented,
    Diminished,
    Diminished7,

    Pow,
    Bass,
}

impl ChordQuality {
    pub(crate) fn is_sus(&self, ints: &PcSet) -> bool {
        match self {
            ChordQuality::Pow | ChordQuality::Bass => false,
            _ => !ints.contains(Pc3) && !ints.intersection(&SUS_SET).is_empty(),
        }
    }

    pub(crate) fn alterations(&self, ints: &IntervalSet) -> IntervalSet {
        self.alteration_mask().intersection(ints)
    }

    pub(crate) fn extensions(&self, ints: &IntervalSet) -> IntervalSet {
        self.extension_mask().intersection(ints)
    }

    pub(crate) fn extension_stack(&self) -> &'static IntDegreeSet {
        const EMPTY_INTERVAL_SET: IntDegreeSet = IntDegreeSet::from_array([]);
        const DEFAULT: IntDegreeSet =
            IntDegreeSet::from_array([IntDegree::Seventh, IntDegree::Ninth, IntDegree::Thirteenth]);
        const M11: IntDegreeSet = IntDegreeSet::from_array([IntDegree::Eleventh]).union(&DEFAULT);
        match self {
            Pow | Bass => &EMPTY_INTERVAL_SET,
            Dom | Maj7 | Maj | Augmented => &DEFAULT,
            _ => &M11,
        }
    }

    fn extension_mask(&self) -> &'static IntervalSet {
        const DEFAULT: IntervalSet =
            IntervalSet::from_array([Interval::Ninth, Interval::Thirteenth]);
        const M7: IntervalSet = IntervalSet::from_array([Interval::MajorSeventh]).union(&DEFAULT);
        const M11: IntervalSet = IntervalSet::from_array([Interval::Eleventh]).union(&DEFAULT);
        const M11_M6: IntervalSet = IntervalSet::from_array([Interval::MajorSixth]).union(&M11);
        const M7_11: IntervalSet = M11.union(&M7);
        const M6: IntervalSet = IntervalSet::from_array([Interval::MajorSixth]).union(&DEFAULT);

        match self {
            Pow | Bass => &EMPTY_INTERVAL_SET,
            Diminished7 | Diminished | Mi6 => &M7_11,
            Mi | Mi7 | MiMaj7 => &M11_M6,
            Maj6 => &M7,
            Dom | Maj7 | Maj | Augmented => &M6,
        }
    }

    fn alteration_mask(&self) -> &'static IntervalSet {
        const DIM: IntervalSet =
            IntervalSet::from_array([AugmentedFifth, MinorSixth, FlatNinth, FlatThirteenth]);
        const AUG: IntervalSet = IntervalSet::from_array([
            DiminishedFifth,
            FlatNinth,
            SharpNinth,
            SharpEleventh,
            FlatThirteenth,
        ]);
        const DEFAULT: IntervalSet = IntervalSet::from_array([
            DiminishedFifth,
            AugmentedFifth,
            MinorSixth,
            FlatNinth,
            SharpNinth,
            SharpEleventh,
            FlatThirteenth,
        ]);
        match self {
            Pow | Bass => &EMPTY_INTERVAL_SET,
            Diminished | Diminished7 => &DIM,
            Augmented => &AUG,
            _ => &DEFAULT,
        }
    }
}

impl From<&[Interval]> for ChordQuality {
    fn from(value: &[Interval]) -> Self {
        let pc: PcSet = value.into();
        (&pc).into()
    }
}

impl From<&PcSet> for ChordQuality {
    fn from(value: &PcSet) -> Self {
        use ChordQuality::*;
        struct Rule {
            quality: ChordQuality,
            matches: fn(&PcSet) -> bool,
        }
        if value.len() == 1 {
            return Bass;
        }

        // Warn: Order matters
        const RULES: &[Rule] = &[
            Rule {
                quality: Augmented,
                matches: is_augmented,
            },
            Rule {
                quality: Diminished7,
                matches: is_diminished7,
            },
            Rule {
                quality: Diminished,
                matches: is_diminished,
            },
        ];
        for rule in RULES {
            if (rule.matches)(value) {
                return rule.quality;
            }
        }
        // Warn II: Order matters as well
        for (quality, set) in QUALITY_SETS {
            if value.is_superset_of(set) {
                return *quality;
            }
        }
        Maj
    }
}

fn is_augmented(value: &PcSet) -> bool {
    // If it has a 7th ir 6th is not handled as aug.
    value.is_superset_of(&AUG_SET) && value.is_disjoint(&SEVENTH_SET) && !value.contains_const(&Pc9)
}
fn is_diminished(value: &PcSet) -> bool {
    // no m7, otherwise b5 will be handled as alteration.
    value.is_superset_of(&DIM_SET) && !value.contains(Pc10)
}
fn is_diminished7(value: &PcSet) -> bool {
    // no m7, otherwise b5 will be handled as alteration.
    value.is_superset_of(&DIM7_SET) && !value.contains(Pc10)
}
