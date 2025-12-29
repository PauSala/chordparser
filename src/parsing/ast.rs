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
    pub(crate) fn apply(&self, intervals: &mut IntervalSet) {
        match self {
            BaseForm::Major => {}
            BaseForm::Power => {
                intervals.remove(Interval::MajorThird);
            }
            BaseForm::Minor => {
                intervals.remove_then_add(Interval::MajorThird, Interval::MinorThird);
                if intervals.contains(Interval::PerfectFifth)
                    && intervals.contains(Interval::DiminishedSeventh)
                {
                    intervals.remove_then_add(Interval::DiminishedSeventh, Interval::MajorSixth);
                }
            }
            BaseForm::Dim | BaseForm::HalfDim | BaseForm::Dim7 => {
                intervals.remove_then_add(Interval::MajorThird, Interval::MinorThird);
                intervals.remove_then_add(Interval::PerfectFifth, Interval::DiminishedFifth);
            }
        }
    }
}
