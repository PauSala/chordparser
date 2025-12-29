use crate::{
    chord::{
        intervals::{IntDegree, IntDegreeSet, Interval, IntervalSet},
        quality::{ChordQuality, EXACT_POW_SET, FIFTHS_SET, Pc, PcSet, THIRDS_SET},
    },
    parsing::evaluator::Evaluator,
};
use ChordQuality::*;

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
        let mut descriptor = self.ast.root.to_string();
        let quality: ChordQuality = self.quality();

        if quality == Bass {
            descriptor.push_str("Bass");
            return descriptor;
        }

        // Collect data
        let is_sus = quality.is_sus(&(self.dc.intervals.as_slice()).into());
        let alterations = quality.alterations(&self.dc.interval_set);
        let extensions = quality
            .extensions(&self.dc.interval_set)
            .replace(Interval::MajorSixth, Interval::Thirteenth);
        let (modifier, adds) = Evaluator::split_extensions(&extensions, &alterations, &quality);
        let omits = self.omits(is_sus, &quality);

        // Render descriptor
        descriptor.push_str(&Evaluator::format_quality_modifier(&quality, modifier));
        if is_sus {
            descriptor.push_str("sus");
        }

        let mut items: Vec<String> = Vec::new();
        for alt in alterations {
            items.push(alt.to_chord_notation());
        }

        for (i, add) in adds.iter().enumerate() {
            let prefix = if i == 0 { "add" } else { "" };
            // Handle 69
            if *add == Interval::Ninth && (quality == Maj6 || quality == Mi6) {
                descriptor.push_str(NINE);
                continue;
            }
            items.push(format!("{}{}", prefix, add.to_chord_notation()));
        }

        for (i, omit) in omits.iter().enumerate() {
            let prefix = if i == 0 { "omit" } else { "" };
            items.push(format!("{}{}", prefix, omit));
        }

        if !items.is_empty() {
            descriptor.push('(');
            descriptor.push_str(&items.join(","));
            descriptor.push(')');
        }

        if let Some(bass) = self.dc.bass {
            descriptor.push_str(&format!("/{}", bass.literal));
        }
        descriptor
    }

    fn format_quality_modifier(quality: &ChordQuality, modifier: Option<Interval>) -> String {
        let mod_str = modifier.map(|m| m.to_chord_notation());
        match quality {
            Maj | Bass => String::new(),
            Maj6 => SIX.into(),
            Maj7 => mod_str.map_or_else(|| MA7.into(), |m| format!("{MA}{m}")),
            Dom => mod_str.unwrap_or_else(|| SEVEN.into()),
            Mi => MI.into(),
            Mi6 => format!("{MI}{SIX}"),
            Mi7 => mod_str.map_or_else(|| MI7.into(), |m| format!("{MI}{m}")),
            MiMaj7 => mod_str.map_or_else(|| MIMA7.into(), |m| format!("{MIMA}{m}")),
            Augmented => mod_str.map_or_else(|| AUG.into(), |m| format!("{AUG}{m}")),
            Diminished => DIM.into(),
            Diminished7 => DIM7.into(),
            Pow => FIVE.into(),
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
            Diminished7 | Dom | Maj7 | Mi7 | MiMaj7 => Some(IntDegree::Seventh),
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
        if matches!(quality, ChordQuality::Bass | ChordQuality::Pow) {
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
