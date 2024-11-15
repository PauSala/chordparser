use super::{
    intervals::{Interval, SemInterval},
    quality::Quality,
    Chord,
};

pub fn normalize(ch: &Chord) -> String {
    let mut res = ch.root.to_string();
    if ch.real_intervals.len() == 1 {
        res.push_str("Bass");
        return res;
    }
    match ch.quality {
        Quality::Power => {
            res.push('5');
            res
        }
        Quality::Major6 => {
            res.push('6');
            let mmod = get_mod(ch);
            if let Some(mo) = mmod {
                res.push_str(&mo.to_string());
            }
            _normalize(ch, res)
        }
        Quality::Minor6 => {
            res.push_str("Min6");
            let mmod = get_mod(ch);
            if let Some(mo) = mmod {
                res.push_str(&mo.to_string());
            }
            _normalize(ch, res)
        }
        Quality::Major7 => {
            res.push_str("Maj");
            let mmod = get_mod(ch).unwrap();
            res.push_str(&mmod.to_string().replace("Maj", ""));
            if should_add_sus(ch) {
                res.push_str("sus");
            }
            _normalize(ch, res)
        }
        Quality::Dominant => {
            res.push_str("");
            let mmod = get_mod(ch).unwrap();
            res.push_str(&mmod.to_string());
            if should_add_sus(ch) {
                res.push_str("sus");
            }
            _normalize(ch, res)
        }
        Quality::Minor7 => {
            res.push_str("Min");
            let mmod = get_mod(ch).unwrap();
            res.push_str(&mmod.to_string());
            _normalize(ch, res)
        }
        Quality::MinorMaj7 => {
            res.push_str("Min");
            let mmod = get_mod(ch).unwrap();
            if mmod != Interval::MajorSeventh {
                res.push_str("Maj");
            }
            res.push_str(&mmod.to_string());
            _normalize(ch, res)
        }
        Quality::Diminished => {
            res.push_str("dim");
            if ch.has(Interval::DiminishedSeventh) {
                res.push('7');
            }
            _normalize(ch, res)
        }
        Quality::Major | Quality::Minor => {
            if ch.quality == Quality::Minor {
                res.push_str("Min");
            }
            // Because sus2 is sus but is just an omit3 with a ninth
            if ch.is_sus && ch.has(Interval::PerfectFourth) {
                res.push_str("sus");
            }
            _normalize(ch, res)
        }
    }
}

fn should_add_sus(ch: &Chord) -> bool {
    (ch.quality == Quality::Dominant
        || ch.quality == Quality::Major7
        || ch.quality == Quality::Major)
        && (ch.has(Interval::Eleventh) || ch.has(Interval::PerfectFourth))
}

fn _normalize(ch: &Chord, mut base: String) -> String {
    let mut ext = Vec::new();
    let alter = get_alt_notes(ch);
    for a in alter {
        ext.push(a.to_chord_notation());
    }
    let adds = get_adds(ch);
    for (i, a) in adds.iter().enumerate() {
        let mut r = String::new();
        if i == 0 {
            r.push_str("add");
        }
        r.push_str(&a.to_chord_notation());
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
    if ch.bass.is_some() {
        base.push('/');
        base.push_str(&ch.bass.as_ref().unwrap().to_string());
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
    if !ch.has_sem(SemInterval::Fifth) && !ch.has(Interval::FlatThirteenth) {
        res.push("5".to_string());
    }
    res
}

fn get_mod(ch: &Chord) -> Option<Interval> {
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
            if ch.has(Interval::Thirteenth) && ch.has_sem(SemInterval::Ninth) {
                return Some(Interval::Thirteenth);
            }
            if ch.has(Interval::Eleventh) && ch.has_sem(SemInterval::Ninth) {
                if (ch.is_sus && !ch.has_sem(SemInterval::Ninth)) || ch.has(Interval::Ninth) {
                    return Some(Interval::Ninth);
                }
                return Some(Interval::MinorSeventh);
            }
            if ch.has(Interval::Ninth) {
                return Some(Interval::Ninth);
            }
            if ch.quality == Quality::Major7 {
                return Some(Interval::MajorSeventh);
            }
            Some(Interval::MinorSeventh)
        }
        Quality::Minor7 | Quality::MinorMaj7 => {
            if ch.has(Interval::Thirteenth)
                && ch.has_sem(SemInterval::Ninth)
                && ch.has_sem(SemInterval::Eleventh)
            {
                return Some(Interval::Thirteenth);
            }
            if ch.has(Interval::Eleventh) && ch.has_sem(SemInterval::Ninth) {
                return Some(Interval::Eleventh);
            }
            if ch.has(Interval::Ninth) {
                return Some(Interval::Ninth);
            }
            if ch.quality == Quality::Minor7 {
                return Some(Interval::MinorSeventh);
            }
            if ch.has(Interval::MajorSeventh) {
                return Some(Interval::MajorSeventh);
            }
            None
        }
        Quality::Diminished => {
            if ch.has(Interval::Thirteenth)
                && ch.has_sem(SemInterval::Ninth)
                && ch.has_sem(SemInterval::Eleventh)
            {
                return Some(Interval::Thirteenth);
            }
            if ch.has(Interval::Eleventh) && ch.has_sem(SemInterval::Ninth) {
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
            if ch.has(Interval::Thirteenth) && !ch.has_sem(SemInterval::Ninth) {
                adds.push(Interval::Thirteenth);
            }
            if ch.has(Interval::Eleventh) && !ch.has(Interval::Ninth) {
                adds.push(Interval::Eleventh);
            }
            adds
        }
        Quality::Minor7 | Quality::MinorMaj7 => {
            if ch.has(Interval::Thirteenth)
                && !ch.has(Interval::MajorSixth)
                && (!ch.has(Interval::Eleventh) && !ch.has(Interval::SharpEleventh)
                    || !ch.has(Interval::Ninth))
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
                        | Interval::Eleventh
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
        Interval::DiminishedSeventh,
    ];
    match ch.quality {
        Quality::Power => res,
        Quality::Diminished => ch
            .real_intervals
            .iter()
            .filter(|i| {
                altered.contains(i)
                    && *i != &Interval::DiminishedFifth
                    && *i != &Interval::DiminishedSeventh
            })
            .cloned()
            .collect(),
        _ => ch
            .real_intervals
            .iter()
            .filter(|i| altered.contains(i))
            .cloned()
            .collect(),
    }
}
