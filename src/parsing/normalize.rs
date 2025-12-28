use crate::{
    chord::{
        c_quality::{ChordQuality, FIFTHS_SET, Pc, PcSet, THIRDS_SET},
        intervals::{IntDegree, IntDegreeSet, Interval, IntervalSet},
    },
    parsing::ast::Ast,
};

impl Ast {
    pub fn normalize(&self) -> String {
        let intervals_slice = self.norm_intervals.as_slice();
        let mut virtual_set: PcSet = intervals_slice.into();

        // This is that in case of an omited third the quality can still be derived as Major or Minor.
        if let Some(third) = self.third {
            virtual_set.insert(Into::<Pc>::into(&third));
        }
        let quality: ChordQuality = (&virtual_set).into();
        let is_sus = quality.is_sus(&intervals_slice.into());
        let interval_set = &IntervalSet::from_slice(intervals_slice);
        let alterations = quality.alterations(interval_set);
        let extensions = quality.extensions(interval_set);
        let (modifier, adds) = Ast::process_extensions(&extensions, &alterations, &quality);
        let omits = self.omits(is_sus);

        let mut descriptor = String::new();
        descriptor.push_str(&Ast::merge_quality_modifier(&quality, modifier));

        if is_sus {
            descriptor.push_str("sus");
        }

        let mut items: Vec<String> = Vec::new();
        for alt in alterations {
            items.push(alt.to_chord_notation());
        }

        for add in adds {
            // Handle 69
            if add == Interval::Ninth
                && (quality == ChordQuality::Maj6 || quality == ChordQuality::Mi6)
            {
                descriptor.push_str("9");
                continue;
            }
            items.push(format!("add{}", add.to_chord_notation()));
        }

        for omit in omits {
            items.push(omit);
        }

        if !items.is_empty() {
            descriptor.push('(');
            descriptor.push_str(&items.join(","));
            descriptor.push(')');
        }
        dbg!(&descriptor);
        descriptor
    }

    fn merge_quality_modifier(quality: &ChordQuality, modifier: Option<Interval>) -> String {
        let mod_str = modifier.map(|m| m.to_chord_notation());

        match quality {
            ChordQuality::Maj | ChordQuality::Bass => "".into(),
            ChordQuality::Maj6 => "6".into(),
            ChordQuality::Maj7 => mod_str.map_or("Maj7".into(), |m| format!("Maj{m}")),
            ChordQuality::Dom => mod_str.unwrap_or_else(|| "7".into()),
            ChordQuality::Mi => "mi".into(),
            ChordQuality::Mi6 => "mi6".into(),
            ChordQuality::Mi7 => mod_str.map_or("mi7".into(), |m| format!("mi{m}")),
            ChordQuality::MiMaj7 => mod_str.map_or("miMa7".into(), |m| format!("miMa{m}")),
            ChordQuality::Augmented => mod_str.map_or("+".into(), |m| format!("+{m}")),
            ChordQuality::Diminished => "dim".into(),
            ChordQuality::Diminished7 => "dim7".into(),
            ChordQuality::Pow => "5".into(),
        }
    }

    fn process_extensions(
        extensions: &IntervalSet,
        alterations: &IntervalSet,
        quality: &ChordQuality,
    ) -> (Option<Interval>, Vec<Interval>) {
        // For dim7 chords all extensions are adds
        if *quality == ChordQuality::Diminished7 {
            return (None, extensions.iter().collect());
        }
        let mut adds: Vec<Interval> = vec![];
        let mut main: Option<Interval> = None;
        let degrees = Ast::extensions_to_degrees(&alterations, &extensions, &quality);

        for curr in extensions.iter() {
            // Maj7 is always an add if it's not part of the quality (e.g. dim7Maj7)
            if curr == Interval::MajorSeventh {
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
            ChordQuality::Diminished7
            | ChordQuality::Dom
            | ChordQuality::Maj7
            | ChordQuality::Mi7
            | ChordQuality::MiMaj7 => Some(IntDegree::Seventh),
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

    fn omits(&self, is_sus: bool) -> Vec<String> {
        let mut omits = vec![];
        let intervals_slice = self.norm_intervals.as_slice();
        let ints: PcSet = intervals_slice.into();
        // is omit 3 if is not sus and there isn't a third
        if !is_sus && ints.intersection(&THIRDS_SET).is_empty() {
            omits.push("omit3".to_string());
        }
        // is omit 5 if there isn't a five and there isn't a b13
        if ints.intersection(&FIFTHS_SET).is_empty() && !ints.contains_const(&Pc::Pc20) {
            omits.push("omit5".to_string());
        }
        omits
    }
}
