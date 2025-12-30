use super::expression::Exp;
use crate::chord::{
    interval::{Interval, IntervalSet},
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
        fn apply_dim(intervals: &mut IntervalSet) {
            intervals.remove_then_add(Interval::MajorThird, Interval::MinorThird);
            intervals.remove_then_add(Interval::PerfectFifth, Interval::DiminishedFifth);
        }
        match self {
            BaseForm::Major => {}
            BaseForm::Power => intervals.remove(Interval::MajorThird),
            BaseForm::Minor => {
                intervals.remove_then_add(Interval::MajorThird, Interval::MinorThird)
            }
            BaseForm::Dim => apply_dim(intervals),
            BaseForm::HalfDim => {
                apply_dim(intervals);
                intervals.insert(Interval::MinorSeventh);
            }
            BaseForm::Dim7 => {
                apply_dim(intervals);
                intervals.insert(Interval::DiminishedSeventh);
            }
        }
    }

    /// Determines the resulting `BaseForm` when transitioning from `self` to `other`
    /// according to the **chord base hierarchy**.
    ///
    /// Since the parser yields tokens in any specific order, and we want to parse anything,
    /// it could happen that an expression triggers a `baseForm` change when there is already one
    /// with higher priority. e.g.: Cdim-9. In this case, the chord is C-9(b5), not Cmin9.
    pub(crate) fn transition(&self, other: BaseForm) -> BaseForm {
        use BaseForm::*;

        fn priority(form: &BaseForm) -> u8 {
            match form {
                Power => 5,
                Dim7 => 4,
                Dim => 3,
                Minor => 2,
                HalfDim => 1,
                Major => 0,
            }
        }

        if priority(self) >= priority(&other) {
            self.clone()
        } else {
            other
        }
    }
}
