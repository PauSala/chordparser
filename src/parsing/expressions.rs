use crate::chord::{intervals::Interval, note::Note};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExtensionExp {
    pub interval: Interval,
    pub pos: usize,
}
impl ExtensionExp {
    pub fn new(interval: Interval, pos: usize) -> Self {
        Self { interval, pos }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AddExp {
    pub interval: Interval,
    pub target_pos: usize,
}

impl AddExp {
    pub fn new(interval: Interval, target_pos: usize) -> Self {
        Self {
            interval,
            target_pos,
        }
    }
    pub fn isvalid(&self) -> (bool, usize) {
        (
            matches!(
                self.interval,
                Interval::MajorSecond
                    | Interval::MajorThird
                    | Interval::PerfectFourth
                    | Interval::MinorSixth
                    | Interval::MajorSixth
                    | Interval::MajorSeventh
                    | Interval::FlatNinth
                    | Interval::Ninth
                    | Interval::SharpNinth
                    | Interval::Eleventh
                    | Interval::SharpEleventh
                    | Interval::FlatThirteenth
                    | Interval::Thirteenth
            ),
            self.target_pos,
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SusExp {
    pub interval: Interval,
}

impl SusExp {
    pub fn new(interval: Interval) -> Self {
        Self { interval }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OmitExp {
    pub interval: Interval,
    pub target_pos: usize,
}

impl OmitExp {
    pub fn new(interval: Interval, target_pos: usize) -> Self {
        Self {
            interval,
            target_pos,
        }
    }
    pub fn isvalid(&self) -> (bool, usize) {
        (
            matches!(self.interval, Interval::MajorThird | Interval::PerfectFifth),
            self.target_pos,
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SlashBassExp {
    pub note: Note,
}

impl SlashBassExp {
    pub fn new(note: Note) -> Self {
        Self { note }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AltExp;
