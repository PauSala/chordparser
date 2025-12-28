use crate::{
    chord::{
        c_quality::{ChordQuality, EXACT_POW_SET, FIFTHS_SET, Pc, PcSet, THIRDS_SET},
        intervals::{IntDegree, IntDegreeSet, Interval, IntervalSet},
    },
    parsing::ast::Ast,
};
use ChordQuality::*;

impl Ast {
    pub fn normalize(&self) -> String {
        let intervals_slice = self.norm_intervals.as_slice();
        let mut virtual_set: PcSet = intervals_slice.into();

        // This is that in case of an omited third the quality can still be derived as Major or Minor.
        if let Some(third) = self.third
            && !virtual_set.difference(&EXACT_POW_SET).is_empty()
        {
            virtual_set.insert(Into::<Pc>::into(&third));
        }
        let mut descriptor = self.root.to_string();

        let quality: ChordQuality = (&virtual_set).into();
        if quality == Bass {
            descriptor.push_str("Bass");
            return descriptor;
        }
        let is_sus = quality.is_sus(&intervals_slice.into());

        let interval_set = &IntervalSet::from_slice(intervals_slice);
        let alterations = quality.alterations(interval_set);
        let extensions = quality
            .extensions(interval_set)
            .upgrade(Interval::MajorSixth, Interval::Thirteenth);

        let (modifier, adds) = Ast::process_extensions(&extensions, &alterations, &quality);
        let omits = self.omits(is_sus, &quality);

        descriptor.push_str(&Ast::merge_quality_modifier(&quality, modifier));
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
                descriptor.push_str("9");
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

        if let Some(bass) = self.bass {
            descriptor.push_str(&format!("/{}", bass.literal.to_string()));
        }
        descriptor
    }

    fn merge_quality_modifier(quality: &ChordQuality, modifier: Option<Interval>) -> String {
        let mod_str = modifier.map(|m| m.to_chord_notation());

        match quality {
            Maj | Bass => "".into(),
            Maj6 => "6".into(),
            Maj7 => mod_str.map_or("Ma7".into(), |m| format!("Ma{m}")),
            Dom => mod_str.unwrap_or_else(|| "7".into()),
            Mi => "mi".into(),
            Mi6 => "mi6".into(),
            Mi7 => mod_str.map_or("mi7".into(), |m| format!("mi{m}")),
            MiMaj7 => mod_str.map_or("miMa7".into(), |m| format!("miMa{m}")),
            Augmented => mod_str.map_or("+".into(), |m| format!("+{m}")),
            Diminished => "dim".into(),
            Diminished7 => "dim7".into(),
            Pow => "5".into(),
        }
    }

    fn process_extensions(
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
        let degrees = Ast::extensions_to_degrees(&alterations, &extensions, &quality);

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
            (&extensions.upgrade(Interval::MajorSixth, Interval::Thirteenth)).into();
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
        let intervals_slice = self.norm_intervals.as_slice();
        let ints: PcSet = intervals_slice.into();
        // is omit 3 if is not sus and there isn't a third
        if !is_sus && ints.intersection(&THIRDS_SET).is_empty() {
            omits.push("3".to_string());
        }
        // is omit 5 if there isn't a five and there isn't a b13
        if ints.intersection(&FIFTHS_SET).is_empty() && !ints.contains_const(&Pc::Pc20) {
            omits.push("5".to_string());
        }
        omits
    }
}
