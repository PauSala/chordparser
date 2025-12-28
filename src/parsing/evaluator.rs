use std::{collections::HashMap, sync::LazyLock};

use crate::{
    chord::{
        Chord,
        intervals::{Interval, IntervalSet},
        note::Note,
    },
    parsing::{
        ast::{Ast, BaseForm},
        expression::Exp,
        expressions::{ExtensionExp, SusExp},
        parser_error::{ParserError, ParserErrors},
        validator::Validator,
    },
};

static CONFLICT_MAP: LazyLock<HashMap<Interval, Vec<Interval>>> = LazyLock::new(|| {
    HashMap::from([
        (
            Interval::Ninth,
            vec![Interval::FlatNinth, Interval::SharpNinth],
        ),
        (Interval::Eleventh, vec![Interval::SharpEleventh]),
        (Interval::Thirteenth, vec![Interval::FlatThirteenth]),
        (Interval::MinorSeventh, vec![Interval::DiminishedSeventh]),
    ])
});

#[derive(Debug, Default)]
pub(crate) struct Descriptor {
    pub(crate) bass: Option<Note>,
    pub(crate) intervals: Vec<Interval>,
    pub(crate) display_intervals: Vec<Interval>,
    pub(crate) base_form: BaseForm,
    pub(crate) omits: Vec<Interval>,
    pub(crate) adds: Vec<Interval>,
    pub(crate) alts: Vec<Interval>,
    pub(crate) sus: Option<Interval>,
    pub(crate) third: Option<Interval>,
    pub(crate) max_extension: Option<Interval>,
    pub(crate) interval_set: IntervalSet,
}

impl Descriptor {
    pub fn new() -> Self {
        Default::default()
    }
    pub(crate) fn insert_seventh(&mut self, seventh: Interval) {
        self.interval_set.insert(seventh);
    }
    pub(crate) fn insert_sixth(&mut self, sixth: Interval) {
        self.interval_set.insert(sixth);
    }
}

pub(crate) struct Evaluator<'a> {
    pub(crate) ast: &'a Ast,
    pub(crate) desc: Descriptor,
    pub(crate) is_valid: bool,
    pub(crate) errors: Vec<ParserError>,
    pub(crate) name: String,
}

impl<'a> Evaluator<'a> {
    pub fn evaluate(ast: &'a Ast, name: String) -> Result<Chord, ParserErrors> {
        Self {
            ast,
            desc: Descriptor::new(),
            is_valid: false,
            errors: vec![],
            name,
        }
        .seed()
        .evaluate_expressions()
        .update_triad()
        .apply_sus()
        .apply_alterations()
        .implied_extensions()
        .apply_omits()
        .apply_adds()
        .prune()
        .set_intervals()
        .validate()
        .build_chord()
    }

    fn seed(mut self) -> Self {
        self.desc.intervals = vec![Interval::Unison];
        self.desc.base_form = BaseForm::Major;
        self.desc.third = Some(Interval::MajorThird);
        self.desc.interval_set.insert(Interval::Unison);
        self.desc.interval_set.insert(Interval::MajorThird);
        self.desc.interval_set.insert(Interval::PerfectFifth);
        self
    }

    /// Evaluates ast's expressions.
    ///
    /// Each expression may mutate both the descriptor's interval set and it's baseForm.
    fn evaluate_expressions(mut self) -> Self {
        for expr in &self.ast.expressions {
            match expr {
                Exp::Maj => {}
                Exp::Maj7 => self.desc.insert_seventh(Interval::MajorSeventh),
                Exp::Minor => Evaluator::evaluate_minor(&mut self.desc),
                Exp::Dim7 => Evaluator::evaluate_dim7(&mut self.desc),
                Exp::Dim => Evaluator::evaluate_dim(&mut self.desc),
                Exp::Alt(_) => Evaluator::evaluate_alt(&mut self.desc),
                Exp::HalfDim => Evaluator::evaluate_half_dim(&mut self.desc),
                Exp::Sus(exp) => Evaluator::evaluate_sus(&mut self.desc, exp),
                Exp::Extension(exp) => Evaluator::evaluate_extension(&mut self.desc, exp),
                Exp::Add(exp) => self.desc.adds.push(exp.interval),
                Exp::Aug => self.desc.alts.push(Interval::AugmentedFifth),
                Exp::Omit(exp) => self.desc.omits.push(exp.interval),
                Exp::Bass => Evaluator::evaluate_bass(&mut self.desc),
                Exp::Power => self.desc.base_form = BaseForm::Power,
                Exp::SlashBass(exp) => self.desc.bass = Some(exp.note),
            }
        }
        self
    }

