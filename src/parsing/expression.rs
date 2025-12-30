use crate::chord::{interval::Interval, note::Note};
use std::fmt::{Display, Formatter};

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

    fn stringify(&self) -> String {
        match self {
            Exp::Extension(_) => "Extension".to_string(),
            Exp::Add(_) => "Add".to_string(),
            Exp::Sus(_) => "Sus".to_string(),
            Exp::Omit(_) => "Omit".to_string(),
            Exp::SlashBass(_) => "SlashBass".to_string(),
            Exp::Bass => "Bass".to_string(),
            Exp::Alt => "Alt".to_string(),
            Exp::Minor => "Minor".to_string(),
            Exp::Aug => "Aug".to_string(),
            Exp::HalfDim => "HalfDim".to_string(),
            Exp::Dim => "Dim".to_string(),
            Exp::Dim7 => "Dim7".to_string(),
            Exp::Maj => "Maj".to_string(),
            Exp::Maj7 => "Maj".to_string(),
            Exp::Power => "Power".to_string(),
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
    pub fn from_priority(p: u32) -> String {
        match p {
            0 => "5".to_string(),
            1 => "Alt".to_string(),
            2 => "Bass".to_string(),
            3 => "Minor".to_string(),
            4 => "Dim7".to_string(),
            5 => "Dim".to_string(),
            6 => "halfDim".to_string(),
            7 => "Sus".to_string(),
            8 => "Maj".to_string(),
            9 => "Maj".to_string(),
            10 => "Extension".to_string(),
            11 => "Add".to_string(),
            12 => "Aug".to_string(),
            13 => "Omit".to_string(),
            14 => "SlashBass".to_string(),
            _ => "".to_string(),
        }
    }
}
impl Display for Exp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.stringify().fmt(f)
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
