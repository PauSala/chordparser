use crate::{
    chord::{
        c_quality::{ChordQuality, PitchClass, PitchClassSet},
        intervals::IntervalSet,
    },
    parsing::ast::Ast,
};

impl Ast {
    pub fn normalize(&self) -> String {
        let normalized = String::new();
        let intervals_slice = self.intervals.as_slice();
        let mut virtual_set: PitchClassSet = intervals_slice.into();

        // This is that in case of an omited third the quality can still be derived as Major or Minor.
        if let Some(third) = self.third {
            virtual_set.insert(Into::<PitchClass>::into(&third));
        }
        let quality: ChordQuality = (&virtual_set).into();
        let is_sus = quality.is_sus(&intervals_slice.into());
        let interval_set = &IntervalSet::from_slice(intervals_slice);
        let alterations = quality.alterations(interval_set);
        let extensions = quality.extensions(interval_set);

        dbg!(quality);
        dbg!(is_sus);
        dbg!(alterations);
        dbg!(extensions);

        normalized
    }
}
