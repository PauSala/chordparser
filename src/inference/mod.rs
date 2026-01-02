use crate::chord::{
    interval::{IntDegree, IntDegreeSet, Interval, IntervalSet},
    note::{Note, NoteModifier},
    quality::{ChordQuality, FIFTHS_SET, Pc, PcSet, THIRDS_SET},
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

pub fn infere(pitch_set: PcSet, interval_set: IntervalSet) -> String {
    let mut descriptor = String::with_capacity(128);
    let quality: ChordQuality = (&pitch_set).into();

    if quality == ChordQuality::Bass {
        descriptor.push_str("Bass");
        return descriptor;
    }

    let is_sus = quality.is_sus(&pitch_set);
    let alterations = quality.alterations(&interval_set);
    let extensions = quality
        .extensions(&interval_set)
        .replace(Interval::MajorSixth, Interval::Thirteenth);

    let (modifier, mut adds) = split_extensions(&extensions, &alterations, &quality);
    let omits = omits(pitch_set, is_sus, &quality);

    append_quality_modifier(&mut descriptor, &quality, modifier);

    if is_sus {
        if interval_set.contains(Interval::MajorThird) {
            adds.push(Interval::MajorThird);
        }
        descriptor.push_str("sus");
    }

    // Handle items inside parentheses (alterations, adds, omits)
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
        descriptor.push(')');
    }

    descriptor
}

pub fn from_midi_codes(midi_codes: &[u8]) -> Vec<String> {
    let mut mask: u16 = 0;
    let mut candidates: Vec<String> = vec![];
    let mut root = String::new();
    for (index, &midi_note) in midi_codes.iter().enumerate() {
        if index == 0
            && let Some(note) = notes_from_midi(midi_note).first()
        {
            root = note.to_string();
        }
        let bit = 1 << (midi_note % 12);
        if mask & bit != 0 {
            continue;
        }
        mask |= bit;
        let pitch_set = midi_codes.iter().fold(PcSet::new(), |mut acc, v| {
            acc.insert(pitch_class(midi_note, *v));
            acc
        });
        let interval_set: IntervalSet = pitch_set.into();
        let mut chord = String::new();
        if let Some(note) = notes_from_midi(midi_note).first() {
            chord.push_str(&note.to_string());
        }

        chord.push_str(&infere(pitch_set, interval_set));
        if index > 0 {
            chord.push('/');
            chord.push_str(&root);
        }
        dbg!(pitch_set);
        dbg!(interval_set);
        dbg!(&chord);
        candidates.push(chord);
    }

    candidates
}

pub fn notes_from_midi(midi: u8) -> Vec<Note> {
    use crate::chord::note::NoteLiteral::*;
    match midi % 12 {
        0 => vec![Note {
            literal: C,
            modifier: None,
        }],
        1 => vec![
            Note {
                literal: C,
                modifier: Some(NoteModifier(1)),
            }, // C#
            Note {
                literal: D,
                modifier: Some(NoteModifier(-1)),
            }, // Db
        ],
        2 => vec![Note {
            literal: D,
            modifier: None,
        }],
        3 => vec![
            Note {
                literal: D,
                modifier: Some(NoteModifier(1)),
            }, // D#
            Note {
                literal: E,
                modifier: Some(NoteModifier(-1)),
            }, // Eb
        ],
        4 => vec![Note {
            literal: E,
            modifier: None,
        }],
        5 => vec![Note {
            literal: F,
            modifier: None,
        }],
        6 => vec![
            Note {
                literal: F,
                modifier: Some(NoteModifier(1)),
            }, // F#
            Note {
                literal: G,
                modifier: Some(NoteModifier(-1)),
            }, // Gb
        ],
        7 => vec![Note {
            literal: G,
            modifier: None,
        }],
        8 => vec![
            Note {
                literal: G,
                modifier: Some(NoteModifier(1)),
            }, // G#
            Note {
                literal: A,
                modifier: Some(NoteModifier(-1)),
            }, // Ab
        ],
        9 => vec![Note {
            literal: A,
            modifier: None,
        }],
        10 => vec![
            Note {
                literal: A,
                modifier: Some(NoteModifier(1)),
            }, // A#
            Note {
                literal: B,
                modifier: Some(NoteModifier(-1)),
            }, // Bb
        ],
        11 => vec![Note {
            literal: B,
            modifier: None,
        }],
        _ => unreachable!(),
    }
}

fn pitch_class(root: u8, other: u8) -> Pc {
    let pc = ((other as i16 - root as i16).rem_euclid(12)) as u8;
    match pc {
        0 | 12 => Pc::Pc0,
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
        13 => Pc::Pc13,
        14 => Pc::Pc14,
        15 => Pc::Pc15,
        16 => Pc::Pc16,
        17 => Pc::Pc17,
        18 => Pc::Pc18,
        19 => Pc::Pc19,
        20 => Pc::Pc20,
        21 => Pc::Pc21,
        22 => Pc::Pc22,
        23 => Pc::Pc23,
        _ => unreachable!(),
    }
}

