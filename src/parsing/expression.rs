use crate::chord::{interval::Interval, note::Note};
use std::fmt::{self};

#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Exp {
    Power,
    Bass,
    Maj,
    Maj7,
    Minor,
    HalfDim,
    Dim,
    Dim7,
    Alt,
    Aug,
    Sus(SusExp),
    Extension(ExtensionExp),
    Add(AddExp),
    Omit(OmitExp),
    SlashBass(SlashBassExp),
}

impl Exp {
    pub fn validate(&self) -> (bool, usize) {
        match self {
            Exp::Omit(exp) => exp.isvalid(),
            Exp::Add(exp) => exp.isvalid(),
            _ => (true, 0),
        }
    }

    pub fn priority(&self) -> u32 {
        match self {
            Exp::Power => 0,
            Exp::Alt => 1,
            Exp::Bass => 2,
            Exp::Minor => 3,
            Exp::Dim7 => 4,
            Exp::Dim => 5,
            Exp::HalfDim => 6,
            Exp::Sus(_) => 7,
            Exp::Maj => 8,
            Exp::Maj7 => 9,
            Exp::Extension(_) => 10,
            Exp::Add(_) => 11,
            Exp::Aug => 12,
            Exp::Omit(_) => 13,
            Exp::SlashBass(_) => 14,
        }
    }
    pub fn from_priority(p: u32) -> &'static str {
        match p {
            0 => "5",
            1 => "Alt",
            2 => "Bass",
            3 => "Minor",
            4 => "Dim7",
            5 => "Dim",
            6 => "halfDim",
            7 => "Sus",
            8 => "Maj",
            9 => "Maj",
            10 => "Extension",
            11 => "Add",
            12 => "Aug",
            13 => "Omit",
            14 => "SlashBass",
            _ => "",
        }
    }
}
impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Exp::Extension(_) => "Extension",
            Exp::Add(_) => "Add",
            Exp::Sus(_) => "Sus",
            Exp::Omit(_) => "Omit",
            Exp::SlashBass(_) => "SlashBass",
            Exp::Bass => "Bass",
            Exp::Alt => "Alt",
            Exp::Minor => "Minor",
            Exp::Aug => "Aug",
            Exp::HalfDim => "HalfDim",
            Exp::Dim => "Dim",
            Exp::Dim7 => "Dim7",
            Exp::Maj => "Maj",
            Exp::Maj7 => "Maj",
            Exp::Power => "Power",
        };
        write!(f, "{}", s)
    }
}

impl Eq for Exp {}

impl PartialOrd for Exp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Exp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.priority().cmp(&other.priority()) {
            std::cmp::Ordering::Equal => match (self, other) {
                (Exp::Extension(expa), Exp::Extension(expb)) => {
                    expa.interval.st().cmp(&expb.interval.st())
                }
                _ => std::cmp::Ordering::Equal,
            },
            other => other,
        }
    }
}

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
