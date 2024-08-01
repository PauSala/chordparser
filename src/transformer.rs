use crate::chord::{
    chord_ir::ChordIr,
    semantics::{NoteDescriptor, SemInterval},
};

pub type Transformer = fn(&mut ChordIr);

pub fn implicit_third(ir: &mut ChordIr) {
    if !ir.is_sus && !ir.omits.third && !ir.has(SemInterval::Third) {
        ir.notes
            .push(NoteDescriptor::new(SemInterval::Third, 4, usize::MAX));
    }
}

pub fn implicit_fifth(ir: &mut ChordIr) {
    if !ir.omits.five && !ir.has(SemInterval::Fifth) {
        ir.notes
            .push(NoteDescriptor::new(SemInterval::Fifth, 7, usize::MAX));
    }
}

pub fn implicit_min_seventh(ir: &mut ChordIr) {
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
        ir.notes
            .push(NoteDescriptor::new(SemInterval::Seventh, 10, usize::MAX));
    }
}

pub fn implicit_ninth(ir: &mut ChordIr) {
    let add13 = ir.has_add(SemInterval::Thirteenth);
    let t13 = ir.has(SemInterval::Thirteenth);
    let t9 = ir.has(SemInterval::Ninth);

    if !add13 && t13 && !t9 {
        ir.notes
            .push(NoteDescriptor::new(SemInterval::Ninth, 14, usize::MAX))
    }

    let t9 = ir.has(SemInterval::Ninth);
    let a11 = ir.has_add(SemInterval::Eleventh);
    let t11 = ir.has(SemInterval::Eleventh);
    if !a11 && t11 && !t9 {
        ir.notes
            .push(NoteDescriptor::new(SemInterval::Ninth, 14, usize::MAX))
    }
}

pub fn implicit_eleventh(ir: &mut ChordIr) {
    let add13 = ir.has_add(SemInterval::Thirteenth);
    let t13 = ir.has(SemInterval::Thirteenth);
    let t11 = ir.has(SemInterval::Eleventh);

    if !add13 && t13 && !t11 && ir.is_minor() {
        ir.notes
            .push(NoteDescriptor::new(SemInterval::Eleventh, 17, usize::MAX))
    }
}

pub fn remove_omits(ir: &mut ChordIr) {
    if ir.omits.five {
        ir.notes = ir
            .notes
            .iter()
            .filter(|n| n.semitone != 7)
            .cloned()
            .collect()
    }
    if ir.omits.third {
        ir.notes = ir
            .notes
            .iter()
            .filter(|n| n.semitone != 4 && n.semitone != 3)
            .cloned()
            .collect();
    }
}
