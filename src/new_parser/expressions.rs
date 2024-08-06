use crate::chord::{intervals::Interval, note::Note};

use super::expression::Exp;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExtensionExp {
    pub interval: Interval,
}
impl ExtensionExp {
    pub fn new(interval: Interval) -> Self {
        Self { interval }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AddExp {
    pub interval: Interval,
}

impl AddExp {
    pub fn new(interval: Interval) -> Self {
        Self { interval }
    }
    pub fn isvalid(&self) -> bool {
        matches!(
            self.interval,
            Interval::MajorSecond
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
        )
    }

    pub fn execute(&self, i: &mut Vec<Interval>) {
        if !i.contains(&self.interval) {
            i.push(self.interval);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SusExp {
    pub interval: Interval,
}

impl SusExp {
    pub fn execute(&self, i: &mut Vec<Interval>) {
        let interval = match self.interval {
            Interval::MinorSecond => Interval::FlatNinth,
            Interval::MajorSecond => Interval::Ninth,
            Interval::PerfectFourth => Interval::Eleventh,
            Interval::AugmentedFourth => Interval::SharpEleventh,
            _ => panic!("Invalid sus interval"),
        };
        if !i.contains(&interval) {
            i.push(interval);
        }
    }
}

impl SusExp {
    pub fn new(interval: Interval) -> Self {
        Self { interval }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OmitExp {
    pub interval: Interval,
}
impl OmitExp {
    pub fn new(interval: Interval) -> Self {
        Self { interval }
    }
    pub fn isvalid(&self) -> bool {
        matches!(self.interval, Interval::MajorThird | Interval::PerfectFifth)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SlashBassExp {
    pub note: Note,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BassExp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DimExp;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dim7Exp;
impl Dim7Exp {
    pub fn execute(&self, i: &mut Vec<Interval>) {
        if !i.contains(&Interval::MinorThird) {
            i.push(Interval::MinorThird);
        }
        if !i.contains(&Interval::DiminishedFifth) {
            i.push(Interval::DiminishedFifth);
        }
        if !i.contains(&Interval::DiminishedSeventh) {
            i.push(Interval::DiminishedSeventh);
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HalfDimExp;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MajExp;
impl MajExp {
    pub fn execute(&self, i: &mut Vec<Interval>, exp: &[Exp]) {
        if exp.iter().any(|e| {
            matches!(
                e,
                Exp::Extension(ExtensionExp {
                    interval: Interval::MinorSeventh,
                }) | Exp::Extension(ExtensionExp {
                    interval: Interval::Ninth,
                }) | Exp::Extension(ExtensionExp {
                    interval: Interval::Eleventh,
                }) | Exp::Extension(ExtensionExp {
                    interval: Interval::Thirteenth,
                })
            )
        }) && !i.contains(&Interval::MajorSeventh)
        {
            i.push(Interval::MajorSeventh);
        }

        if !exp.iter().any(|e| {
            matches!(
                e,
                Exp::Sus(_)
                    | Exp::Bass(_)
                    | Exp::Minor(_)
                    | Exp::HalfDim(_)
                    | Exp::Dim(_)
                    | Exp::Dim7(_)
                    | Exp::Power(_)
            )
        }) && !i.contains(&Interval::MajorThird)
        {
            i.push(Interval::MajorThird);
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MinorExp;
impl MinorExp {
    pub fn execute(&self, i: &mut Vec<Interval>) {
        if !i.contains(&Interval::MinorThird) {
            i.push(Interval::MinorThird);
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AugExp;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AltExp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PowerExp;
