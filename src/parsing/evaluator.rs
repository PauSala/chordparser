use std::{collections::HashMap, sync::LazyLock};

use crate::{
    chord::{
        Chord,
        intervals::{Interval, IntervalSet},
        note::Note,
    },
    parsing::{
        ast::{Ast, BaseForm},
        expression::*,
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
    pub(crate) dc: Descriptor,
    pub(crate) is_valid: bool,
    pub(crate) errors: Vec<ParserError>,
    pub(crate) name: String,
}

impl<'a> Evaluator<'a> {
    pub fn evaluate(ast: &'a Ast, name: String) -> Result<Chord, ParserErrors> {
        Self {
            ast,
            dc: Descriptor::new(),
            is_valid: false,
            errors: vec![],
            name,
        }
        .seed_defaults()
        .evaluate_expressions()
        .apply_base_form()
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

    fn seed_defaults(mut self) -> Self {
        self.dc.intervals = vec![Interval::Unison];
        self.dc.base_form = BaseForm::Major;
        self.dc.third = Some(Interval::MajorThird);
        self.dc.interval_set.insert(Interval::Unison);
        self.dc.interval_set.insert(Interval::MajorThird);
        self.dc.interval_set.insert(Interval::PerfectFifth);
        self
    }

    /// Evaluates ast's expressions.
    ///
    /// Each expression may mutate both the descriptor's interval set and it's baseForm.
    fn evaluate_expressions(mut self) -> Self {
        for expr in &self.ast.expressions {
            match expr {
                Exp::Maj => {}
                Exp::Maj7 => self.dc.insert_seventh(Interval::MajorSeventh),
                Exp::Minor => Evaluator::evaluate_minor(&mut self.dc),
                Exp::Dim7 => Evaluator::evaluate_dim7(&mut self.dc),
                Exp::Dim => Evaluator::evaluate_dim(&mut self.dc),
                Exp::Alt => Evaluator::evaluate_alt(&mut self.dc),
                Exp::HalfDim => Evaluator::evaluate_half_dim(&mut self.dc),
                Exp::Sus(exp) => Evaluator::evaluate_sus(&mut self.dc, exp),
                Exp::Extension(exp) => Evaluator::evaluate_extension(&mut self.dc, exp),
                Exp::Add(exp) => self.dc.adds.push(exp.interval),
                Exp::Aug => self.dc.alts.push(Interval::AugmentedFifth),
                Exp::Omit(exp) => self.dc.omits.push(exp.interval),
                Exp::Bass => Evaluator::evaluate_bass(&mut self.dc),
                Exp::Power => self.dc.base_form = BaseForm::Power,
                Exp::SlashBass(exp) => self.dc.bass = Some(exp.note),
            }
        }
        self
    }

    /// Update interval set based on the derived baseForm
    fn apply_base_form(mut self) -> Self {
        self.dc.base_form.apply(&mut self.dc.interval_set);
        self
    }

    fn apply_sus(mut self) -> Self {
        if let Some(sus) = self.dc.sus {
            Self::remove_thirds(&mut self.dc.interval_set);
            self.dc.interval_set.insert(sus);
        }
        self
    }

    fn apply_alterations(mut self) -> Self {
        self.dc.alts.iter().for_each(|alt| match alt {
            Interval::DiminishedFifth | Interval::AugmentedFifth | Interval::FlatThirteenth => self
                .dc
                .interval_set
                .remove_then_add(Interval::PerfectFifth, *alt),
            _ => self.dc.interval_set.insert(*alt),
        });
        self
    }

    fn implied_extensions(mut self) -> Self {
        if let Some(ext) = self.dc.max_extension {
            if self.dc.base_form == BaseForm::Major && ext == Interval::Eleventh {
                self.dc
                    .interval_set
                    .remove_then_add(Interval::MajorThird, Interval::PerfectFourth);
            } else {
                self.dc.interval_set.insert(ext);
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

            let thirteenth = if self.dc.base_form == BaseForm::Major {
                vec![Interval::Ninth, seventh]
            } else {
                vec![Interval::Eleventh, Interval::Ninth, seventh]
            };

            let to_add: Vec<Interval> = match ext {
                Interval::Thirteenth => thirteenth,
                Interval::Eleventh => vec![Interval::Ninth, seventh],
                Interval::Ninth => {
                    let has_sixth = self.dc.interval_set.contains(Interval::MajorSixth)
                        || self.dc.interval_set.contains(Interval::MinorSixth);

                    if has_sixth { vec![] } else { vec![seventh] }
                }
                _ => vec![],
            };

            for interval in to_add {
                let conflicts = CONFLICT_MAP.get(&interval).cloned().unwrap_or_default();
                let blocked = self.dc.interval_set.contains(interval)
                    || conflicts.iter().any(|c| self.dc.interval_set.contains(c));

                if !blocked {
                    self.dc.interval_set.insert(interval);
                }
            }
        }
        self
    }

    fn apply_omits(mut self) -> Self {
        for omit in &self.dc.omits {
            match omit {
                Interval::PerfectFifth => {
                    self.dc.interval_set.remove(Interval::PerfectFifth);
                    self.dc.interval_set.remove(Interval::AugmentedFifth);
                    self.dc.interval_set.remove(Interval::DiminishedFifth);
                }
                Interval::MajorThird => Self::remove_thirds(&mut self.dc.interval_set),
                _ => {}
            }
        }
        self
    }

    fn apply_adds(mut self) -> Self {
        for add in &self.dc.adds {
            if *add == Interval::FlatThirteenth {
                self.dc.interval_set.remove(Interval::PerfectFifth);
            }
            self.dc.interval_set.insert(*add);
        }
        self
    }

    fn prune(mut self) -> Self {
        if self.dc.interval_set.contains(Interval::MajorSixth) {
            self.dc.interval_set.remove(Interval::Thirteenth);
        }
        self
    }

    fn set_intervals(mut self) -> Self {
        self.dc.intervals = self.dc.interval_set.iter().collect();
        self.dc.intervals.sort_by_key(|i| i.st());
        self.dc.display_intervals = self.dc.intervals.clone();
        if let Some(Exp::Sus(sus_exp)) = self
            .ast
            .expressions
            .iter()
            .find(|e| matches!(e, Exp::Sus(_)))
        {
            self.dc.display_intervals = self
                .dc
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
            self.dc.display_intervals.sort_by_key(|i| i.st());
        }
        self
    }

    fn validate(mut self) -> Self {
        let validator = Validator {};
        self.is_valid =
            validator.validate(&self.ast.expressions, &mut self.errors, &self.dc.intervals);
        self
    }

    fn build_chord(mut self) -> Result<Chord, ParserErrors> {
        if !self.is_valid {
            return Err(ParserErrors::new(self.errors));
        }

        let notes = self.notes();
        let note_literals = notes.iter().map(|a| a.to_string()).collect();

        let mut interval_degrees = Vec::new();
        for e in &self.dc.intervals {
            interval_degrees.push(e.to_degree().numeric());
        }

        let mut semitones = Vec::new();
        for e in &self.dc.display_intervals {
            let v = e.st();
            semitones.push(v);
        }
        let normalized = self.normalize();

        Ok(Chord::builder(&self.name, self.ast.root)
            .descriptor(&self.descriptor(&self.name))
            .bass(self.dc.bass)
            .notes(notes)
            .note_literals(note_literals)
            .semitones(semitones)
            .interval_degrees(interval_degrees)
            .quality(self.quality())
            .normalized_intervals(self.dc.intervals)
            .intervals(self.dc.display_intervals)
            .normalized(normalized)
            .build())
    }

    /// Get the notes of the chord
    fn notes(&mut self) -> Vec<Note> {
        let mut notes = Vec::new();
        for n in &self.dc.display_intervals {
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
        use Interval::*;
        match exp.interval {
            PerfectFourth | AugmentedFourth | DiminishedFifth | AugmentedFifth | FlatNinth
            | SharpNinth | SharpEleventh | FlatThirteenth => desc.alts.push(exp.interval),
            MajorSixth | MinorSixth => desc.insert_sixth(exp.interval),
            MinorSeventh => {
                desc.insert_seventh(exp.interval);
                desc.alts.push(exp.interval);
            }
            Ninth | Eleventh | Thirteenth => {
                desc.max_extension = Some(exp.interval.max(desc.max_extension.unwrap_or(Unison)))
            }
            _ => {}
        }
    }
}
