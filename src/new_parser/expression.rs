use crate::chord::note::{Note, NoteLiteral};

use super::expressions::{
    AddExp, AltExp, AugExp, BassExp, Dim7Exp, DimExp, ExtensionExp, HalfDimExp, MajExp, MinorExp,
    OmitExp, SlashBassExp, SusExp,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expresssion {
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
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast {
    pub root: Note,
    pub bass: Option<Note>,
    pub expressions: Vec<Expresssion>,
}

impl Default for Ast {
    fn default() -> Ast {
        Ast {
            root: Note::new(NoteLiteral::C, None),
            bass: None,
            expressions: Vec::new(),
        }
    }
}
