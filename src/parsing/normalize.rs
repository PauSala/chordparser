use crate::{chord::c_quality::ChordQuality, parsing::ast::Ast};

impl Ast {
    pub fn normalize(&self) -> String {
        let normalized = String::new();
        let mut intervals = self.intervals.clone();
        if let Some(third) = self.third {
            intervals.push(third);
        }
        intervals.sort();
        dbg!(&intervals);

        let quality: ChordQuality = intervals.as_slice().into();
        dbg!(quality);
        normalized
    }
}
