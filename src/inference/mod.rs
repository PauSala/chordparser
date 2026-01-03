//! Inference module
use crate::chord::Chord;
use crate::inference::normalize::normalized_descriptor;
use crate::inference::tables::notes_from_midi;
use crate::{
    chord::{
        interval::{Interval, IntervalSet, THIRDS_SET},
        quality::{Pc, PcSet},
    },
    parsing::Parser,
};
pub(crate) mod normalize;
mod tables;

/// Builds an iterator of chords from the given MIDI note numbers.
///
/// If the MIDI notes can be interpreted as a valid chord, the iterator yields:
/// - the chord built with the lowest note treated as the root, and
/// - all inversions of that chord, treating the lowest note as the bass and
///   each chord tone in turn as the root.
///
/// Invalid or unparseable chord descriptors are skipped.
pub fn from_midi_codes<'a>(midi_codes: &'a [u8]) -> impl Iterator<Item = Chord> + 'a {
    let mut parser = Parser::new();

    descriptors_from_midi_codes(midi_codes)
        .into_iter()
        .filter_map(move |f| parser.parse(&f).ok())
}

/// Builds a list of chord descriptor strings from the given MIDI note numbers.
///
/// Each descriptor represents either:
/// - a chord assuming the lowest note as the root, or
/// - an inversion, with the lowest note treated as the bass.
///
/// The generated descriptors are *candidates only* and are **not guaranteed**
/// to represent valid chords. They can be validated and converted into a
/// [`crate::chord::Chord`] by passing them to the chord parser.
///
/// The returned descriptors are always enharmonized as the flat version.
pub fn descriptors_from_midi_codes(midi_codes: &[u8]) -> Vec<String> {
    if midi_codes.is_empty() {
        return vec![];
    }

    // TODO: we are skipping the sharp versions
    let root_note = notes_from_midi(midi_codes[0])
        .last()
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
            .last()
            .map(|n| n.to_string())
            .unwrap_or_default();
        chord_name.push_str(&normalized_descriptor(interval_set, (&pitch_set).into()));

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

#[cfg(test)]
mod test {
    use crate::inference::descriptors_from_midi_codes;

    #[test]
    fn test_from_midi_codes() {
        // Ebdim7(b13, add9)
        let midi_codes: &[u8] = &[3, 6, 9, 12, 17, 107];
        let candidates = descriptors_from_midi_codes(midi_codes);
        let expected = [
            "Ebdim7(b13,add9)",
            "Gbdim7(addMa7,11)/Eb",
            "Adim7(b13,add9)/Eb",
            "Cdim7(addMa7,11)/Eb",
            "F7(b9,#11)/Eb",
            "B7(b9,#11)/Eb",
        ];
        for (result, expected) in candidates.iter().zip(expected) {
            assert_eq!(result, expected);
        }

        // C69
        let midi_codes: &[u8] = &[0, 4, 7, 9, 14];
        let candidates = descriptors_from_midi_codes(midi_codes);
        let expected = [
            "C69",
            "Emi7(b13,add11)/C",
            "G6sus9/C",
            "Ami7(add11)/C",
            "D9sus/C",
        ];
        for (result, expected) in candidates.iter().zip(expected) {
            assert_eq!(result, expected);
        }

        // CmiMa11
        let midi_codes: &[u8] = &[0, 27, 43, 59, 74, 5];
        let candidates = descriptors_from_midi_codes(midi_codes);
        let expected = [
            "CmiMa11",
            "Eb69(b13,addMa7)/C",
            "G7sus(b13,add3)/C",
            "B+(b5,b9,#9)/C",
            "Dmi13(b9,omit5)/C",
            "F13(#11,omit3)/C",
        ];
        for (result, expected) in candidates.iter().zip(expected) {
            assert_eq!(result, expected);
        }
    }
}
