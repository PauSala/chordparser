use std::fmt::{Display, Formatter};

use crate::parsing::{
    ast::{Ast, BaseForm},
    expressions::Maj7Exp,
};

use super::expressions::{
    AddExp, AltExp, AugExp, ExtensionExp, HalfDimExp, MajExp, MinorExp, OmitExp, PowerExp,
    SlashBassExp, SusExp,
};

pub(crate) trait Expression {
    fn pass(&self, ast: &mut Ast);
}

#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Exp {
    Power(PowerExp),
    Alt(AltExp),
    Bass,
    Minor(MinorExp),
    Dim7,
    Dim,
    HalfDim(HalfDimExp),
    Sus(SusExp),
    Maj(MajExp),
    Maj7(Maj7Exp),
    Extension(ExtensionExp),
    Add(AddExp),
    Aug(AugExp),
    Omit(OmitExp),
    SlashBass(SlashBassExp),
}

impl Exp {
    pub(crate) fn pass(&self, ast: &mut Ast) {
        match self {
            Exp::Power(exp) => exp.pass(ast),
            Exp::Alt(exp) => exp.pass(ast),
            Exp::Bass => {}
            Exp::Minor(exp) => exp.pass(ast),
            Exp::Dim7 => ast.base_form = BaseForm::Dim7,
            Exp::Dim => ast.base_form = BaseForm::Dim,
            Exp::HalfDim(exp) => exp.pass(ast),
            Exp::Sus(exp) => exp.pass(ast),
            Exp::Maj(exp) => exp.pass(ast),
            Exp::Maj7(exp) => exp.pass(ast),
            Exp::Extension(exp) => exp.pass(ast),
            Exp::Add(exp) => exp.pass(ast),
            Exp::Aug(exp) => exp.pass(ast),
            Exp::Omit(exp) => exp.pass(ast),
            Exp::SlashBass(exp) => exp.pass(ast),
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
            Exp::Minor(_) => "Minor".to_string(),
            Exp::Aug(_) => "Aug".to_string(),
            Exp::HalfDim(_) => "HalfDim".to_string(),
            Exp::Dim => "Dim".to_string(),
            Exp::Dim7 => "Dim7".to_string(),
            Exp::Maj(_) => "Maj".to_string(),
            Exp::Maj7(_) => "Maj".to_string(),
            Exp::Power(_) => "Power".to_string(),
        }
    }

    pub fn priority(&self) -> u32 {
        match self {
            Exp::Power(_) => 0,
            Exp::Alt(_) => 1,
            Exp::Bass => 2,
            Exp::Minor(_) => 3,
            Exp::Dim7 => 4,
            Exp::Dim => 5,
            Exp::HalfDim(_) => 6,
            Exp::Sus(_) => 7,
            Exp::Maj(_) => 8,
            Exp::Maj7(_) => 9,
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
