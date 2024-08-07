use std::collections::HashMap;

use crate::{
    chord::{
        intervals::Interval,
        note::{Note, NoteLiteral},
        Chord,
    },
    new_parser::expressions::{OmitExp, PowerExp},
};

use super::expression::Exp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast {
    pub root: Note,
    pub bass: Option<Note>,
    pub expressions: Vec<Exp>,
    pub intervals: Vec<Interval>,
    pub is_sus: bool,
}

impl Ast {
    pub fn intervals(&mut self) {
        for exp in &self.expressions {
            match exp {
                Exp::Minor(min) => min.execute(&mut self.intervals),
                Exp::Dim7(dim) => dim.execute(&mut self.intervals),
                Exp::Dim(dim) => dim.execute(&mut self.intervals),
                Exp::HalfDim(half) => half.execute(&mut self.intervals),
                Exp::Sus(sus) => {
                    sus.execute(&mut self.intervals);
                    self.is_sus = true;
                }
                Exp::Maj(maj) => maj.execute(&mut self.intervals, &self.expressions),
                Exp::Extension(ext) => ext.execute(&mut self.intervals),
                Exp::Add(add) => add.execute(&mut self.intervals),
                _ => (),
            }
        }
        self.add_third();
        self.add_five();
        self.intervals.sort_by_key(|i| i.st());
    }

    /// Get the notes of the chord
    pub(crate) fn get_notes(&mut self) -> Vec<Note> {
        let mut notes = Vec::new();
        for n in &self.intervals {
            let note = self
                .root
                .get_note(n.st(), n.to_semantic_interval().numeric());
            notes.push(note);
        }
        notes
    }

    pub fn get_descriptor(&mut self, name: &str) -> String {
        let modifier_str = match &self.root.modifier {
            Some(m) => m.to_string(),
            None => "".to_string(),
        };
        name.replace(
            &format!("{}{}", self.root.literal.to_string(), modifier_str),
            "",
        )
    }

    pub fn to_chord(&mut self, name: &str) -> Chord {
        self.intervals();
        let notes = self.get_notes();
        let mut semitones = Vec::new();
        let mut semantic_intervals = Vec::new();
        let note_literals = notes.iter().map(|a| a.to_string()).collect::<Vec<String>>();
        for e in &self.intervals {
            semitones.push(e.st());
            semantic_intervals.push(e.to_semantic_interval().numeric());
        }
        Chord::builder(name, self.root.clone())
            .descriptor(&self.get_descriptor(name))
            .bass(self.bass.clone())
            .notes(notes)
            .note_literals(note_literals)
            .semitones(semitones)
            .semantic_intervals(semantic_intervals)
            .real_intervals(self.intervals.clone())
            .is_sus(self.is_sus)
            .adds(vec![])
            .build()
    }

    fn add_third(&mut self) {
        if !self.intervals.contains(&Interval::MinorThird)
            && !self.is_sus
            && !self.expressions.iter().any(|exp| {
                matches!(
                    exp,
                    Exp::Omit(OmitExp {
                        interval: Interval::MajorThird
                    }) | Exp::Power(PowerExp)
                )
            })
        {
            self.intervals.push(Interval::MajorThird);
        }
    }

    pub fn add_five(&mut self) {
        if !self.intervals.contains(&Interval::DiminishedFifth)
            && !self.intervals.contains(&Interval::AugmentedFifth)
            && !self.intervals.contains(&Interval::FlatThirteenth)
            && !self.expressions.iter().any(|exp| {
                matches!(
                    exp,
                    Exp::Omit(OmitExp {
                        interval: Interval::PerfectFifth
                    })
                )
            })
        {
            self.intervals.push(Interval::PerfectFifth);
        }
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
            intervals: vec![Interval::Unison],
            is_sus: false,
        }
    }
}
