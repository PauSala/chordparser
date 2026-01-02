use crate::chord::{
    interval::{FIFTHS_SET, IntDegree, IntDegreeSet, Interval, IntervalSet, THIRDS_SET},
    quality::{ChordQuality, PcSet},
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

pub fn normalize(pitch_set: PcSet, interval_set: IntervalSet, quality: ChordQuality) -> String {
    let mut descriptor = String::with_capacity(128);

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
    let (omits_list, omits_count) = omits(interval_set, is_sus, &quality);

    append_quality_modifier(&mut descriptor, &quality, modifier);

    if is_sus {
        if interval_set.contains(Interval::MajorThird) {
            adds.push(Interval::MajorThird);
        }
        descriptor.push_str("sus");
    }

    let mut open_paren = false;
    let ensure_paren = |d: &mut String, has_p: &mut bool| {
        if !*has_p {
            d.push('(');
            *has_p = true;
        } else {
            d.push(',');
        }
    };

    for alt in alterations {
        ensure_paren(&mut descriptor, &mut open_paren);
        descriptor.push_str(alt.to_chord_notation());
    }

    for (i, add) in adds.iter().enumerate() {
        if *add == Interval::Ninth && (quality == Maj6 || quality == Mi6) {
            descriptor.push_str(NINE);
            continue;
        }
        ensure_paren(&mut descriptor, &mut open_paren);
        if i == 0 {
            descriptor.push_str("add");
        }
        descriptor.push_str(add.to_chord_notation());
    }

    for i in 0..omits_count {
        if let Some(omit_str) = omits_list[i] {
            ensure_paren(&mut descriptor, &mut open_paren);
            if i == 0 {
                descriptor.push_str("omit");
            }
            descriptor.push_str(omit_str);
        }
    }

    if open_paren {
        descriptor.push(')');
    }

    descriptor
}

fn omits(
    interval_set: IntervalSet,
    is_sus: bool,
    quality: &ChordQuality,
) -> ([Option<&'static str>; 2], usize) {
    let mut res = [None; 2];
    let mut count = 0;

    if matches!(quality, ChordQuality::Bass | ChordQuality::Power) {
        return (res, 0);
    }
    // is omit 3 if is not sus and there isn't a third
    if !is_sus && interval_set.intersection(&THIRDS_SET).is_empty() {
        res[count] = Some("3");
        count += 1;
    }
    // is omit 5 if there isn't a five and there isn't a b13 (bc in this case the 5 is omited by default)
    if interval_set.intersection(&FIFTHS_SET).is_empty()
        && !interval_set.contains(&Interval::FlatThirteenth)
    {
        res[count] = Some("5");
        count += 1;
    }

    (res, count)
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
            let modstring = modifier.map_or(SEVEN, |m| m.to_chord_notation());
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
