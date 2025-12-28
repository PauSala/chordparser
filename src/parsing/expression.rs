use super::expressions::{AddExp, AltExp, AugExp, ExtensionExp, OmitExp, SlashBassExp, SusExp};
use crate::{
    chord::intervals::Interval,
    parsing::ast::{Ast, BaseForm},
};
use std::fmt::{Display, Formatter};

pub(crate) trait Expression {
    fn pass(&self, ast: &mut Ast);
}

#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Exp {
    Power,
    Alt(AltExp),
    Bass,
    Minor,
    Dim7,
    Dim,
    HalfDim,
    Sus(SusExp),
    Maj,
    Maj7,
    Extension(ExtensionExp),
    Add(AddExp),
    Aug(AugExp),
    Omit(OmitExp),
    SlashBass(SlashBassExp),
}

impl Exp {
    pub(crate) fn pass(&self, ast: &mut Ast) {
        self.set_third(ast);
        match self {
            Exp::Power => ast.base_form = BaseForm::Power,
            Exp::Alt(exp) => exp.pass(ast),
            Exp::Bass => {
                ast.interval_set.remove(Interval::PerfectFifth);
                ast.interval_set.remove(Interval::MajorThird);
                ast.third = None;
            }
            Exp::Minor => ast.base_form = BaseForm::Minor,
            Exp::Dim7 => ast.base_form = BaseForm::Dim7,
            Exp::Dim => ast.base_form = BaseForm::Dim,
            Exp::HalfDim => {
                ast.base_form = BaseForm::HalfDim;
                ast.seventh = Some(Interval::MinorSeventh);
            }
            Exp::Sus(exp) => exp.pass(ast),
            Exp::Maj => {}
            Exp::Maj7 => ast.seventh = Some(Interval::MajorSeventh),
            Exp::Extension(exp) => exp.pass(ast),
            Exp::Add(exp) => exp.pass(ast),
            Exp::Aug(exp) => exp.pass(ast),
            Exp::Omit(exp) => exp.pass(ast),
            Exp::SlashBass(exp) => exp.pass(ast),
        }
    }

    /// The third is stored to not lose information about the quality for the normalization step,   
    /// which could happen for sus or omits.
    pub(crate) fn set_third(&self, ast: &mut Ast) {
        match self {
            Exp::Minor | Exp::Dim | Exp::Dim7 | Exp::HalfDim => {
                ast.third = Some(Interval::MinorThird)
            }
            Exp::Power | Exp::Bass => ast.third = None,
            _ => {}
        }
    }

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
            Exp::Alt(_) => "Alt".to_string(),
            Exp::Minor => "Minor".to_string(),
            Exp::Aug(_) => "Aug".to_string(),
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
            Exp::Alt(_) => 1,
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
            Exp::Aug(_) => 12,
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
