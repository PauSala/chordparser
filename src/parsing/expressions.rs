use crate::{
    chord::{intervals::Interval, note::Note},
    parsing::expression::Expression,
};

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

impl Expression for ExtensionExp {
    fn pass(&self, ast: &mut super::ast::Ast) {
        match self.interval {
            Interval::PerfectFourth
            | Interval::AugmentedFourth
            | Interval::DiminishedFifth
            | Interval::AugmentedFifth
            | Interval::FlatNinth
            | Interval::SharpNinth
            | Interval::SharpEleventh
            | Interval::FlatThirteenth => ast.alts.push(self.interval),
            Interval::MajorSixth | Interval::MinorSixth => ast.sixth = Some(self.interval),
            Interval::MinorSeventh => {
                ast.seventh = Some(Interval::MinorSeventh);
                ast.alts.push(self.interval);
            }
            Interval::Ninth | Interval::Eleventh | Interval::Thirteenth => {
                ast.extension_cap = Some(
                    self.interval
                        .max(ast.extension_cap.unwrap_or(Interval::Unison)),
                )
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AddExp {
    pub interval: Interval,
    pub target_pos: usize,
}

impl Expression for AddExp {
    fn pass(&self, ast: &mut super::ast::Ast) {
        ast.adds.push(self.interval);
    }
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

impl Expression for SusExp {
    fn pass(&self, ast: &mut super::ast::Ast) {
        ast.omits.push(Interval::MajorThird);
        match self.interval {
            Interval::PerfectFourth => {
                ast.is_sus = true;
                ast.sus = Some(self.interval);
            }
            Interval::AugmentedFourth => ast.alts.push(Interval::SharpEleventh),
            Interval::MinorSecond => ast.alts.push(Interval::FlatNinth),
            Interval::MajorSecond => ast.alts.push(Interval::Ninth),
            _ => {}
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
    pub target_pos: usize,
}

impl Expression for OmitExp {
    fn pass(&self, ast: &mut super::ast::Ast) {
        ast.omits.push(self.interval);
    }
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

impl Expression for SlashBassExp {
    fn pass(&self, ast: &mut super::ast::Ast) {
        ast.bass = Some(self.note)
    }
}

impl SlashBassExp {
    pub fn new(note: Note) -> Self {
        Self { note }
    }
}

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub struct Maj7Exp;
// impl Expression for Maj7Exp {
//     fn pass(&self, ast: &mut super::ast::Ast) {
//         ast.seventh = Some(Interval::MajorSeventh);
//     }
// }

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AugExp;
impl Expression for AugExp {
    fn pass(&self, ast: &mut super::ast::Ast) {
        ast.alts.push(Interval::AugmentedFifth);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AltExp;
impl Expression for AltExp {
    fn pass(&self, ast: &mut super::ast::Ast) {
        ast.omits.push(Interval::PerfectFifth);
        ast.seventh = Some(Interval::MinorSeventh);
        ast.alts.push(Interval::FlatNinth);
        ast.alts.push(Interval::SharpNinth);
        ast.alts.push(Interval::SharpEleventh);
        ast.alts.push(Interval::FlatThirteenth);
    }
}
