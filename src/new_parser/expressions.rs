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
    fn include_seventh(&self, i: &mut Vec<Interval>) {
        if !i.contains(&Interval::MinorSeventh)
            && !i.contains(&Interval::MajorSeventh)
            && !i.contains(&Interval::DiminishedSeventh)
        {
            i.push(Interval::MinorSeventh);
        }
    }
    fn include_ninth(&self, i: &mut Vec<Interval>) {
        if !i.contains(&Interval::Ninth)
            && !i.contains(&Interval::FlatNinth)
            && !i.contains(&Interval::SharpNinth)
        {
            i.push(Interval::Ninth);
        }
    }
    fn include_eleventh(&self, i: &mut Vec<Interval>) {
        if !i.contains(&Interval::Eleventh)
            && !i.contains(&Interval::SharpEleventh)
            && i.contains(&Interval::MinorThird)
        {
            i.push(Interval::Eleventh);
        }
    }
    pub fn execute(&self, i: &mut Vec<Interval>) {
        match self.interval {
            Interval::PerfectFourth
            | Interval::AugmentedFourth
            | Interval::DiminishedFifth
            | Interval::AugmentedFifth
            | Interval::MinorSixth
            | Interval::MajorSixth
            | Interval::FlatNinth
            | Interval::SharpNinth
            | Interval::SharpEleventh
            | Interval::FlatThirteenth => {
                if !i.contains(&self.interval) {
                    i.push(self.interval);
                }
            }
            Interval::MinorSeventh => {
                if !i.contains(&self.interval)
                    && !i.contains(&Interval::MajorSeventh)
                    && !i.contains(&Interval::DiminishedSeventh)
                {
                    i.push(self.interval);
                }
            }
            Interval::Ninth => {
                self.include_seventh(i);
                self.include_ninth(i);
            }
            Interval::Eleventh => {
                self.include_seventh(i);
                self.include_ninth(i);
                if !i.contains(&self.interval) && !i.contains(&Interval::SharpEleventh) {
                    i.push(self.interval);
                }
            }
            Interval::Thirteenth => {
                self.include_seventh(i);
                self.include_ninth(i);
                self.include_eleventh(i);
                if !i.contains(&self.interval) && !i.contains(&Interval::FlatThirteenth) {
                    i.push(self.interval);
                }
            }
            _ => (),
        }
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
impl DimExp {
    pub fn execute(&self, i: &mut Vec<Interval>) {
        if !i.contains(&Interval::MinorThird) {
            i.push(Interval::MinorThird);
        }
        if !i.contains(&Interval::DiminishedFifth) {
            i.push(Interval::DiminishedFifth);
        }
    }
}
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