fn omits(pitch_set: PcSet, is_sus: bool, quality: &ChordQuality) -> Vec<String> {
    let mut omits = vec![];
    if matches!(quality, ChordQuality::Bass | ChordQuality::Power) {
        return omits;
    }
    // is omit 3 if is not sus and there isn't a third
    if !is_sus && pitch_set.intersection(&THIRDS_SET).is_empty() {
        omits.push("3".to_string());
    }
    // is omit 5 if there isn't a five and there isn't a b13 (bc in this case the 5 is omited by default)
    if pitch_set.intersection(&FIFTHS_SET).is_empty() && !pitch_set.contains_const(&Pc::Pc20) {
        omits.push("5".to_string());
    }
    omits
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
    let degrees = extensions_to_degrees(alterations, extensions, quality);

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

fn append_quality_modifier(f: &mut String, quality: &ChordQuality, modifier: Option<Interval>) {
    match quality {
        Maj | Bass => {}
        Maj6 => match modifier {
            None => f.push_str(SIX),
            Some(m) => {
                f.push_str(SIX);
                f.push_str(&m.to_chord_notation());
            }
        },
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

impl From<PcSet> for IntervalSet {
    fn from(pitch_set: PcSet) -> Self {
        let mut iset = IntervalSet::new();
        let mut process_later = IntervalSet::new();
        for pitch in pitch_set {
            match pitch {
                Pc::Pc0 | Pc::Pc12 => iset.insert(Interval::Unison),
                Pc::Pc1 | Pc::Pc13 => iset.insert(Interval::FlatNinth),
                Pc::Pc2 | Pc::Pc14 => iset.insert(Interval::Ninth),
                Pc::Pc3 | Pc::Pc15 => process_later.insert(Interval::MinorThird),
                Pc::Pc4 | Pc::Pc16 => iset.insert(Interval::MajorThird),
                Pc::Pc5 | Pc::Pc17 => process_later.insert(Interval::PerfectFourth),
                Pc::Pc6 | Pc::Pc18 => process_later.insert(Interval::AugmentedFourth),
                Pc::Pc7 | Pc::Pc19 => iset.insert(Interval::PerfectFifth),
                Pc::Pc8 | Pc::Pc20 => process_later.insert(Interval::AugmentedFifth),
                Pc::Pc9 | Pc::Pc21 => process_later.insert(Interval::MajorSixth),
                Pc::Pc10 | Pc::Pc22 => iset.insert(Interval::MinorSeventh),
                Pc::Pc11 | Pc::Pc23 => iset.insert(Interval::MajorSeventh),
            }
        }
        let mut pending_aug_fifth = false;
        for interval in process_later {
            match interval {
                Interval::MinorThird => {
                    if iset.contains(Interval::MajorThird) {
                        iset.insert(Interval::SharpNinth);
                    } else {
                        iset.insert(Interval::MinorThird);
                    }
                }
                Interval::PerfectFourth => {
                    if iset.contains(Interval::MajorThird) {
                        iset.insert(Interval::Eleventh);
                    } else {
                        iset.insert(Interval::PerfectFourth);
                    }
                }
                Interval::AugmentedFourth => {
                    if iset.contains(Interval::PerfectFifth) {
                        iset.insert(Interval::SharpEleventh);
                    } else {
                        iset.insert(Interval::DiminishedFifth);
                    }
                }
                Interval::AugmentedFifth => pending_aug_fifth = true,
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
                _ => {} //unreachable
            }

            if pending_aug_fifth {
                if iset.contains(Interval::MinorSeventh)
                    || iset.contains(Interval::DiminishedSeventh)
                    || iset.contains(Interval::MajorSixth)
                {
                    iset.insert(Interval::FlatThirteenth);
                } else if iset.contains(Interval::MajorThird) {
                    iset.insert(Interval::AugmentedFifth);
                } else {
                    iset.insert(Interval::MinorSixth)
                }
            }
        }

        iset
    }
}

#[cfg(test)]
mod test {
    use crate::{
        chord::{interval::IntervalSet, quality::PcSet},
        inference::{from_midi_codes, infere},
        parsing::Parser,
    };

    #[test]
    fn test() {
        let mut parser = Parser::new();
        let parsed = parser.parse("C6").unwrap();
        let intervals_slice = parsed.intervals.as_slice();
        let pcset: PcSet = intervals_slice.into();
        let intervals: IntervalSet = pcset.into();
        dbg!(pcset);
        dbg!(intervals);
        let normalized = infere(pcset, intervals);
        dbg!(normalized);
    }

    #[test]
    fn test_from_midi_codes() {
        let midi_codes: &[u8] = &[16, 55, 72, 93];
        let candidates = from_midi_codes(midi_codes);
        // dbg!(candidates);
    }
}
