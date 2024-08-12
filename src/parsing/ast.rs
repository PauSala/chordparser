use std::collections::HashMap;

use crate::{
    chord::{
        intervals::Interval,
        note::{Note, NoteLiteral},
        Chord,
    },
    parsing::{
        expressions::{BassExp, OmitExp, PowerExp},
        parser_error::ParserErrors,
    },
};

use super::expression::Exp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast {
    pub root: Note,
    pub bass: Option<Note>,
    pub expressions: Vec<Exp>,
    pub intervals: Vec<Interval>,
    pub is_sus: bool,
    pub errors: Vec<String>,
}

impl Ast {
    pub fn set_intervals(&mut self) {
        self.expressions.sort();
        for exp in &self.expressions {
            match exp {
                Exp::Minor(min) => min.execute(&mut self.intervals, &self.expressions),
                Exp::Dim7(dim) => dim.execute(&mut self.intervals),
                Exp::Dim(dim) => dim.execute(&mut self.intervals),
                Exp::HalfDim(half) => half.execute(&mut self.intervals),
                Exp::Sus(sus) => {
                    sus.execute(&mut self.intervals);
                    self.is_sus = true;
                }
                Exp::Maj(maj) => maj.execute(&mut self.intervals, &self.expressions),
                Exp::Extension(ext) => ext.execute(&mut self.intervals, &mut self.is_sus),
                Exp::Add(add) => add.execute(&mut self.intervals),
                Exp::Aug(aug) => aug.execute(&mut self.intervals),
                Exp::SlashBass(bass) => self.bass = Some(bass.note.clone()),
                Exp::Alt(alt) => alt.execute(&mut self.intervals),
                Exp::Power(pw) => pw.execute(&mut self.intervals),
                Exp::Bass(_) => (),
                _ => (),
            }
        }
        self.add_third();
        self.add_five();
        self.intervals.sort_by_key(|i| i.st());
    }

    fn add_third(&mut self) {
        if !self.intervals.contains(&Interval::MajorThird)
            && !self.intervals.contains(&Interval::MinorThird)
            && !self.is_sus
            && !self.expressions.iter().any(|exp| {
                matches!(
                    exp,
                    Exp::Omit(OmitExp {
                        interval: Interval::MajorThird
                    }) | Exp::Power(PowerExp)
                        | Exp::Bass(BassExp)
                )
            })
        {
            self.intervals.push(Interval::MajorThird);
        }
    }

    pub fn add_five(&mut self) {
        if !self.intervals.contains(&Interval::DiminishedFifth)
            && !self.intervals.contains(&Interval::PerfectFifth)
            && !self.intervals.contains(&Interval::AugmentedFifth)
            && !self.intervals.contains(&Interval::FlatThirteenth)
            && !self.expressions.iter().any(|exp| {
                matches!(
                    exp,
                    Exp::Omit(OmitExp {
                        interval: Interval::PerfectFifth
                    }) | Exp::Bass(BassExp)
                )
            })
        {
            self.intervals.push(Interval::PerfectFifth);
        }
    }

    /// Checks if there are any three consecutive semitones, which are illegal.
    pub fn validate_semitones(&mut self) -> bool {
        let mut is_valid = true;
        let mut count = [false; 12];
        let mut map = HashMap::new();
        for s in self.intervals.iter() {
            let pos = s.st() % 12;
            count[pos as usize] = true;
            map.insert(pos, s);
        }
        for i in 0..12 {
            let a = (i + 1) % 12;
            let b = (i + 2) % 12;
            if count[i] && count[a] && count[b] {
                is_valid = false;
                self.errors.push(format!(
                    "Three consecutive semitones: {}, {}, {}",
                    map.get(&(i as u8)).unwrap().to_string(),
                    map.get(&(a as u8)).unwrap().to_string(),
                    map.get(&(b as u8)).unwrap().to_string(),
                ))
            }
        }
        is_valid
    }

    fn has_inconsistent_extension(&self, int: &Interval, matches: Vec<&Interval>) -> bool {
        for i in matches {
            if self.intervals.contains(i) && self.intervals.contains(int) {
                return true;
            }
        }
        false
    }

    /// Finds illegal extensions combinations (for example 9 and b9/#9)
    fn has_inconsistent_extensions(&mut self) -> bool {
        if self.has_inconsistent_extension(
            &Interval::Ninth,
            vec![&Interval::FlatNinth, &Interval::SharpNinth],
        ) || self.has_inconsistent_extension(&Interval::Eleventh, vec![&Interval::SharpEleventh])
            || self
                .has_inconsistent_extension(&Interval::Thirteenth, vec![&Interval::FlatThirteenth])
            || self.has_inconsistent_extension(&Interval::MajorSixth, vec![&Interval::MinorSixth])
            || self.has_inconsistent_extension(&Interval::MajorThird, vec![&Interval::MinorThird])
        {
            self.errors.push("Inconsistent extensions".to_string());
            return true;
        }
        false
    }

    /// Validates extensions finding for duplicates and incosistencies.
    fn validate_extensions(&mut self) -> bool {
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
                        self.errors
                            .push(format!("Invalid extension {}", ext.interval));
                        return false;
                    }
                    _ => (),
                }
                if ext_count[index] > 0 {
                    self.errors.push("Duplicate extensions".to_string());
                    return false;
                }
                ext_count[index] += 1;
            }
        }
        !self.has_inconsistent_extensions()
    }

    /// Validates expressions both individually and finding illegal duplicates
    fn validate_expressions(&mut self) -> bool {
        let mut is_valid = true;
        let mut counts: HashMap<u32, usize> = HashMap::new();
        for exp in &self.expressions {
            is_valid = exp.validate();
            if !is_valid {
                self.errors.push(format!("Invalid expression {}", exp));
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
                self.errors
                    .push(format!("Duplicate '{}' modifier", Exp::from_priority(key)));
                return false;
            }
        }
        is_valid
    }

    /// Analizes expressions and intervals finding inconsistencies.  
    /// If any inconcistence is found, self.errors is populated and false is returned.
    pub fn is_valid(&mut self) -> bool {
        let valid_exp = self.validate_expressions();
        let valid_ext = self.validate_extensions();
        let valid_sem = self.validate_semitones();
        valid_exp && valid_ext && valid_sem
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
        name.replace(&format!("{}{}", self.root.literal, modifier_str), "")
    }

    pub fn build_chord(&mut self, name: &str) -> Result<Chord, ParserErrors> {
        self.set_intervals();
        let notes = self.get_notes();
        let mut semitones = Vec::new();
        let mut semantic_intervals = Vec::new();
        let note_literals = notes.iter().map(|a| a.to_string()).collect::<Vec<String>>();
        for e in &self.intervals {
            semitones.push(e.st());
            semantic_intervals.push(e.to_semantic_interval().numeric());
        }

        if !self.is_valid() {
            return Err(ParserErrors::new(self.errors.clone()));
        }
        Ok(Chord::builder(name, self.root.clone())
            .descriptor(&self.get_descriptor(name))
            .bass(self.bass.clone())
            .notes(notes)
            .note_literals(note_literals)
            .semitones(semitones)
            .semantic_intervals(semantic_intervals)
            .real_intervals(self.intervals.clone())
            .is_sus(self.is_sus)
            .adds(vec![])
            .build())
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
            errors: Vec::new(),
        }
    }
}