    /// Update the triad base on the derived baseForm
    fn update_triad(mut self) -> Self {
        self.desc
            .base_form
            .update_triad(&mut self.desc.interval_set);
        self
    }

    fn apply_sus(mut self) -> Self {
        if let Some(sus) = self.desc.sus {
            Self::remove_thirds(&mut self.desc.interval_set);
            self.desc.interval_set.insert(sus);
        }
        self
    }

    fn apply_alterations(mut self) -> Self {
        self.desc.alts.iter().for_each(|alt| match alt {
            Interval::DiminishedFifth | Interval::AugmentedFifth | Interval::FlatThirteenth => {
                self.desc.interval_set.replace(Interval::PerfectFifth, *alt)
            }
            _ => self.desc.interval_set.insert(*alt),
        });
        self
    }

    fn implied_extensions(mut self) -> Self {
        if let Some(ext) = self.desc.max_extension {
            if self.desc.base_form == BaseForm::Major && ext == Interval::Eleventh {
                self.desc
                    .interval_set
                    .replace(Interval::MajorThird, Interval::PerfectFourth);
            } else {
                self.desc.interval_set.insert(ext);
            }

            let seventh = if self
                .ast
                .expressions
                .iter()
                .any(|exp| matches!(exp, Exp::Maj7 | Exp::Maj))
            {
                Interval::MajorSeventh
            } else {
                Interval::MinorSeventh
            };

            let thirteenth = if self.desc.base_form == BaseForm::Major {
                vec![Interval::Ninth, seventh]
            } else {
                vec![Interval::Eleventh, Interval::Ninth, seventh]
            };

            let to_add: Vec<Interval> = match ext {
                Interval::Thirteenth => thirteenth,
                Interval::Eleventh => vec![Interval::Ninth, seventh],
                Interval::Ninth => {
                    let has_sixth = self.desc.interval_set.contains(Interval::MajorSixth)
                        || self.desc.interval_set.contains(Interval::MinorSixth);

                    if has_sixth { vec![] } else { vec![seventh] }
                }
                _ => vec![],
            };

            for interval in to_add {
                let conflicts = CONFLICT_MAP.get(&interval).cloned().unwrap_or_default();
                let blocked = self.desc.interval_set.contains(interval)
                    || conflicts.iter().any(|c| self.desc.interval_set.contains(c));

                if !blocked {
                    self.desc.interval_set.insert(interval);
                }
            }
        }
        self
    }

    fn apply_omits(mut self) -> Self {
        for omit in &self.desc.omits {
            match omit {
                Interval::PerfectFifth => {
                    self.desc.interval_set.remove(Interval::PerfectFifth);
                    self.desc.interval_set.remove(Interval::AugmentedFifth);
                    self.desc.interval_set.remove(Interval::DiminishedFifth);
                }
                Interval::MajorThird => Self::remove_thirds(&mut self.desc.interval_set),
                _ => {}
            }
        }
        self
    }

    fn apply_adds(mut self) -> Self {
        for add in &self.desc.adds {
            if *add == Interval::FlatThirteenth {
                self.desc.interval_set.remove(Interval::PerfectFifth);
            }
            self.desc.interval_set.insert(*add);
        }
        self
    }

    fn prune(mut self) -> Self {
        if self.desc.interval_set.contains(Interval::MajorSixth) {
            self.desc.interval_set.remove(Interval::Thirteenth);
        }
        self
    }

    fn set_intervals(mut self) -> Self {
        self.desc.intervals = self.desc.interval_set.iter().collect();
        self.desc.intervals.sort_by_key(|i| i.st());
        self.desc.display_intervals = self.desc.intervals.clone();
        if let Some(Exp::Sus(sus_exp)) = self
            .ast
            .expressions
            .iter()
            .find(|e| matches!(e, Exp::Sus(_)))
        {
            self.desc.display_intervals = self
                .desc
                .display_intervals
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
            self.desc.display_intervals.sort_by_key(|i| i.st());
        }
        self
    }

    fn validate(mut self) -> Self {
        let validator = Validator {};
        self.is_valid = validator.validate(
            &self.ast.expressions,
            &mut self.errors,
            &self.desc.intervals,
        );
        self
    }

