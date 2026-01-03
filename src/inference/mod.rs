use crate::chord::{
    interval::{Interval, IntervalSet, THIRDS_SET},
    quality::{Pc, PcSet},
};
use crate::inference::normalize::normalize;
use crate::inference::tables::notes_from_midi;
pub mod normalize;
mod tables;

pub fn from_midi_codes(midi_codes: &[u8]) -> Vec<String> {
    if midi_codes.is_empty() {
        return vec![];
    }

    let root_note = notes_from_midi(midi_codes[0])
        .first()
        .map(|n| n.to_string())
        .unwrap_or_default();
    let mut candidates = Vec::with_capacity(midi_codes.len());
    let mut seen_masks = std::collections::HashSet::new();

    for (index, &midi_note) in midi_codes.iter().enumerate() {
        let bit_mask = 1 << (midi_note % 12);

        if !seen_masks.insert(bit_mask) {
            continue;
        }

        let mut pitch_set: PcSet = midi_codes
            .iter()
            .map(|&m| pitch_class(midi_note, m))
            .collect();

        let interval_set: IntervalSet = pitch_set.into();
        // Add a third for sus/omit chords to be detected
        if interval_set.intersection(&THIRDS_SET).is_empty() {
            pitch_set.insert(Pc::Pc4);
        }

        let mut chord_name = notes_from_midi(midi_note)
            .first()
            .map(|n| n.to_string())
            .unwrap_or_default();
        chord_name.push_str(&normalize(interval_set, (&pitch_set).into()));

        if index > 0 {
            chord_name.push('/');
            chord_name.push_str(&root_note);
        }
        candidates.push(chord_name);
    }
    candidates
}

fn pitch_class(root: u8, other: u8) -> Pc {
    let pc = ((other as i16 - root as i16).rem_euclid(12)) as u8;
    match pc {
        0 => Pc::Pc0,
        1 => Pc::Pc1,
        2 => Pc::Pc2,
        3 => Pc::Pc3,
        4 => Pc::Pc4,
        5 => Pc::Pc5,
        6 => Pc::Pc6,
        7 => Pc::Pc7,
        8 => Pc::Pc8,
        9 => Pc::Pc9,
        10 => Pc::Pc10,
        11 => Pc::Pc11,
        _ => unreachable!(),
    }
}

enum Phase {
    Immediate(Interval),
    Deferred(Interval),
    PostProcess,
}

fn classify_pc(pc: Pc) -> Phase {
    use Phase::*;
    match pc {
        Pc::Pc0 | Pc::Pc12 => Immediate(Interval::Unison),
        Pc::Pc1 | Pc::Pc13 => Immediate(Interval::FlatNinth),
        Pc::Pc2 | Pc::Pc14 => Immediate(Interval::Ninth),
        Pc::Pc3 | Pc::Pc15 => Deferred(Interval::MinorThird),
        Pc::Pc4 | Pc::Pc16 => Immediate(Interval::MajorThird),
        Pc::Pc5 | Pc::Pc17 => Immediate(Interval::Eleventh),
        Pc::Pc6 | Pc::Pc18 => Deferred(Interval::AugmentedFourth),
        Pc::Pc7 | Pc::Pc19 => Immediate(Interval::PerfectFifth),
        Pc::Pc8 | Pc::Pc20 => PostProcess,
        Pc::Pc9 | Pc::Pc21 => Deferred(Interval::MajorSixth),
        Pc::Pc10 | Pc::Pc22 => Immediate(Interval::MinorSeventh),
        Pc::Pc11 | Pc::Pc23 => Immediate(Interval::MajorSeventh),
    }
}

fn resolve_interval(iset: &mut IntervalSet, interval: Interval) {
    match interval {
        Interval::MinorThird => {
            if iset.contains(Interval::MajorThird) {
                iset.insert(Interval::SharpNinth);
            } else {
                iset.insert(Interval::MinorThird);
            }
        }
        Interval::AugmentedFourth => {
            if iset.contains(Interval::PerfectFifth) {
                iset.insert(Interval::SharpEleventh);
            } else {
                iset.insert(Interval::DiminishedFifth);
            }
        }
        Interval::MajorSixth => {
            if iset.contains(Interval::MinorSeventh) {
                iset.insert(Interval::Thirteenth);
            } else if iset.contains(Interval::DiminishedFifth)
                && iset.contains(Interval::MinorThird)
            {
                iset.insert(Interval::DiminishedSeventh);
            } else {
                iset.insert(Interval::MajorSixth);
            }
        }
        _ => {}
    }
}

fn resolve_augmented_fifth(iset: &mut IntervalSet) {
    if iset.contains(Interval::MinorSeventh)
        || iset.contains(Interval::DiminishedSeventh)
        || iset.contains(Interval::MajorSixth)
    {
        iset.insert(Interval::FlatThirteenth);
    } else if iset.contains(Interval::MajorThird) {
        iset.insert(Interval::AugmentedFifth);
    } else {
        iset.insert(Interval::MinorSixth);
    }
}

impl From<PcSet> for IntervalSet {
    fn from(pitch_set: PcSet) -> Self {
        let mut iset = IntervalSet::new();
        let mut process_later = IntervalSet::new();
        let mut pending_aug_fifth = false;
        for pc in pitch_set {
            match classify_pc(pc) {
                Phase::Immediate(i) => iset.insert(i),
                Phase::Deferred(i) => process_later.insert(i),
                Phase::PostProcess => pending_aug_fifth = true,
            }
        }
        for interval in process_later {
            if interval == Interval::AugmentedFifth {
                pending_aug_fifth = true;
            }
            resolve_interval(&mut iset, interval);
        }
        if pending_aug_fifth {
            resolve_augmented_fifth(&mut iset);
        }

        iset
    }
}

#[cfg(test)]
mod test {
    use crate::{
        chord::{interval::IntervalSet, quality::PcSet},
        inference::{from_midi_codes, normalize},
        parsing::Parser,
    };

    #[test]
    fn test() {
        let mut parser = Parser::new();
        let parsed = parser.parse("C6").unwrap();
        let intervals_slice = parsed.intervals.as_slice();
        let pitch_set: PcSet = intervals_slice.into();
        let intervals: IntervalSet = pitch_set.into();
        let _normalized = normalize(intervals, (&pitch_set).into());
    }

    #[test]
    fn test_from_midi_codes() {
        let midi_codes: &[u8] = &[4, 7, 12];
        let candidates = from_midi_codes(midi_codes);
        dbg!(candidates);
    }
}
