#[derive(Debug, PartialEq, Eq, Clone)]
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

// 0  1  2  3  4  5  6  7  8  9  10  11 12 13 14 15 16 17 18 19 20 21 22 23 24
// T     2  m3 M3 P4 b5 P5 b6 6  m7  M7
impl From<&[bool; 24]> for ChordQuality {
    fn from(value: &[bool; 24]) -> Self {
        todo!()
    }
}
