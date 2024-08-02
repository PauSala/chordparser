use crate::chord::{
    chord_ir::ChordIr,
    intervals::{Interval, SemInterval},
    note::NoteDescriptor,
};

pub(crate) type Transformer = fn(&mut ChordIr);

pub(crate) fn implicit_third(ir: &mut ChordIr) {
    if !ir.is_sus && !ir.omits.third && !ir.has(SemInterval::Third) {
        ir.notes.push(NoteDescriptor::new(
            SemInterval::Third,
            Interval::MajorThird.st(),
            usize::MAX,
        ));
    }
}

pub(crate) fn implicit_fifth(ir: &mut ChordIr) {
    if !ir.omits.five && !ir.has(SemInterval::Fifth) {
        ir.notes.push(NoteDescriptor::new(
            SemInterval::Fifth,
            Interval::PerfectFifth.st(),
            usize::MAX,
        ));
    }
}

pub(crate) fn implicit_min_seventh(ir: &mut ChordIr) {
    let add_len = ir.adds.len();
    let tensions_len = ir
        .notes
        .iter()
        .filter(|n| {
            matches!(
                n.sem_interval,
                SemInterval::Ninth | SemInterval::Eleventh | SemInterval::Thirteenth
            )
        })
        .collect::<Vec<&NoteDescriptor>>()
        .len();
    let t7 = ir.has(SemInterval::Seventh);

    // Implicit seventh is only set when there are tensions not comming from an add modifier
    // and a sixth has not been set.
    if !t7 && add_len < tensions_len && !ir.has(SemInterval::Sixth) {
        ir.notes.push(NoteDescriptor::new(
            SemInterval::Seventh,
            Interval::MinorSeventh.st(),
            usize::MAX,
        ));
    }
}

pub(crate) fn implicit_ninth(ir: &mut ChordIr) {
    let add13 = ir.has_add(SemInterval::Thirteenth);
    let t13 = ir.has(SemInterval::Thirteenth);
    let t11 = ir.has(SemInterval::Eleventh);
    let t9 = ir.has(SemInterval::Ninth);
    let a11 = ir.has_add(SemInterval::Eleventh);

    if !add13 && !a11 && t13 && !t9 {
        ir.notes.push(NoteDescriptor::new(
            SemInterval::Ninth,
            Interval::Ninth.st(),
            usize::MAX,
        ))
    }

    let t9 = ir.has(SemInterval::Ninth);
    if !a11 && t11 && !t9 {
        ir.notes.push(NoteDescriptor::new(
            SemInterval::Ninth,
            Interval::Ninth.st(),
            usize::MAX,
        ))
    }
}

pub(crate) fn implicit_eleventh(ir: &mut ChordIr) {
    let add13 = ir.has_add(SemInterval::Thirteenth);
    let t13 = ir.has(SemInterval::Thirteenth);
    let t11 = ir.has(SemInterval::Eleventh);

    if !add13 && t13 && !t11 && ir.has_minor_third() {
        ir.notes.push(NoteDescriptor::new(
            SemInterval::Eleventh,
            Interval::Eleventh.st(),
            usize::MAX,
        ))
    }
}

pub(crate) fn remove_omits(ir: &mut ChordIr) {
    if ir.omits.five {
        ir.notes = ir
            .notes
            .iter()
            .filter(|n| n.semitone != Interval::PerfectFifth.st())
            .cloned()
            .collect()
    }
    if ir.omits.third {
        ir.notes = ir
            .notes
            .iter()
            .filter(|n| {
                n.semitone != Interval::MajorThird.st() && n.semitone != Interval::MinorThird.st()
            })
            .cloned()
            .collect();
    }
}
