use core::panic;

use crate::chord::{intervals::Interval, note::Note};

use super::expression::Exp;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExtensionExp {
    pub interval: Interval,
    pub pos: usize,
}
impl ExtensionExp {
    pub fn new(interval: Interval, pos: usize) -> Self {
        Self { interval, pos }
    }
    fn include_seventh(&self, i: &mut Vec<Interval>) {
        if !i.contains(&Interval::MajorSixth)
            && !i.contains(&Interval::MinorSeventh)
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
    pub fn execute(&self, i: &mut Vec<Interval>, is_sus: &mut bool, exp: &[Exp]) {
        match self.interval {
            Interval::PerfectFourth
            | Interval::AugmentedFourth
            | Interval::MinorSixth
            | Interval::FlatNinth
            | Interval::SharpNinth
            | Interval::SharpEleventh
            | Interval::FlatThirteenth => {
                if !i.contains(&self.interval) {
                    i.push(self.interval);
                }
            }
            Interval::AugmentedFifth => {
                if !i.contains(&self.interval)
                    && !exp.iter().any(|e| {
                        matches!(
                            e,
                            Exp::Omit(OmitExp {
                                interval: Interval::PerfectFifth,
                                ..
                            })
                        )
                    })
                {
                    i.push(self.interval);
                }
            }
            Interval::DiminishedFifth => {
                if !i.contains(&self.interval)
                    && !exp.iter().any(|e| {
                        matches!(
                            e,
                            Exp::Omit(OmitExp {
                                interval: Interval::PerfectFifth,
                                ..
                            })
                        )
                    })
                {
                    i.push(self.interval);
                }
            }
            Interval::MajorSixth => {
                if !i.contains(&self.interval) && !i.contains(&Interval::Thirteenth) {
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
                i.push(Interval::Ninth);
            }
            Interval::Eleventh => {
                self.include_seventh(i);
                self.include_ninth(i);
                if !i.contains(&self.interval) {
                    i.push(self.interval);
                }
                *is_sus = !i.contains(&Interval::MinorThird);
            }
            Interval::Thirteenth => {
                self.include_seventh(i);
                self.include_ninth(i);
                self.include_eleventh(i);
                if !i.contains(&self.interval) {
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
    pub fn new(interval: Interval) -> Self {
        Self { interval }
    }
    pub fn execute(&self, i: &mut Vec<Interval>) {
        let interval = match self.interval {
            Interval::MinorSecond => Interval::FlatNinth,
            Interval::MajorSecond => Interval::Ninth,
            Interval::PerfectFourth => Interval::PerfectFourth,
            Interval::AugmentedFourth => Interval::SharpEleventh,
            _ => panic!("Invalid sus interval, this should not happen"),
        };
        if !i.contains(&interval) {
            i.push(interval);
        }
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SlashBassExp {
    pub note: Note,
}
impl SlashBassExp {
    pub fn new(note: Note) -> Self {
        Self { note }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BassExp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DimExp;
impl DimExp {
    pub fn execute(&self, i: &mut Vec<Interval>, exp: &[Exp]) {
        if !i.contains(&Interval::MinorThird)
            && !exp.iter().any(|e| {
                matches!(
                    e,
                    Exp::Omit(OmitExp {
                        interval: Interval::MajorThird,
                        ..
                    })
                )
            })
            && !exp.iter().any(|e| matches!(e, Exp::Sus(SusExp { .. })))
        {
            i.push(Interval::MinorThird);
        }
        if !i.contains(&Interval::DiminishedFifth)
            && !exp.iter().any(|e| {
                matches!(
                    e,
                    Exp::Omit(OmitExp {
                        interval: Interval::PerfectFifth,
                        ..
                    })
                )
            })
        {
            i.push(Interval::DiminishedFifth);
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dim7Exp;
impl Dim7Exp {
    pub fn execute(&self, i: &mut Vec<Interval>, exp: &[Exp]) {
        if !i.contains(&Interval::MinorThird)
            && !exp.iter().any(|e| {
                matches!(
                    e,
                    Exp::Omit(OmitExp {
                        interval: Interval::MajorThird,
                        ..
                    })
                )
            })
            && !exp.iter().any(|e| matches!(e, Exp::Sus(SusExp { .. })))
        {
            i.push(Interval::MinorThird);
        }
        if !i.contains(&Interval::DiminishedFifth)
            && !exp.iter().any(|e| {
                matches!(
                    e,
                    Exp::Omit(OmitExp {
                        interval: Interval::PerfectFifth,
                        ..
                    })
                )
            })
        {
            i.push(Interval::DiminishedFifth);
        }
        if !i.contains(&Interval::DiminishedSeventh) {
            i.push(Interval::DiminishedSeventh);
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HalfDimExp;
impl HalfDimExp {
    pub fn execute(&self, i: &mut Vec<Interval>, exp: &[Exp]) {
        if !i.contains(&Interval::MinorThird)
            && !exp.iter().any(|e| {
                matches!(
                    e,
                    Exp::Omit(OmitExp {
                        interval: Interval::MajorThird,
                        ..
                    })
                )
            })
            && !exp.iter().any(|e| matches!(e, Exp::Sus(SusExp { .. })))
        {
            i.push(Interval::MinorThird);
        }
        if !i.contains(&Interval::DiminishedFifth)
            && !exp.iter().any(|e| {
                matches!(
                    e,
                    Exp::Omit(OmitExp {
                        interval: Interval::PerfectFifth,
                        ..
                    })
                )
            })
        {
            i.push(Interval::DiminishedFifth);
        }
        if !i.contains(&Interval::MinorSeventh) {
            i.push(Interval::MinorSeventh);
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MajExp;
impl MajExp {
    pub fn execute(&self, i: &mut Vec<Interval>, exp: &[Exp]) {
        if exp.iter().any(|e| {
            matches!(
                e,
                Exp::Extension(ExtensionExp {
                    interval: Interval::MinorSeventh,
                    ..
                }) | Exp::Extension(ExtensionExp {
                    interval: Interval::Ninth,
                    ..
                }) | Exp::Extension(ExtensionExp {
                    interval: Interval::Eleventh,
                    ..
                }) | Exp::Extension(ExtensionExp {
                    interval: Interval::Thirteenth,
                    ..
                })
            )
        }) && !i.contains(&Interval::MajorSeventh)
        {
            i.push(Interval::MajorSeventh);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Maj7Exp;
impl Maj7Exp {
    pub fn execute(&self, i: &mut Vec<Interval>) {
        if !i.contains(&Interval::MajorSeventh) {
            i.push(Interval::MajorSeventh);
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MinorExp;
impl MinorExp {
    pub fn execute(&self, i: &mut Vec<Interval>, exp: &[Exp]) {
        if !i.contains(&Interval::MinorThird)
            && !exp.iter().any(|e| {
                matches!(
                    e,
                    Exp::Omit(OmitExp {
                        interval: Interval::MajorThird,
                        ..
                    })
                )
            })
            && !exp.iter().any(|e| matches!(e, Exp::Sus(SusExp { .. })))
        {
            i.push(Interval::MinorThird);
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AugExp;
impl AugExp {
    pub fn execute(&self, i: &mut Vec<Interval>, exp: &[Exp]) {
        if !i.contains(&Interval::AugmentedFifth)
            && !exp.iter().any(|e| {
                matches!(
                    e,
                    Exp::Omit(OmitExp {
                        interval: Interval::PerfectFifth,
                        ..
                    })
                )
            })
        {
            i.push(Interval::AugmentedFifth);
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AltExp;
impl AltExp {
    pub fn execute(&self, i: &mut Vec<Interval>) {
        i.push(Interval::MinorSeventh);
        i.push(Interval::FlatNinth);
        i.push(Interval::SharpNinth);
        i.push(Interval::SharpEleventh);
        i.push(Interval::FlatThirteenth);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PowerExp;
impl PowerExp {
    pub fn execute(&self, i: &mut Vec<Interval>) {
        i.push(Interval::PerfectFifth);
    }
}
