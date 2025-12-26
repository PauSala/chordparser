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

static MAJ_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc4]);
static MAJ6_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc4, PitchClass::Pc9]);
static MAJ7_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc4, PitchClass::Pc11]);
static DOMT_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc4, PitchClass::Pc10]);

static MIN_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc3]);
static MIN6_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc3, PitchClass::Pc9]);
static MIN7_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc3, PitchClass::Pc10]);
static MIMA7SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc3, PitchClass::Pc11]);

static AUG_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc4, PitchClass::Pc8]);
static DIM_SET: PitchClassSet = PitchClassSet::from_array([PitchClass::Pc3, PitchClass::Pc6]);
static DIM7_SET: PitchClassSet =
    PitchClassSet::from_array([PitchClass::Pc3, PitchClass::Pc6, PitchClass::Pc9]);

static QUALITY_SETS: &[(ChordQuality, PitchClassSet)] = &[
    (ChordQuality::Diminished7, DIM7_SET),
    (ChordQuality::Diminished, DIM_SET),
    (ChordQuality::Augmented, AUG_SET),
    (ChordQuality::MinorMaj7, MIMA7SET),
    (ChordQuality::Minor7, MIN7_SET),
    (ChordQuality::Minor6, MIN6_SET),
    (ChordQuality::Minor, MIN_SET),
    (ChordQuality::Dominant7, DOMT_SET),
    (ChordQuality::Major7, MAJ7_SET),
    (ChordQuality::Major6, MAJ6_SET),
    (ChordQuality::Major, MAJ_SET),
];

impl From<&PitchClassSet> for ChordQuality {
    fn from(value: &PitchClassSet) -> Self {
        for (quality, set) in QUALITY_SETS {
            if value.is_superset_of(set) {
                return *quality;
            }
        }
        panic!("No matching ChordQuality for {:?}", value);
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
    Pc15, // #9
    Pc16, // m10 (♭3 octave)
    Pc17, // M10 (3 octave)
    Pc18, // 11
    Pc19, // #11
    Pc20, // 12 (5 octave)
    Pc21, // b13
    Pc22, // 13
    Pc23, // M14 (7 octave)
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
