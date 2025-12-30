use crate::{
    chord::{
        intervals::{IntDegree, IntDegreeSet, Interval, IntervalSet},
        quality::{ChordQuality, EXACT_POW_SET, FIFTHS_SET, Pc, PcSet, THIRDS_SET},
    },
    parsing::evaluator::Evaluator,
};
use ChordQuality::*;
use std::fmt::Write;

const MI: &str = "mi";
const MA: &str = "Ma";
const MA7: &str = "Ma7";
const MI7: &str = "mi7";
const MIMA7: &str = "miMa7";
const MIMA: &str = "miMa";
const AUG: &str = "+";
const DIM: &str = "dim";
const DIM7: &str = "dim7";
const FIVE: &str = "5";
const SIX: &str = "6";
const SEVEN: &str = "7";
const NINE: &str = "9";

impl<'a> Evaluator<'a> {
    pub fn quality(&self) -> ChordQuality {
        let intervals_slice = self.dc.intervals.as_slice();
        let mut virtual_set: PcSet = intervals_slice.into();

        // This is just in case of an omited third the quality can still be derived as Major or Minor.
        if let Some(third) = self.dc.third
            && !virtual_set.difference(&EXACT_POW_SET).is_empty()
        {
            virtual_set.insert(Into::<Pc>::into(&third));
        }
        (&virtual_set).into()
    }

    pub fn normalize(&self) -> String {
        let mut descriptor = String::with_capacity(128);
        write!(descriptor, "{}", self.ast.root).ok();

        let quality = self.quality();
        if quality == Bass {
            descriptor.push_str("Bass");
            return descriptor;
        }

        let intervals_pc: PcSet = self.dc.intervals.as_slice().into();
        let is_sus = quality.is_sus(&intervals_pc);
        let alterations = quality.alterations(&self.dc.interval_set);
        let extensions = quality
            .extensions(&self.dc.interval_set)
            .replace(Interval::MajorSixth, Interval::Thirteenth);

        let (modifier, mut adds) = Evaluator::split_extensions(&extensions, &alterations, &quality);
        let omits = self.omits(is_sus, &quality);

        Self::append_quality_modifier(&mut descriptor, &quality, modifier);

        if is_sus {
            if self.dc.interval_set.contains(Interval::MajorThird) {
                adds.push(Interval::MajorThird);
            }
            descriptor.push_str("sus");
        }

        // Handle the items inside parentheses (alterations, adds, omits)
        let mut has_items = false;

        let mut append_item = |desc: &mut String, item: &str, prefix: &str| {
            if !has_items {
                desc.push('(');
                has_items = true;
            } else {
                desc.push(',');
            }
            desc.push_str(prefix);
            desc.push_str(item);
        };

        for alt in alterations {
            append_item(&mut descriptor, &alt.to_chord_notation(), "");
        }

        for (i, add) in adds.iter().enumerate() {
            if *add == Interval::Ninth && (quality == Maj6 || quality == Mi6) {
                descriptor.push_str(NINE);
                continue;
            }
            let prefix = if i == 0 { "add" } else { "" };
            append_item(&mut descriptor, &add.to_chord_notation(), prefix);
        }

        for (i, omit) in omits.iter().enumerate() {
            let prefix = if i == 0 { "omit" } else { "" };
            append_item(&mut descriptor, omit, prefix);
        }

        if has_items {
            write!(descriptor, ")").ok();
        }

        if let Some(bass) = self.dc.bass {
            write!(descriptor, "/{}", bass.literal).ok();
        }

        descriptor
    }

    fn append_quality_modifier(f: &mut String, quality: &ChordQuality, modifier: Option<Interval>) {
        match quality {
            Maj | Bass => {}
            Maj6 => f.push_str(SIX),
            Maj7 => match modifier {
                None => f.push_str(MA7),
                Some(m) => {
                    f.push_str(MA);
                    f.push_str(&m.to_chord_notation());
                }
            },
            Dominant7 => {
                let modstring = modifier.map_or(SEVEN.to_string(), |m| m.to_chord_notation());
                f.push_str(&modstring);
            }
            Mi => f.push_str(MI),
            Mi6 => {
                f.push_str(MI);
                f.push_str(SIX);
            }
            Mi7 => match modifier {
                None => f.push_str(MI7),
                Some(m) => {
                    f.push_str(MI);
                    f.push_str(&m.to_chord_notation());
                }
            },
            MiMaj7 => match modifier {
                None => f.push_str(MIMA7),
                Some(m) => {
                    f.push_str(MIMA);
                    f.push_str(&m.to_chord_notation());
                }
            },
            Augmented => {
                f.push_str(AUG);
                if let Some(m) = modifier {
                    f.push_str(&m.to_chord_notation());
                }
            }
            Diminished => f.push_str(DIM),
            Diminished7 => f.push_str(DIM7),
            Power => f.push_str(FIVE),
        }
    }

    fn split_extensions(
        extensions: &IntervalSet,
        alterations: &IntervalSet,
        quality: &ChordQuality,
    ) -> (Option<Interval>, Vec<Interval>) {
        // For dim chords all extensions are adds
        if matches!(
            quality,
            ChordQuality::Diminished7 | ChordQuality::Diminished
        ) {
            return (None, extensions.iter().collect());
        }

        let mut adds: Vec<Interval> = vec![];
        let mut main: Option<Interval> = None;
        let degrees = Evaluator::extensions_to_degrees(alterations, extensions, quality);

        for curr in extensions.iter() {
            // Maj7 is always an add if it isn't part of the quality (e.g. dim7Maj7)
            if matches!(curr, Interval::MajorSeventh) {
                adds.push(curr);
                continue;
            }
            let stack: IntDegreeSet = quality
                .extension_stack()
                .iter()
                .filter(|ext| ext <= &curr.into())
                .collect();
            if stack.is_subset_of(&degrees) {
                main = Some(curr);
            } else {
                adds.push(curr);
            }
        }
        (main, adds)
    }

    fn extensions_to_degrees(
        alterations: &IntervalSet,
        extensions: &IntervalSet,
        quality: &ChordQuality,
    ) -> IntDegreeSet {
        let seventh = match quality {
            Diminished7 | Dominant7 | Maj7 | Mi7 | MiMaj7 => Some(IntDegree::Seventh),
            _ => None,
        };
        let alt_degrees: IntDegreeSet = alterations.into();
        let ext_degrees: IntDegreeSet =
            (&extensions.replace(Interval::MajorSixth, Interval::Thirteenth)).into();
        let mut res = alt_degrees.union(&ext_degrees);
        if let Some(seventh) = seventh {
            res.insert(seventh);
        }
        res
    }

    fn omits(&self, is_sus: bool, quality: &ChordQuality) -> Vec<String> {
        let mut omits = vec![];
        if matches!(quality, ChordQuality::Bass | ChordQuality::Power) {
            return omits;
        }
        let intervals_slice = self.dc.intervals.as_slice();
        let ints: PcSet = intervals_slice.into();
        // is omit 3 if is not sus and there isn't a third
        if !is_sus && ints.intersection(&THIRDS_SET).is_empty() {
            omits.push("3".to_string());
        }
        // is omit 5 if there isn't a five and there isn't a b13 (bc in this case the 5 is omited by default)
        if ints.intersection(&FIFTHS_SET).is_empty() && !ints.contains_const(&Pc::Pc20) {
            omits.push("5".to_string());
        }
        omits
    }
}
