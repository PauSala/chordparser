//! # Midi Codes voicing generator

use std::u8;

use crate::chord::{intervals::Interval, note::Note, Chord};

static MAX_MIDI_CODE: u8 = 79;
static MIN_MIDI_CODE: u8 = 51;

#[derive(Debug, PartialEq, Eq, Clone)]
struct MidiNote {
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
        midi_notes.push(MidiNote::new(n, i))
    }
    midi_notes
}

pub type MidiCodesVoicing = Vec<u8>;

/// Find the note near to lead
fn nearest_lead(pl: u8, pool: &mut Vec<MidiNote>) -> u8 {
    // filter at b9 distance
    let mut not_allowed: Vec<Interval> = Vec::new();
    for i in 0..pool.len() {
        let curr = &pool[i];
        let next = &pool[(i + 1) % pool.len()];
        let dist = (curr.base as i32 - next.base as i32).abs();
        if dist == 1 || dist == 11 {
            not_allowed.push(next.int);
        }
    }
    let mut min = (u8::MAX, Interval::Unison, 0);
    for i in pool.iter() {
        if not_allowed.contains(&i.int) {
            continue;
        }
        for n in &i.available {
            let dist = (pl as i16 - *n as i16).unsigned_abs() as u8;
            if dist < min.0 {
                min.0 = dist;
                min.1 = i.int;
                min.2 = *n;
            }
        }
    }
    let mut found = (false, 0);
    for (i, el) in pool.iter().enumerate() {
        for e in &el.available {
            if min.2 == *e {
                found = (true, i);
            }
        }
    }
    if found.0 {
        pool.remove(found.1);
    }
    min.2
}

/// Sets guide notes, including major sixth, altered fifths and fourths
fn guide_notes(pool: &mut [MidiNote], v: &mut MidiCodesVoicing) {
    let binding = pool.to_owned();
    // Get guide notes
    let mut guides: Vec<&MidiNote> = binding
        .iter()
        .filter(|g| {
            matches!(
                g.int,
                Interval::MinorThird
                    | Interval::MajorThird
                    | Interval::PerfectFourth
                    | Interval::AugmentedFourth
                    | Interval::DiminishedFifth
                    | Interval::AugmentedFifth
                    | Interval::DiminishedSeventh
                    | Interval::MinorSeventh
            )
        })
        .collect();

    // Handle sixths and sevenths to avoid stacking 6ths and 7ths too close.
    // If Minor seventh is present, major sixth is handled as tension
    // If sixth or dim7 is present maj7 is handled as tension
    let has_sixth_or_dim_seventh = pool
        .iter()
        .any(|x| x.int == Interval::MajorSixth || x.int == Interval::DiminishedSeventh);
    let has_minor_seventh = pool.iter().any(|x| x.int == Interval::MinorSeventh);

    if !has_minor_seventh && has_sixth_or_dim_seventh {
        let sixth = pool.iter().find(|&x| x.int == Interval::MajorSixth);
        if let Some(s) = sixth {
            guides.push(s);
        }
    } else if !has_sixth_or_dim_seventh {
        let maj_seventh = pool.iter().find(|&x| x.int == Interval::MajorSeventh);
        if let Some(s) = maj_seventh {
            guides.push(s);
        }
    }

    let mut min = (u8::MAX, Interval::Unison);
    let mut len = guides.len();
    while !guides.is_empty() {
        for g in &guides {
            for n in &g.available {
                if *n < min.0 && *n >= MIN_MIDI_CODE {
                    min = (*n, g.int);
                }
            }
        }
        if min.0 != u8::MAX {
            v.push(min.0);
        }
        guides.retain(|i| i.int != min.1);
        min = (u8::MAX, Interval::Unison);
        if guides.len() == len {
            break;
        }
        len = guides.len();
    }
}

/// Sets non guide notes, including perfect fifth and excluding Root
fn non_guide(pool: &mut [MidiNote], v: &mut MidiCodesVoicing, lead: u8) {
    let binding = pool.to_owned();
    let mut ts: Vec<&MidiNote> = binding
        .iter()
        .filter(|g| {
            matches!(
                g.int,
                Interval::FlatNinth
                    | Interval::Ninth
                    | Interval::SharpNinth
                    | Interval::Eleventh
                    | Interval::SharpEleventh
                    | Interval::FlatThirteenth
                    | Interval::Thirteenth
            )
        })
        .collect();
    if pool.len() < 5 {
        let fifth = pool.iter().find(|&x| x.int == Interval::PerfectFifth);
        if let Some(f) = fifth {
            ts.push(f);
        }
    }

    // Handle sixths and sevenths to avoid stacking 6ths and 7ths too close.
    // If Minor seventh is present, major sixth is handled as tension
    // If sixth or dim7 is present maj7 is handled as tension
    let has_sixth_or_dim_seventh = pool
        .iter()
        .any(|x| x.int == Interval::MajorSixth || x.int == Interval::DiminishedSeventh);
    let has_minor_seventh = pool.iter().any(|x| x.int == Interval::MinorSeventh);

    // Minor seventh has been set as guide, so sixth is tension if exist
    if has_sixth_or_dim_seventh && has_minor_seventh {
        let sixth = pool.iter().find(|&x| x.int == Interval::MajorSixth);
        if let Some(s) = sixth {
            ts.push(s);
        }
    // If has a sixth, it has been set as guide and maj7 needs to be set as tension if present
    } else if has_sixth_or_dim_seventh {
        let maj_seventh = pool.iter().find(|&x| x.int == Interval::MajorSeventh);
        if let Some(s) = maj_seventh {
            ts.push(s);
        }
    }
    let mut max = (u8::MIN, Interval::Unison);
    let mut len = ts.len();
    while !ts.is_empty() {
        for g in &ts {
            for n in &g.available {
                if *n > max.0 && *n < lead && *n >= MIN_MIDI_CODE {
                    max = (*n, g.int);
                }
            }
        }
        if max.0 != u8::MIN {
            v.push(max.0);
        }
        ts.retain(|i| i.int != max.1);
        max = (u8::MIN, Interval::Unison);
        if ts.len() == len {
            break;
        }
        len = ts.len();
    }
}

/// Creates a voicing for a chord.  
/// The voicing is generated in a range from C1 to G5. Accepts a lead note to generate the voicing around it, which allows chaining distinct chords smoothly.
/// # Arguments
/// * `ch` - The chord to generate the voicing
/// * `lead_note` - The lead note of the voicing.
/// If `lead_note` is not present in the chord it will be used as boundary (meaning that the actual lead note will be the nearest note in the chord, up or down).
/// If `lead_note` is None it will be set to 79 (G4).   
/// # Returns
/// A vector of MIDI codes representing the voicing for given chord
pub fn generate_voicing(ch: &Chord, lead_note: Option<u8>) -> MidiCodesVoicing {
    let mut prev_lead = lead_note.unwrap_or(MAX_MIDI_CODE);
    if prev_lead < 65 {
        prev_lead = 65;
    }
    let mut res = Vec::new();
    let mut pool = notes_pool(ch);
    pool.sort_by_key(|f| f.base);

    if ch.bass.is_some() {
        res.push(ch.bass.as_ref().unwrap().to_midi_code() - 12);
        res.push(ch.root.to_midi_code());
    } else {
        res.push(ch.root.to_midi_code() - 12);
    }
    let lead = nearest_lead(prev_lead, &mut pool);
    guide_notes(&mut pool, &mut res);
    non_guide(&mut pool, &mut res, lead);
    res.push(lead);
    res
}
