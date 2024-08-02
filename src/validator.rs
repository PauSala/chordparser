use crate::chord::{
    chord_ir::ChordIr,
    intervals::{Interval, SemInterval},
};

pub type Validator = fn(&mut ChordIr, &mut Vec<String>);

pub fn no_minor_and_major_thirds(chord: &mut ChordIr, errors: &mut Vec<String>) {
    let mut minthirds = (false, 0);
    let mut majthirds = (false, 0);
    for note in &chord.notes {
        if note.sem_interval == SemInterval::Third {
            if note.semitone == Interval::MinorThird.st() {
                minthirds = (true, note.pos);
            }
            if note.semitone == Interval::MajorThird.st() {
                majthirds = (true, note.pos);
            }
        }
    }
    if minthirds.0 && majthirds.0 {
        errors.push(format!(
            "Error: A chord cannot have both major and minor third at position {} {}",
            minthirds.1, majthirds.1
        ));
    }
}

pub fn no_perfect_fifth_and_altered_fifth(chord: &mut ChordIr, errors: &mut Vec<String>) {
    let mut p5 = (false, 0);
    let mut a5 = (false, 0);
    for note in &chord.notes {
        if note.sem_interval == SemInterval::Fifth {
            if note.semitone == Interval::PerfectFifth.st() {
                p5 = (true, note.pos);
            } else {
                a5 = (true, note.pos);
            }
        }
    }
    if p5.0 && a5.0 {
        errors.push(format!(
            "Error: A chord cannot have both a perfect fifth and an altered one at position {} {}",
            p5.1, a5.1
        ));
    }
}

pub fn no_duplicate_seventh(chord: &mut ChordIr, errors: &mut Vec<String>) {
    let mut m7 = 0;
    let mut maj7 = 0;
    let mut dim7 = 0;
    for note in &chord.notes {
        if note.sem_interval == SemInterval::Seventh {
            if note.semitone == Interval::DiminishedSeventh.st() {
                dim7 += 1;
            }
            if note.semitone == Interval::MinorSeventh.st() {
                m7 += 1;
            }
            if note.semitone == Interval::MajorSeventh.st() {
                maj7 += 1;
            }
        }
    }
    if m7 > 1 {
        errors.push("Error: A chord cannot have multiple minor sevenths".to_string());
    }
    if dim7 > 1 {
        errors.push("Error: A chord cannot have multiple diminished sevenths".to_string());
    }
    if maj7 > 1 {
        errors.push("Error: A chord cannot have multiple major sevenths".to_string());
    }
}

pub fn no_minor_and_major_seventh(chord: &mut ChordIr, errors: &mut Vec<String>) {
    let mut m7 = (false, 0);
    let mut maj7 = (false, 0);
    for note in &chord.notes {
        if note.sem_interval == SemInterval::Seventh {
            if note.semitone == Interval::MinorSeventh.st() {
                m7 = (true, note.pos);
            }
            if note.semitone == Interval::MajorSeventh.st() {
                maj7 = (true, note.pos);
            }
        }
    }
    if m7.0 && maj7.0 {
        errors.push(format!(
            "Error: A chord cannot have both a Minor and Major seventh at position {} {}",
            m7.1, maj7.1
        ));
    }
}

pub fn no_natural_and_altered_nine(chord: &mut ChordIr, errors: &mut Vec<String>) {
    let mut f = (false, 0);
    let mut n = (false, 0);
    let mut s = (false, 0);
    for note in &chord.notes {
        if note.sem_interval == SemInterval::Ninth {
            match note.semitone {
                13 => f = (true, note.pos),
                14 => n = (true, note.pos),
                15 => s = (true, note.pos),
                _ => {}
            }
        }
    }
    if n.0 && f.0 {
        errors.push(format!(
            "Error: A chord cannot have both a 9 and a b9 at position {} {}",
            n.1, f.1
        ));
    }

    if n.0 && s.0 {
        errors.push(format!(
            "Error: A chord cannot have both a 9 and a #9 at position {} {}",
            n.1, s.1
        ));
    }
}

pub fn no_double_eleventh(chord: &mut ChordIr, errors: &mut Vec<String>) {
    let mut n = (false, 0);
    let mut s = (false, 0);
    for note in &chord.notes {
        if note.sem_interval == SemInterval::Eleventh {
            match note.semitone {
                17 => n = (true, note.pos),
                18 => s = (true, note.pos),
                _ => {}
            }
        }
    }
    if n.0 && s.0 {
        errors.push(format!(
            "Error: A chord cannot have both an 11 and a #11 at position {} {}",
            n.1, s.1
        ));
    }
}

pub fn no_double_thirteenth(chord: &mut ChordIr, errors: &mut Vec<String>) {
    let mut f = (false, 0);
    let mut n = (false, 0);
    for note in &chord.notes {
        if note.sem_interval == SemInterval::Thirteenth {
            match note.semitone {
                20 => f = (true, note.pos),
                21 => n = (true, note.pos),
                _ => {}
            }
        }
    }
    if n.0 && f.0 {
        errors.push(format!(
            "Error: A chord cannot have both a 13 and a b13 at position {} {}",
            n.1, f.1
        ));
    }
}
