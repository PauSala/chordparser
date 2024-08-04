use crate::chord::{
    chord_ir::ChordIr,
    intervals::{Interval, SemInterval},
    note::NoteDescriptor,
};

pub(crate) type Transformer = fn(&mut ChordIr);

pub(crate) fn implicit_third(ir: &mut ChordIr) {
    if !ir.is_sus && !ir.omits.third && !ir.has_sem_int(SemInterval::Third) {
        ir.notes
            .push(NoteDescriptor::new(Interval::MajorThird, usize::MAX));
    }
}

pub(crate) fn implicit_fifth(ir: &mut ChordIr) {
    if !ir.omits.five
        && !ir.has_sem_int(SemInterval::Fifth)
        && !ir.has_int(Interval::FlatThirteenth)
    {
        ir.notes
            .push(NoteDescriptor::new(Interval::PerfectFifth, usize::MAX));
    }
}

pub(crate) fn implicit_min_seventh(ir: &mut ChordIr) {
    let tensions_len = ir
        .notes
        .iter()
        .filter(|n| {
            matches!(
                n.interval,
                Interval::Ninth | Interval::Eleventh | Interval::Thirteenth | Interval::MajorSixth
            ) && !ir.adds.contains(&n.interval)
        })
        .collect::<Vec<&NoteDescriptor>>()
        .len();
    let t7 = ir.has_sem_int(SemInterval::Seventh);
    let is_add_6 = !ir
        .adds
        .iter()
        .filter(|a| a.to_semantic_interval() == SemInterval::Sixth)
        .collect::<Vec<_>>()
        .is_empty();

    // Implicit seventh is only set when there are natural tensions not comming from an add modifier
    // and a sixth has not been set or is not an add.
    if !t7 && tensions_len > 0 && (!ir.has_sem_int(SemInterval::Sixth) || is_add_6) {
        ir.notes
            .push(NoteDescriptor::new(Interval::MinorSeventh, usize::MAX));
    }
}

pub(crate) fn implicit_ninth(ir: &mut ChordIr) {
    let add13 = ir.has_add(Interval::Thirteenth);
    let t13 = ir.has_int(Interval::Thirteenth);
    let t9 = ir.has_sem_int(SemInterval::Ninth);

    if !add13 && t13 && !t9 {
        ir.notes
            .push(NoteDescriptor::new(Interval::Ninth, usize::MAX))
    }

    let t11 = ir.has_sem_int(SemInterval::Eleventh);
    let a11 = ir.has_add(Interval::Eleventh);
    let t9 = ir.has_sem_int(SemInterval::Ninth);
    if !a11 && t11 && !t9 {
        ir.notes
            .push(NoteDescriptor::new(Interval::Ninth, usize::MAX))
    }
}

pub(crate) fn implicit_eleventh(ir: &mut ChordIr) {
    let add13 = ir.has_add(Interval::Thirteenth);
    let t13 = ir.has_int(Interval::Thirteenth);
    let t11 = ir.has_sem_int(SemInterval::Eleventh);

    if !add13 && t13 && !t11 && ir.has_int(Interval::MinorThird) {
        ir.notes
            .push(NoteDescriptor::new(Interval::Eleventh, usize::MAX))
    }
}

pub(crate) fn remove_omits(ir: &mut ChordIr) {
    if ir.omits.five {
        ir.notes = ir
            .notes
            .iter()
            .filter(|n| n.interval.st() != Interval::PerfectFifth.st())
            .cloned()
            .collect()
    }
    if ir.omits.third {
        ir.notes = ir
            .notes
            .iter()
            .filter(|n| {
                n.interval.st() != Interval::MajorThird.st()
                    && n.interval.st() != Interval::MinorThird.st()
            })
            .cloned()
            .collect();
    }
}
