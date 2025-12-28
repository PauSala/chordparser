use super::expression::Exp;
use crate::chord::{
    intervals::{Interval, IntervalSet},
    note::{Note, NoteLiteral},
};
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast {
    pub(crate) root: Note,
    pub(crate) expressions: Vec<Exp>,
}

impl Ast {}

impl Default for Ast {
    fn default() -> Ast {
        Ast {
            root: Note::new(NoteLiteral::C, None),
            expressions: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[repr(u8)]
pub(crate) enum BaseForm {
    #[default]
    Major,
    Minor,
    Dim,
    HalfDim,
    Dim7,
    Power,
}

impl BaseForm {
    /// Mutates `intervals` adding or removing thirds and fifths
    pub(crate) fn update_triad(&self, intervals: &mut IntervalSet) {
        match self {
            BaseForm::Major => {}
            BaseForm::Power => {
                intervals.remove(Interval::MajorThird);
            }
            BaseForm::Minor => {
                intervals.replace(Interval::MajorThird, Interval::MinorThird);
            }
            BaseForm::Dim | BaseForm::HalfDim | BaseForm::Dim7 => {
                intervals.replace(Interval::MajorThird, Interval::MinorThird);
                intervals.replace(Interval::PerfectFifth, Interval::DiminishedFifth);
            }
        }
    }
}
