//! Quality of the chord and its derived rules

use crate::chord::interval::{IntDegree, IntDegreeSet, Interval, IntervalSet};
use ChordQuality::*;
use Interval::*;
use Pc::*;
use enum_bitset::EnumBitset;
use serde::{Deserialize, Serialize};

// Quality sets
const EMPTY_SET: PcSet = PcSet::from_array([]);
const POW_SET: PcSet = PcSet::from_array([Pc7]);
pub(crate) const EXACT_POW_SET: PcSet = PcSet::from_array([Pc::Pc0, Pc::Pc7]);
const MAJ_SET: PcSet = PcSet::from_array([Pc4]);
const MAJ6_SET: PcSet = PcSet::from_array([Pc4, Pc9]);
const MAJ7_SET: PcSet = PcSet::from_array([Pc4, Pc11]);
const DOM7_SET: PcSet = PcSet::from_array([Pc4, Pc10]);
const SUS7_SET: PcSet = PcSet::from_array([Pc5, Pc10]);

const MIN_SET: PcSet = PcSet::from_array([Pc3]);
const MIN6_SET: PcSet = PcSet::from_array([Pc3, Pc9]);
const MIN7_SET: PcSet = PcSet::from_array([Pc3, Pc10]);
const MIMA7SET: PcSet = PcSet::from_array([Pc3, Pc11]);

const AUG_SET: PcSet = PcSet::from_array([Pc4, Pc8]);
const DIM_SET: PcSet = PcSet::from_array([Pc3, Pc6]);
const DIM7_SET: PcSet = PcSet::from_array([Pc3, Pc6, Pc9]);

// Other convenient sets
const SEVENTH_SET: PcSet = PcSet::from_array([Pc10, Pc11]);
const SUS_SET: PcSet = PcSet::from_array([Pc5]);

const QUALITY_SETS: &[(ChordQuality, PcSet)] = &[
    (Dominant7, DOM7_SET),
    (Maj6, MAJ6_SET),
    (Maj7, MAJ7_SET),
    (Maj, MAJ_SET),
    (MiMaj7, MIMA7SET),
    (Mi7, MIN7_SET),
    (Mi6, MIN6_SET),
    (Mi, MIN_SET),
    (Dominant7, SUS7_SET),
    (Power, POW_SET),
];

// Interval sets
const EMPTY_INTERVAL_SET: IntervalSet = IntervalSet::from_array([]);

/// PitchClass: semitones in two octaves
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, EnumBitset)]
pub(crate) enum Pc {
    Pc0,  // Root
    Pc1,  // m2
    Pc2,  // M2
    Pc3,  // m3
    Pc4,  // M3
    Pc5,  // P4
    Pc6,  // #4 / b5
    Pc7,  // P5
    Pc8,  // #5 / b6
    Pc9,  // M6 / d7
    Pc10, // m7
    Pc11, // M7
}

impl From<&Interval> for Pc {
    fn from(value: &Interval) -> Self {
        match value {
            Unison | Octave => Pc0,
            MinorSecond | FlatNinth => Pc1,
            MajorSecond | Ninth => Pc2,
            MinorThird | SharpNinth => Pc3,
            MajorThird => Pc4,
            PerfectFourth | Eleventh => Pc5,
            AugmentedFourth | DiminishedFifth | SharpEleventh => Pc6,
            PerfectFifth => Pc7,
            AugmentedFifth | MinorSixth | FlatThirteenth => Pc8,
            MajorSixth | DiminishedSeventh | Thirteenth => Pc9,
            MinorSeventh => Pc10,
            MajorSeventh => Pc11,
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

impl From<IntervalSet> for PcSet {
    fn from(value: IntervalSet) -> Self {
        value.iter().fold(PcSet::new(), |mut acc, int| {
            acc.insert(Into::<Pc>::into(&int));
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
    Dominant7,

    Mi,
    Mi6,
    Mi7,
    MiMaj7,

    Augmented,
    Diminished,
    Diminished7,

    Power,
    Bass,
}

impl ChordQuality {
    pub(crate) fn is_sus(&self, ints: &PcSet) -> bool {
        match self {
            ChordQuality::Power | ChordQuality::Bass => false,
            _ => !ints.contains(Pc3) && !ints.intersection(&SUS_SET).is_empty(),
        }
    }

    pub(crate) fn self_mask(&self) -> PcSet {
        match self {
            Maj => MAJ_SET,
            Maj6 => MAJ6_SET,
            Maj7 => MAJ7_SET,
            Dominant7 => DOM7_SET,
            Mi => MIN_SET,
            Mi6 => MIN6_SET,
            Mi7 => MIN7_SET,
            MiMaj7 => MIMA7SET,
            Augmented => AUG_SET,
            Diminished => DIM_SET,
            Diminished7 => DIM7_SET,
            Power => POW_SET,
            Bass => EMPTY_SET,
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
            Power | Bass => &EMPTY_INTERVAL_SET,
            Dominant7 | Maj7 | Maj | Augmented => &DEFAULT,
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
            Power | Bass => &EMPTY_INTERVAL_SET,
            Diminished7 | Diminished | Mi6 => &M7_11,
            Mi | Mi7 | MiMaj7 => &M11_M6,
            Maj6 => &M7,
            Dominant7 | Maj7 | Maj | Augmented => &M6,
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
            Power | Bass => &EMPTY_INTERVAL_SET,
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

const DIM_SET_EXCLUDES: PcSet = PcSet::from_array([Pc4, Pc7, Pc10]);

fn is_augmented(value: &PcSet) -> bool {
    // If it has a 7th or 6th is not handled as aug.
    value.is_superset_of(&AUG_SET) && value.is_disjoint(&SEVENTH_SET) && !value.contains_const(&Pc9)
}
fn is_diminished(value: &PcSet) -> bool {
    // no m7 or M3, otherwise b5 will be handled as alteration.
    value.is_superset_of(&DIM_SET) && value.intersection(&DIM_SET_EXCLUDES).is_empty()
}
fn is_diminished7(value: &PcSet) -> bool {
    // no m7 or M3, otherwise b5 will be handled as alteration.
    value.is_superset_of(&DIM7_SET) && value.intersection(&DIM_SET_EXCLUDES).is_empty()
}
