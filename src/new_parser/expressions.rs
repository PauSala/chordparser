use crate::chord::{intervals::Interval, note::Note};
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
}
impl OmitExp {
    pub fn new(interval: Interval) -> Self {
        Self { interval }
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
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HalfDimExp;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MajExp;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MinorExp;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AugExp;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AltExp;
