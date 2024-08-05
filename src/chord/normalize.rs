use super::{
    intervals::{Interval, SemInterval},
    quality::Quality,
    Chord,
};

pub fn normalize(ch: &Chord) -> String {
    let mut res = ch.root.to_string();
    match ch.quality {
        Quality::Power => {
            res.push('5');
        }
        Quality::Major6 => {
            res.push('6');
            let mmod = get_main_mod(ch);
            if let Some(mo) = mmod {
                res.push_str(&mo.to_string());
            }
            return _normalize(ch, res);
        }
        Quality::Major7 => {
            res.push_str("Maj");
            let mut mmod = get_main_mod(ch).unwrap();
            if mmod == Interval::Eleventh
                && ch.is_sus
                && !ch
                    .semantic_intervals
                    .iter()
                    .any(|i| *i == SemInterval::Ninth.numeric())
            {
                mmod = Interval::Ninth;
            } else if mmod == Interval::Eleventh {
                mmod = Interval::MinorSeventh
            }
            res.push_str(&mmod.to_string().replace("Maj", ""));
            if should_add_sus(ch) {
                res.push_str("sus");
            }
            return _normalize(ch, res);
        }
        Quality::Minor6 => {
            res.push_str("min6");
            let mmod = get_main_mod(ch);
            if let Some(mo) = mmod {
                res.push_str(&mo.to_string());
            }
            return _normalize(ch, res);
        }
        Quality::Minor7 | Quality::SemiDiminished => {
            res.push_str("min");
            let mmod = get_main_mod(ch).unwrap();
            res.push_str(&mmod.to_string());
            return _normalize(ch, res);
        }
        Quality::MinorMaj7 => {
            res.push_str("min");
            let mmod = get_main_mod(ch).unwrap();
            if mmod != Interval::MajorSeventh {
                res.push_str("Maj");
            }
            res.push_str(&mmod.to_string());
            return _normalize(ch, res);
        }
        Quality::Diminished => {
            res.push_str("dim");
            if ch.has(Interval::DiminishedSeventh) {
                res.push('7');
            }
            return _normalize(ch, res);
        }
        Quality::Dominant => {
            res.push_str("");
            let mut mmod = get_main_mod(ch).unwrap();
            if mmod == Interval::Eleventh
                && ch.is_sus
                && !ch
                    .semantic_intervals
                    .iter()
                    .any(|i| *i == SemInterval::Ninth.numeric())
            {
                mmod = Interval::Ninth;
            } else if mmod == Interval::Eleventh {
                mmod = Interval::MinorSeventh
            }
            res.push_str(&mmod.to_string());
            if should_add_sus(ch) {
                res.push_str("sus");
            }
            return _normalize(ch, res);
        }
        Quality::Major | Quality::Minor => {
            if ch.quality == Quality::Minor {
                res.push_str("min");
            }
            if ch.is_sus {
                res.push_str("sus");
            }
            return _normalize(ch, res);
        }
    }
    if ch.bass.is_some() {
        res.push('/');
        res.push_str(&ch.bass.as_ref().unwrap().to_string());
    }
    res
}

fn should_add_sus(ch: &Chord) -> bool {
    ch.is_sus && (ch.has(Interval::Eleventh) || ch.has(Interval::PerfectFourth))
}

fn _normalize(ch: &Chord, mut base: String) -> String {
    let mut ext = Vec::new();
    let alter = get_alt_notes(ch);
    for a in alter {
        ext.push(a.to_human_readable());
    }
    let adds = get_adds(ch);
    for (i, a) in adds.iter().enumerate() {
        let mut r = String::new();
        if i == 0 {
            r.push_str("add");
        }
        r.push_str(&a.to_human_readable());
        ext.push(r);
    }
    let omits = get_omits(ch);
    for (i, o) in omits.iter().enumerate() {
        let mut r = String::new();
        if i == 0 {
            r.push_str("omit");
        }
        r.push_str(o);
        ext.push(r);
    }
    if !ext.is_empty() {
        base.push('(');
        base.push_str(&ext.join(","));
        base.push(')');
    }
    base
}

fn get_omits(ch: &Chord) -> Vec<String> {
    let mut res = Vec::new();
    if !(ch
        .semantic_intervals
        .iter()
        .any(|i| *i == SemInterval::Third.numeric() || *i == SemInterval::Fourth.numeric())
        || ch.is_sus && ch.has(Interval::Eleventh))
    {
        res.push("3".to_string());
    }
    if !ch
        .semantic_intervals
        .iter()
        .any(|i| *i == SemInterval::Fifth.numeric())
        && !ch.has(Interval::FlatThirteenth)
    {
        res.push("5".to_string());
    }
    res
}