    fn build_chord(mut self) -> Result<Chord, ParserErrors> {
        if !self.is_valid {
            return Err(ParserErrors::new(self.errors));
        }

        let notes = self.notes();
        let note_literals = notes.iter().map(|a| a.to_string()).collect();

        let mut interval_degrees = Vec::new();
        for e in &self.desc.intervals {
            interval_degrees.push(e.to_degree().numeric());
        }

        let mut semitones = Vec::new();
        for e in &self.desc.display_intervals {
            let v = e.st();
            semitones.push(v);
        }
        let normalized = self.normalize();

        Ok(Chord::builder(&self.name, self.ast.root)
            .descriptor(&self.descriptor(&self.name))
            .bass(self.desc.bass)
            .notes(notes)
            .note_literals(note_literals)
            .semitones(semitones)
            .interval_degrees(interval_degrees)
            .quality(self.quality())
            .normalized_intervals(self.desc.intervals)
            .intervals(self.desc.display_intervals)
            .normalized(normalized)
            .build())
    }

    /// Get the notes of the chord
    fn notes(&mut self) -> Vec<Note> {
        let mut notes = Vec::new();
        for n in &self.desc.display_intervals {
            let note = self.ast.root.get_note(n.st(), n.to_degree().numeric());
            notes.push(note);
        }
        notes
    }

    pub fn descriptor(&self, name: &str) -> String {
        let modifier_str = match &self.ast.root.modifier {
            Some(m) => m.to_string(),
            None => "".to_string(),
        };
        name.replace(&format!("{}{}", self.ast.root.literal, modifier_str), "")
    }

    fn remove_thirds(interval_set: &mut IntervalSet) {
        interval_set.remove(Interval::MinorThird);
        interval_set.remove(Interval::MajorThird);
    }

    fn evaluate_alt(desc: &mut Descriptor) {
        desc.omits.push(Interval::PerfectFifth);
        desc.insert_seventh(Interval::MinorSeventh);
        desc.alts.push(Interval::FlatNinth);
        desc.alts.push(Interval::SharpNinth);
        desc.alts.push(Interval::SharpEleventh);
        desc.alts.push(Interval::FlatThirteenth);
    }

    fn evaluate_bass(desc: &mut Descriptor) {
        desc.interval_set.remove(Interval::PerfectFifth);
        desc.interval_set.remove(Interval::MajorThird);
        desc.third = None;
    }

    fn evaluate_sus(desc: &mut Descriptor, exp: &SusExp) {
        desc.omits.push(Interval::MajorThird);
        match exp.interval {
            Interval::PerfectFourth => desc.sus = Some(exp.interval),
            Interval::AugmentedFourth => desc.alts.push(Interval::SharpEleventh),
            Interval::MinorSecond => desc.alts.push(Interval::FlatNinth),
            Interval::MajorSecond => desc.alts.push(Interval::Ninth),
            _ => {}
        }
    }

    fn evaluate_minor(desc: &mut Descriptor) {
        desc.base_form = BaseForm::Minor;
        desc.third = Some(Interval::MinorThird);
    }

    fn evaluate_dim(desc: &mut Descriptor) {
        desc.base_form = BaseForm::Dim;
        desc.third = Some(Interval::MinorThird);
    }

    fn evaluate_dim7(desc: &mut Descriptor) {
        desc.base_form = BaseForm::Dim7;
        desc.third = Some(Interval::MinorThird);
        desc.insert_seventh(Interval::DiminishedSeventh);
    }

    fn evaluate_half_dim(desc: &mut Descriptor) {
        desc.base_form = BaseForm::HalfDim;
        desc.third = Some(Interval::MinorThird);
        desc.insert_seventh(Interval::MinorSeventh);
    }

    fn evaluate_extension(desc: &mut Descriptor, exp: &ExtensionExp) {
        match exp.interval {
            Interval::PerfectFourth
            | Interval::AugmentedFourth
            | Interval::DiminishedFifth
            | Interval::AugmentedFifth
            | Interval::FlatNinth
            | Interval::SharpNinth
            | Interval::SharpEleventh
            | Interval::FlatThirteenth => desc.alts.push(exp.interval),
            Interval::MajorSixth | Interval::MinorSixth => desc.insert_sixth(exp.interval),
            Interval::MinorSeventh => {
                desc.insert_seventh(exp.interval);
                desc.alts.push(exp.interval);
            }
            Interval::Ninth | Interval::Eleventh | Interval::Thirteenth => {
                desc.max_extension = Some(
                    exp.interval
                        .max(desc.max_extension.unwrap_or(Interval::Unison)),
                )
            }
            _ => {}
        }
    }
}
