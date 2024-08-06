use std::collections::HashMap;

use crate::chord::{
    intervals::Interval,
    note::{Note, NoteLiteral},
};

use super::expression::Exp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast {
    pub root: Note,
    pub bass: Option<Note>,
    pub expressions: Vec<Exp>,
    pub intervals: Vec<Interval>,
}

impl Ast {
    pub fn intervals(&mut self) {
        for exp in &self.expressions {
            match exp {
                Exp::Minor(min) => min.execute(&mut self.intervals),
                Exp::Dim7(dim) => dim.execute(&mut self.intervals),
                Exp::Dim(dim) => dim.execute(&mut self.intervals),
                Exp::HalfDim(half) => half.execute(&mut self.intervals),
                Exp::Sus(sus) => sus.execute(&mut self.intervals),
                Exp::Maj(maj) => maj.execute(&mut self.intervals, &self.expressions),
                Exp::Extension(ext) => ext.execute(&mut self.intervals),
                Exp::Add(add) => add.execute(&mut self.intervals),
                _ => (),
            }
        }
        dbg!(&self.intervals);
    }

    pub fn is_valid(&mut self) -> bool {
        self.expressions.sort_by_key(|exp| exp.priority());
        if !self.validate_expressions() {
            return false;
        }
        if !self.validate_extensions() {
            return false;
        }
        true
    }

    fn validate_extensions(&self) -> bool {
        let mut ext_count = [0; 24];
        let filtered = self
            .expressions
            .iter()
            .filter(|exp| matches!(exp, Exp::Extension(_)));
        for ext in filtered {
            if let Exp::Extension(ext) = ext {
                let index = ext.interval.st() as usize;
                match ext.interval {
                    Interval::MinorSecond
                    | Interval::MajorSecond
                    | Interval::MinorThird
                    | Interval::MajorThird
                    | Interval::DiminishedSeventh
                    | Interval::MajorSeventh => {
                        dbg!("Invalid extension");
                        return false;
                    }
                    _ => (),
                }
                if ext_count[index] > 0 {
                    dbg!("Duplicate extensions");
                    return false;
                }
                ext_count[index] += 1;
            }
        }
        true
    }

    fn validate_expressions(&self) -> bool {
        let mut is_valid = true;
        let mut counts: HashMap<u32, usize> = HashMap::new();
        for exp in &self.expressions {
            is_valid = exp.validate();
            if !is_valid {
                return false;
            }
            let key = match exp {
                Exp::Extension(_) | Exp::Add(_) | Exp::Sus(_) | Exp::Omit(_) => u32::MAX,
                _ => exp.priority(),
            };
            *counts.entry(key).or_insert(0) += 1;
        }

        for (key, count) in counts {
            if key < u32::MAX && count > 1 {
                dbg!("Duplicate modifiers");
                return false;
            }
        }
        is_valid
    }
}

impl Default for Ast {
    fn default() -> Ast {
        Ast {
            root: Note::new(NoteLiteral::C, None),
            bass: None,
            expressions: Vec::new(),
            intervals: Vec::new(),
        }
    }
}
