use super::expressions::{
    AddExp, AltExp, AugExp, BassExp, Dim7Exp, DimExp, ExtensionExp, HalfDimExp, MajExp, MinorExp,
    OmitExp, PowerExp, SlashBassExp, SusExp,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Exp {
    Extension(ExtensionExp),
    Add(AddExp),
    Sus(SusExp),
    Omit(OmitExp),
    SlashBass(SlashBassExp),
    Bass(BassExp),
    Alt(AltExp),
    Minor(MinorExp),
    Aug(AugExp),
    HalfDim(HalfDimExp),
    Dim(DimExp),
    Dim7(Dim7Exp),
    Maj(MajExp),
    Power(PowerExp),
}

impl Exp {
    pub fn validate(&self) -> bool {
        match self {
            Exp::Omit(exp) => exp.isvalid(),
            Exp::Add(exp) => exp.isvalid(),
            _ => true,
        }
    }

    pub fn priority(&self) -> u32 {
        match self {
            Exp::Power(_) => 0,
            Exp::Alt(_) => 1,
            Exp::Bass(_) => 2,
            Exp::Dim7(_) => 3,
            Exp::Dim(_) => 4,
            Exp::HalfDim(_) => 5,
            Exp::Minor(_) => 6,
            Exp::Sus(_) => 7,
            Exp::Maj(_) => 8,
            Exp::Extension(_) => 9,
            Exp::Add(_) => 10,
            Exp::Aug(_) => 11,
            Exp::Omit(_) => 12,
            Exp::SlashBass(_) => 13,
        }
    }
}
