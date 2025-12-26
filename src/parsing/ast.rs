use std::{
    collections::{HashMap, HashSet},
    mem,
};

use crate::{
    chord::{
        Chord,
        intervals::Interval,
        note::{Note, NoteLiteral},
    },
    parsing::parser_error::ParserErrors,
};

use super::{expression::Exp, parser_error::ParserError};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[repr(u8)]
pub enum Quality {
    #[default]
    Major,
    Minor,
    Dim,
    HalfDim,
    Dim7,
    Power,
}

impl Quality {
    fn build(&self, intervals: &mut HashSet<Interval>) {
        match self {
            Quality::Major => {}
            Quality::Minor => {
                intervals.remove(&Interval::MajorThird);
                intervals.insert(Interval::MinorThird);
            }
            Quality::Dim => {
                intervals.remove(&Interval::MajorThird);
                intervals.remove(&Interval::PerfectFifth);
                intervals.insert(Interval::MinorThird);
                intervals.insert(Interval::DiminishedFifth);
            }
            Quality::HalfDim => {
                intervals.remove(&Interval::MajorThird);
                intervals.remove(&Interval::PerfectFifth);
                intervals.insert(Interval::MinorThird);
                intervals.insert(Interval::DiminishedFifth);
                intervals.insert(Interval::MinorSeventh);
            }
            Quality::Dim7 => {
                intervals.remove(&Interval::MajorThird);
                intervals.remove(&Interval::PerfectFifth);
                intervals.insert(Interval::MinorThird);
                intervals.insert(Interval::DiminishedFifth);
                intervals.insert(Interval::DiminishedSeventh);
            }
            Quality::Power => {
                intervals.remove(&Interval::MajorThird);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast {
    pub(crate) root: Note,
    pub(crate) bass: Option<Note>,
    pub(crate) expressions: Vec<Exp>,
    pub(crate) norm_intervals: Vec<Interval>,
    pub(crate) intervals: Vec<Interval>,
    pub(crate) is_sus: bool,
    pub(crate) errors: Vec<ParserError>,

    pub(crate) quality: Quality,
    pub(crate) omits: Vec<Interval>,
    pub(crate) adds: Vec<Interval>,
    pub(crate) alts: Vec<Interval>,
    pub(crate) sus: Option<Interval>,
    pub(crate) sixth: Option<Interval>,
    pub(crate) seventh: Option<Interval>,
    pub(crate) extension_cap: Option<Interval>,
    pub(crate) interval_set: HashSet<Interval>,
}

impl Ast {
    fn interval_set(&mut self) {
        let expressions = mem::take(&mut self.expressions);

        if expressions.iter().any(|exp| matches!(exp, Exp::Bass(..))) {
            self.interval_set.remove(&Interval::PerfectFifth);
            self.interval_set.remove(&Interval::MajorThird);
            return;
        }

        for exp in &expressions {
            exp.pass(self);
        }
        self.expressions = expressions;

        // Set quality intervals
        self.quality.build(&mut self.interval_set);

        // Set seventh
        if let Some(seventh) = self.seventh {
            self.interval_set.insert(seventh);
        }

        // Set sixth
        if let Some(sixth) = self.sixth {
            self.interval_set.insert(sixth);
        }

        if let Some(sus) = self.sus {
            self.interval_set.remove(&Interval::MajorThird);
            self.interval_set.remove(&Interval::MinorThird);
            self.interval_set.insert(sus);
        }

        // Alts
        for alt in &self.alts {
            match alt {
                Interval::DiminishedFifth | Interval::AugmentedFifth | Interval::FlatThirteenth => {
                    self.interval_set.remove(&Interval::PerfectFifth);
                    self.interval_set.insert(*alt);
                }
                _ => {
                    self.interval_set.insert(*alt);
                }
            }
        }

        // Caps
        self.extension_caps();

        // Omits
        for omit in &self.omits {
            match omit {
                Interval::PerfectFifth => {
                    self.interval_set.remove(&Interval::PerfectFifth);
                    self.interval_set.remove(&Interval::AugmentedFifth);
                    self.interval_set.remove(&Interval::DiminishedFifth);
                }
                Interval::MajorThird => {
                    self.interval_set.remove(&Interval::MinorThird);
                    self.interval_set.remove(&Interval::MajorThird);
                }
                _ => {}
            }
        }

        // Adds
        for add in &self.adds {
            if *add == Interval::FlatThirteenth {
                self.interval_set.remove(&Interval::PerfectFifth);
            }
            dbg!(&add);
            self.interval_set.insert(*add);
        }
    }

    fn seventh(&self) -> Interval {
        if self
            .expressions
            .iter()
            .any(|exp| matches!(exp, Exp::Maj7(..) | Exp::Maj(..)))
        {
            Interval::MajorSeventh
        } else {
            Interval::MinorSeventh
        }
    }

    fn extension_caps(&mut self) {
        let conflict_map: HashMap<Interval, Vec<Interval>> = [
            (
                Interval::Ninth,
                vec![Interval::FlatNinth, Interval::SharpNinth],
            ),
            (Interval::Eleventh, vec![Interval::SharpEleventh]),
            (Interval::Thirteenth, vec![Interval::FlatThirteenth]),
            (Interval::MinorSeventh, vec![Interval::DiminishedSeventh]),
        ]
        .into_iter()
        .collect();

        let seventh = self.seventh();
        if let Some(cap) = self.extension_cap {
            if self.quality == Quality::Major && cap == Interval::Eleventh {
                self.interval_set.remove(&Interval::MajorThird);
                self.interval_set.insert(Interval::PerfectFourth);
            } else {
                self.interval_set.insert(cap);
            }

            let caps_to_add: Vec<Interval> = match self.quality {
                Quality::Major => match cap {
                    Interval::Thirteenth => vec![Interval::Ninth, seventh],
                    Interval::Eleventh => vec![Interval::Ninth, seventh],
                    Interval::Ninth => {
                        if let Some(_) = self.sixth {
                            vec![]
                        } else {
                            vec![seventh]
                        }
                    }
                    _ => vec![],
                },
                _ => match cap {
                    Interval::Thirteenth => vec![Interval::Eleventh, Interval::Ninth, seventh],
                    Interval::Eleventh => vec![Interval::Ninth, seventh],
                    Interval::Ninth => {
                        if let Some(_) = self.sixth {
                            vec![]
                        } else {
                            vec![seventh]
                        }
                    }
                    _ => vec![],
                },
            };

            for interval in caps_to_add {
                let conflicts = conflict_map.get(&interval).cloned().unwrap_or_default();

                let blocked = self.interval_set.contains(&interval)
                    || conflicts.iter().any(|c| self.interval_set.contains(c));

                if !blocked {
                    self.interval_set.insert(interval);
                }
            }
        }
    }

    fn set_intervals(&mut self) {
        self.norm_intervals = self.interval_set.iter().cloned().collect();
        self.norm_intervals.sort_by_key(|i| i.st());
        self.intervals = self.norm_intervals.clone();
        if let Some(Exp::Sus(sus_exp)) = self.expressions.iter().find(|e| matches!(e, Exp::Sus(_)))
        {
            self.intervals = self
                .intervals
                .iter()
                .map(|i| match (sus_exp.interval, i) {
                    (Interval::MinorSecond, Interval::FlatNinth) => Interval::MinorSecond,
                    (Interval::MajorSecond, Interval::Ninth) => Interval::MajorSecond,
                    (Interval::AugmentedFourth, Interval::SharpEleventh) => {
                        Interval::AugmentedFourth
                    }
                    _ => *i,
                })
                .collect();
            self.intervals.sort_by_key(|i| i.st());
        }
    }

    /// Checks if there are any three consecutive semitones, which are illegal.
    fn validate_semitones(&mut self) -> bool {
        let mut is_valid = true;
        let mut count = 0u16;
        let mut intervals = [None; 12];

        for s in self.norm_intervals.iter() {
            let pos = s.st() % 12;
            count |= 1 << pos;
            intervals[pos as usize] = Some(s);
        }

        for i in 0..12 {
            let a = (i + 1) % 12;
            let b = (i + 2) % 12;
            if (count & (1 << i) != 0) && (count & (1 << a) != 0) && (count & (1 << b) != 0) {
                is_valid = false;
                self.errors
                    .push(ParserError::ThreeConsecutiveSemitones(vec![
                        format!("{}", intervals[i].unwrap()),
                        format!("{}", intervals[a].unwrap()),
                        format!("{}", intervals[b].unwrap()),
                    ]));
            }
        }

        is_valid
    }

    fn has_inconsistent_extension(&self, int: &Interval, matches: Vec<&Interval>) -> bool {
        for i in matches {
            if self.norm_intervals.contains(i) && self.norm_intervals.contains(int) {
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
        ) {
            self.errors.push(ParserError::InconsistentExtension(
                Interval::Ninth.to_string(),
            ));
            return true;
        }
        if self.has_inconsistent_extension(&Interval::Eleventh, vec![&Interval::SharpEleventh]) {
            self.errors.push(ParserError::InconsistentExtension(
                Interval::Eleventh.to_string(),
            ));
            return true;
        }
        if self.has_inconsistent_extension(&Interval::Thirteenth, vec![&Interval::FlatThirteenth]) {
            self.errors.push(ParserError::InconsistentExtension(
                Interval::Thirteenth.to_string(),
            ));
            return true;
        }
        if self.has_inconsistent_extension(&Interval::MajorSixth, vec![&Interval::MinorSixth]) {
            self.errors.push(ParserError::InconsistentExtension(
                Interval::MajorSixth.to_string(),
            ));
            return true;
        }
        if self.has_inconsistent_extension(&Interval::MajorThird, vec![&Interval::MinorThird]) {
            self.errors.push(ParserError::InconsistentExtension(
                Interval::MajorThird.to_string(),
            ));
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
                        self.errors.push(ParserError::InvalidExtension(ext.pos));
                        return false;
                    }
                    _ => (),
                }
                if ext_count[index] > 0 {
                    self.errors.push(ParserError::DuplicateExtension(ext.pos));
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
        let mut target_pos;
        let mut counts: HashMap<u32, usize> = HashMap::new();
        for exp in &self.expressions {
            (is_valid, target_pos) = exp.validate();
            if !is_valid {
                self.errors
                    .push(ParserError::WrongExpressionTarget(target_pos));
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
                    .push(ParserError::DuplicateModifier(Exp::from_priority(key)));
                return false;
            }
        }
        is_valid
    }

    /// Analizes expressions and intervals finding inconsistencies.  
    /// If any inconcistence is found, self.errors is populated and false is returned.
    fn is_valid(&mut self) -> bool {
        let valid_exp = self.validate_expressions();
        let valid_ext = self.validate_extensions();
        let valid_sem = self.validate_semitones();
        valid_exp && valid_ext && valid_sem && self.errors.is_empty()
    }

    /// Get the notes of the chord
    fn notes(&mut self) -> Vec<Note> {
        let mut notes = Vec::new();
        for n in &self.intervals {
            let note = self
                .root
                .get_note(n.st(), n.to_semantic_interval().numeric());
            notes.push(note);
        }
        notes
    }

    pub fn descriptor(&mut self, name: &str) -> String {
        let modifier_str = match &self.root.modifier {
            Some(m) => m.to_string(),
            None => "".to_string(),
        };
        name.replace(&format!("{}{}", self.root.literal, modifier_str), "")
    }

    pub(crate) fn build_chord(&mut self, name: &str) -> Result<Chord, ParserErrors> {
        self.interval_set();
        self.set_intervals();

        let notes = self.notes();
        let mut semitones = Vec::new();
        let mut semantic_intervals = Vec::new();
        let note_literals = notes.iter().map(|a| a.to_string()).collect();

        let mut rbs = [false; 24];
        for e in &self.norm_intervals {
            let v = e.st();
            rbs[v as usize] = true;
            semantic_intervals.push(e.to_semantic_interval().numeric());
        }

        for e in &self.intervals {
            let v = e.st();
            semitones.push(v);
        }

        if !self.is_valid() {
            return Err(ParserErrors::new(self.errors.clone()));
        }

        Ok(Chord::builder(name, self.root.clone())
            .descriptor(&self.descriptor(name))
            .bass(self.bass.clone())
            .notes(notes)
            .note_literals(note_literals)
            .rbs(rbs)
            .semitones(semitones)
            .semantic_intervals(semantic_intervals)
            .normalized_intervals(self.norm_intervals.clone())
            .intervals(self.intervals.clone())
            .is_sus(self.is_sus)
            .build())
    }
}

impl Default for Ast {
    fn default() -> Ast {
        Ast {
            root: Note::new(NoteLiteral::C, None),
            bass: None,
            expressions: Vec::new(),
            norm_intervals: vec![Interval::Unison],
            intervals: vec![],
            is_sus: false,
            errors: Vec::new(),

            quality: Quality::Major,
            omits: Default::default(),
            adds: Default::default(),
            seventh: None,
            extension_cap: None,
            alts: Default::default(),
            sus: Default::default(),
            sixth: Default::default(),
            interval_set: vec![
                Interval::Unison,
                Interval::MajorThird,
                Interval::PerfectFifth,
            ]
            .into_iter()
            .collect(),
        }
    }
}