fn get_main_mod(ch: &Chord) -> Option<Interval> {
    match ch.quality {
        Quality::Power => None,
        Quality::Major => None,
        Quality::Minor => None,
        Quality::Major6 | Quality::Minor6 => {
            if ch.has(Interval::Ninth) {
                return Some(Interval::Ninth);
            }
            None
        }
        Quality::Major7 | Quality::Dominant => {
            if ch.has(Interval::Thirteenth)
                && ch
                    .semantic_intervals
                    .iter()
                    .any(|i| i == &SemInterval::Ninth.numeric())
            {
                return Some(Interval::Thirteenth);
            }
            if ch.has(Interval::Eleventh)
                && ch
                    .semantic_intervals
                    .iter()
                    .any(|i| i == &SemInterval::Ninth.numeric())
            {
                return Some(Interval::Eleventh);
            }
            if ch.has(Interval::Ninth) {
                return Some(Interval::Ninth);
            }
            if ch.quality == Quality::Major7 {
                return Some(Interval::MajorSeventh);
            }
            Some(Interval::MinorSeventh)
        }
        Quality::Minor7 | Quality::MinorMaj7 | Quality::SemiDiminished => {
            if ch.has(Interval::Thirteenth)
                && ch
                    .semantic_intervals
                    .iter()
                    .any(|i| i == &SemInterval::Ninth.numeric())
                && ch
                    .semantic_intervals
                    .iter()
                    .any(|i| i == &SemInterval::Eleventh.numeric())
            {
                return Some(Interval::Thirteenth);
            }
            if ch.has(Interval::Eleventh)
                && ch
                    .semantic_intervals
                    .iter()
                    .any(|i| i == &SemInterval::Ninth.numeric())
            {
                return Some(Interval::Eleventh);
            }
            if ch.has(Interval::Ninth) {
                return Some(Interval::Ninth);
            }
            if ch.quality == Quality::Minor7 || ch.quality == Quality::SemiDiminished {
                return Some(Interval::MinorSeventh);
            }
            if ch.has(Interval::MajorSeventh) {
                return Some(Interval::MajorSeventh);
            }
            None
        }
        Quality::Diminished => {
            if ch.has(Interval::Thirteenth)
                && ch
                    .semantic_intervals
                    .iter()
                    .any(|i| i == &SemInterval::Ninth.numeric())
                && ch
                    .semantic_intervals
                    .iter()
                    .any(|i| i == &SemInterval::Eleventh.numeric())
            {
                return Some(Interval::Thirteenth);
            }
            if ch.has(Interval::Eleventh)
                && ch
                    .semantic_intervals
                    .iter()
                    .any(|i| i == &SemInterval::Ninth.numeric())
            {
                return Some(Interval::Eleventh);
            }
            if ch.has(Interval::Ninth) {
                return Some(Interval::Ninth);
            }
            None
        }
    }
}

fn get_adds(ch: &Chord) -> Vec<Interval> {
    let mut adds = Vec::new();
    match ch.quality {
        Quality::Power => adds,
        Quality::Major7 | Quality::Dominant => {
            if ch.has(Interval::Thirteenth)
                && !ch
                    .semantic_intervals
                    .iter()
                    .any(|i| *i == SemInterval::Ninth.numeric())
            {
                adds.push(Interval::Thirteenth);
            }
            if ch.has(Interval::Eleventh)
                && !ch.real_intervals.iter().any(|i| *i == Interval::Ninth)
                && !ch.is_sus
            {
                adds.push(Interval::Eleventh);
            }
            adds
        }
        Quality::Minor7 | Quality::MinorMaj7 | Quality::SemiDiminished => {
            if ch.has(Interval::Thirteenth)
                && !ch.has(Interval::MajorSixth)
                && (!ch.real_intervals.iter().any(|i| *i == Interval::Eleventh)
                    || !ch.real_intervals.iter().any(|i| *i == Interval::Ninth))
            {
                adds.push(Interval::Thirteenth);
            }
            if ch.has(Interval::Eleventh)
                && !ch.real_intervals.iter().any(|i| *i == Interval::Ninth)
            {
                adds.push(Interval::Eleventh);
            }
            if ch.has(Interval::MajorSixth) {
                adds.push(Interval::MajorSixth);
            }
            adds
        }
        Quality::Diminished => ch
            .real_intervals
            .iter()
            .filter(|a| {
                matches!(
                    a,
                    Interval::MajorSeventh
                        | Interval::Ninth
                        | Interval::FlatNinth
                        | Interval::SharpNinth
                        | Interval::Eleventh
                        | Interval::SharpEleventh
                        | Interval::FlatThirteenth
                        | Interval::Thirteenth
                )
            })
            .cloned()
            .collect(),
        Quality::Major6 | Quality::Minor6 => {
            if ch.has(Interval::Eleventh) {
                adds.push(Interval::Eleventh);
            }
            if ch.has(Interval::MajorSeventh) {
                adds.push(Interval::MajorSeventh);
            }
            adds
        }
        Quality::Major | Quality::Minor => ch
            .real_intervals
            .iter()
            .filter(|a| {
                matches!(
                    a,
                    Interval::Ninth | Interval::Eleventh | Interval::Thirteenth
                )
            })
            .cloned()
            .collect(),
    }
}

fn get_alt_notes(ch: &Chord) -> Vec<Interval> {
    let res = Vec::new();
    let altered = [
        Interval::DiminishedFifth,
        Interval::AugmentedFifth,
        Interval::MinorSixth,
        Interval::FlatNinth,
        Interval::SharpNinth,
        Interval::SharpEleventh,
        Interval::FlatThirteenth,
    ];
    match ch.quality {
        Quality::Power => res,
        Quality::Diminished => Vec::new(),
        _ => ch
            .real_intervals
            .iter()
            .filter(|i| altered.contains(i))
            .cloned()
            .collect(),
    }
}
