use std::fmt::{Display, Formatter};

use super::expressions::{
    AddExp, AltExp, AugExp, BassExp, Dim7Exp, DimExp, ExtensionExp, HalfDimExp, MajExp, MinorExp,
    OmitExp, PowerExp, SlashBassExp, SusExp,
};

#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Exp {
    Power(PowerExp),
    Alt(AltExp),
    Bass(BassExp),
    Minor(MinorExp),
    Dim7(Dim7Exp),
    Dim(DimExp),
    HalfDim(HalfDimExp),
    Sus(SusExp),
    Maj(MajExp),
    Extension(ExtensionExp),
    Add(AddExp),
    Aug(AugExp),
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

    pub fn stringify(&self) -> String {
        match self {
            Exp::Extension(_) => "Extension".to_string(),
            Exp::Add(_) => "Add".to_string(),
            Exp::Sus(_) => "Sus".to_string(),
            Exp::Omit(_) => "Omit".to_string(),
            Exp::SlashBass(_) => "SlashBass".to_string(),
            Exp::Bass(_) => "Bass".to_string(),
            Exp::Alt(_) => "Alt".to_string(),
            Exp::Minor(_) => "Minor".to_string(),
            Exp::Aug(_) => "Aug".to_string(),
            Exp::HalfDim(_) => "HalfDim".to_string(),
            Exp::Dim(_) => "Dim".to_string(),
            Exp::Dim7(_) => "Dim7".to_string(),
            Exp::Maj(_) => "Maj".to_string(),
            Exp::Power(_) => "Power".to_string(),
        }
    }

    pub fn priority(&self) -> u32 {
        match self {
            Exp::Power(_) => 0,
            Exp::Alt(_) => 1,
            Exp::Bass(_) => 2,
            Exp::Minor(_) => 3,
            Exp::Dim7(_) => 4,
            Exp::Dim(_) => 5,
            Exp::HalfDim(_) => 6,
            Exp::Sus(_) => 7,
            Exp::Maj(_) => 8,
            Exp::Extension(_) => 9,
            Exp::Add(_) => 10,
            Exp::Aug(_) => 11,
            Exp::Omit(_) => 12,
            Exp::SlashBass(_) => 13,
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
            9 => "Extension".to_string(),
            10 => "Add".to_string(),
            11 => "Aug".to_string(),
            12 => "Omit".to_string(),
            13 => "SlashBass".to_string(),
            _ => panic!("Invalid priority"),
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
