use crate::{
    chord::interval::{Interval, IntervalSet},
    parsing::{expression::Exp, parser_error::ParserError},
};
use Interval::*;
use std::collections::HashMap;
const INCOMPAT_SET: &[(Interval, IntervalSet)] = &[
    (
        MajorThird,
        IntervalSet::from_array([MajorThird, MinorThird]),
    ),
    (
        PerfectFifth,
        IntervalSet::from_array([PerfectFifth, DiminishedFifth]),
    ),
    (
        PerfectFifth,
        IntervalSet::from_array([PerfectFifth, AugmentedFifth]),
    ),
    (
        MajorSixth,
        IntervalSet::from_array([MajorSixth, MinorSixth]),
    ),
    (Ninth, IntervalSet::from_array([Ninth, FlatNinth])),
    (Ninth, IntervalSet::from_array([Ninth, SharpNinth])),
    (Eleventh, IntervalSet::from_array([Eleventh, SharpEleventh])),
    (
        Thirteenth,
        IntervalSet::from_array([Thirteenth, FlatThirteenth]),
    ),
];

pub(crate) struct Validator;
impl Validator {
    /// Analizes expressions and intervals finding inconsistencies.  
    /// If any inconcistence is found, self.errors is populated and false is returned.
    pub fn validate(
        &self,
        expressions: &Vec<Exp>,
        errors: &mut Vec<ParserError>,
        intervals: &[Interval],
    ) -> bool {
        let valid_exp = self.validate_expressions(expressions, errors);
        let valid_ext = self.validate_extensions(expressions, errors, intervals);
        let valid_sem = self.validate_semitones(errors, intervals);
        valid_exp && valid_ext && valid_sem && errors.is_empty()
    }

    /// Validates expressions both individually and finding illegal duplicates
    fn validate_expressions(&self, expressions: &Vec<Exp>, errors: &mut Vec<ParserError>) -> bool {
        let mut is_valid = true;
        let mut target_pos;
        let mut counts: HashMap<u32, usize> = HashMap::new();
        for exp in expressions {
            (is_valid, target_pos) = exp.validate();
            if !is_valid {
                errors.push(ParserError::WrongExpressionTarget(target_pos));
                return false;
            }
            let key = match exp {
                Exp::Extension(_) | Exp::Add(_) | Exp::Omit(_) => u32::MAX,
                _ => exp.priority(),
            };
            *counts.entry(key).or_insert(0) += 1;
        }

        for (key, count) in counts {
            if key < u32::MAX && count > 1 {
                errors.push(ParserError::DuplicateModifier(
                    Exp::from_priority(key).to_string(),
                ));
                return false;
            }
        }
        is_valid
    }

    /// Validates extensions finding for duplicates and incosistencies.
    fn validate_extensions(
        &self,
        expressions: &[Exp],
        errors: &mut Vec<ParserError>,
        intervals: &[Interval],
    ) -> bool {
        let mut ext_count = [0; 24];
        let filtered = expressions
            .iter()
            .filter(|exp| matches!(exp, Exp::Extension(_)));
        for ext in filtered {
            if let Exp::Extension(ext) = ext {
                let index = ext.interval.st() as usize;
                match ext.interval {
                    MinorSecond | MajorSecond | MinorThird | MajorThird | DiminishedSeventh
                    | MajorSeventh => {
                        errors.push(ParserError::InvalidExtension(ext.pos));
                        return false;
                    }
                    _ => (),
                }
                if ext_count[index] > 0 {
                    errors.push(ParserError::DuplicateExtension(ext.pos));
                    return false;
                }
                ext_count[index] += 1;
            }
        }
        !self.has_inconsistent_extensions(errors, intervals)
    }

    /// Finds illegal extensions combinations (for example 9 and b9/#9)
    fn has_inconsistent_extensions(
        &self,
        errors: &mut Vec<ParserError>,
        intervals: &[Interval],
    ) -> bool {
        let chord_set: IntervalSet = intervals
            .iter()
            .copied()
            .fold(IntervalSet::default(), |acc, s| acc | s);

        INCOMPAT_SET
            .iter()
            .filter(|(_, set)| set.is_subset_of(&chord_set))
            .map(|(int, _)| {
                errors.push(ParserError::InconsistentExtension(*int));
            })
            .count()
            > 0
    }

    /// Checks if there are any three consecutive semitones, which are illegal.
    fn validate_semitones(
        &self,
        errors: &mut Vec<ParserError>,
        input_intervals: &[Interval],
    ) -> bool {
        let mut is_valid = true;
        let mut count = 0u16;
        let mut intervals = [None; 12];

        for s in input_intervals {
            let pos = s.st() % 12;
            count |= 1 << pos;
            intervals[pos as usize] = Some(s);
        }

        for i in 0..12 {
            let a = (i + 1) % 12;
            let b = (i + 2) % 12;
            if (count & (1 << i) != 0) && (count & (1 << a) != 0) && (count & (1 << b) != 0) {
                is_valid = false;
                errors.push(ParserError::ThreeConsecutiveSemitones(vec![
                    format!("{}", intervals[i].unwrap()),
                    format!("{}", intervals[a].unwrap()),
                    format!("{}", intervals[b].unwrap()),
                ]));
            }
        }

        is_valid
    }
}
