use std::u8;

use crate::chord::{intervals::Interval, note::Note, Chord};

static MAX_MIDI_CODE: u8 = 72;
static MIN_MIDI_CODE: u8 = 52;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MidiNote {
    base: u8,
    available: Vec<u8>,
    int: Interval,
}

impl MidiNote {
    pub fn new(note: &Note, int: Interval) -> MidiNote {
        let mut candidates = Vec::new();
        let mut candidate = note.to_midi_code();
        while candidate <= MAX_MIDI_CODE {
            if candidate >= MIN_MIDI_CODE {
                candidates.push(candidate);
            }
            candidate += 12;
        }
        MidiNote {
            base: int.st() % 12,
            int,
            available: candidates,
        }
    }
}

fn notes_pool(ch: &Chord) -> Vec<MidiNote> {
    let mut midi_notes = Vec::new();
    for (n, i) in ch.notes.iter().zip(ch.real_intervals.clone()) {
        midi_notes.push(MidiNote::new(&n, i))
    }
    midi_notes
}

pub type Voicing = Vec<u8>;

pub fn nearest_lead(pl: u8, pool: &mut Vec<MidiNote>, imap: &mut Vec<(Interval, usize)>) -> u8 {
    imap.sort_by_key(|e| -(e.1 as i32));
    let mut not_allowed: Vec<Interval> = Vec::new();
    for i in 0..pool.len() {
        let curr = &pool[i];
        let next = &pool[(i + 1) % pool.len()];
        let distance = (curr.base as i32 - next.base as i32).abs();
        if distance == 1 || distance == 11 {
            not_allowed.push(next.int);
        }
    }
    let mut min = (u8::MAX, Interval::Unison, 0);
    for i in pool.iter() {
        if not_allowed.contains(&i.int) {
            continue;
        }
        for n in &i.available {
            let dist = (pl as i16 - *n as i16).abs() as u8;
            if dist < min.0 {
                min.0 = dist;
                min.1 = i.int;
                min.2 = *n;
            }
        }
    }
    let found = imap.iter_mut().find(|a| a.0 == min.1).unwrap();
    found.1 += 1;
    for i in pool {
        let mut found = (false, 0);
        for (i, e) in i.available.iter().enumerate() {
            if min.2 == *e {
                found = (true, i);
            }
        }
        if found.0 {
            i.available.remove(found.1);
        }
    }
    min.2
}

pub fn guide_notes(pool: &mut Vec<MidiNote>, v: &mut Voicing) {
    let binding = pool.clone();
    let mut guides: Vec<&MidiNote> = binding
        .iter()
        .filter(|g| match g.int {
            Interval::MinorThird
            | Interval::MajorThird
            | Interval::PerfectFourth
            | Interval::AugmentedFourth
            | Interval::DiminishedFifth
            | Interval::AugmentedFifth
            | Interval::MajorSixth
            | Interval::DiminishedSeventh
            | Interval::MinorSeventh
            | Interval::MajorSeventh => true,
            _ => false,
        })
        .collect();
    let mut min = (u8::MAX, Interval::Unison);
    while guides.len() > 0 {
        for g in &guides {
            for n in &g.available {
                if *n < min.0 && *n >= MIN_MIDI_CODE {
                    min = (*n, g.int);
                }
            }
        }
        v.push(min.0);
        guides.retain(|i| i.int != min.1);
        min = (u8::MAX, Interval::Unison);
    }
}

pub fn tensions(pool: &mut Vec<MidiNote>, v: &mut Voicing, lead: u8) {
    let binding = pool.clone();
    let mut ts: Vec<&MidiNote> = binding
        .iter()
        .filter(|g| match g.int {
            Interval::PerfectFifth
            | Interval::Unison
            | Interval::FlatNinth
            | Interval::Ninth
            | Interval::SharpNinth
            | Interval::Eleventh
            | Interval::SharpEleventh
            | Interval::FlatThirteenth
            | Interval::Thirteenth => true,
            _ => false,
        })
        .collect();
    let mut min = (u8::MIN, Interval::Unison);
    while ts.len() > 0 {
        for g in &ts {
            for n in &g.available {
                if *n > min.0 && *n < lead {
                    min = (*n, g.int);
                }
            }
        }
        v.push(min.0);
        ts.retain(|i| i.int != min.1);
        min = (u8::MIN, Interval::Unison);
    }
}

pub fn voicing(ch: &Chord, _voices: usize, prev_lead: Option<u8>) -> Voicing {
    let prev_lead = prev_lead.unwrap_or(MAX_MIDI_CODE);
    let mut res = Vec::new();
    let mut pool = notes_pool(ch);
    pool.sort_by_key(|f| f.base);
    let mut imap = Vec::new();
    for i in &pool {
        imap.push((i.int, 0));
    }
    let root = {
        if ch.bass.is_some() {
            ch.bass.as_ref().unwrap().to_midi_code() - 12
        } else {
            ch.root.to_midi_code() - 12
        }
    };
    res.push(root);
    let lead = nearest_lead(prev_lead, &mut pool, &mut imap);
    guide_notes(&mut pool, &mut res);
    tensions(&mut pool, &mut res, lead);
    res.push(lead);
    // while res.len() < voices {
    //     imap.sort_by_key(|e| -(e.1 as i32));
    //     //let e =
    // }
    // dbg!(imap);
    // dbg!(pool);
    // dbg!(&res);
    res
}

#[cfg(test)]
mod test {
    use crate::parsing::Parser;

    use super::*;
    #[test]
    pub fn work() {
        let chord = Parser::new().parse("CMaj769b5(#11)").unwrap();
        let v = voicing(&chord, 4, None);
        dbg!(v);
    }
}
