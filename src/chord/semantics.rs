use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NoteLiteral {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl NoteLiteral {
    pub fn from_string(i: &str) -> NoteLiteral {
        match i {
            "C" => NoteLiteral::C,
            "D" => NoteLiteral::D,
            "E" => NoteLiteral::E,
            "F" => NoteLiteral::F,
            "G" => NoteLiteral::G,
            "A" => NoteLiteral::A,
            "B" => NoteLiteral::B,
            _ => panic!("Unknown note literal"),
        }
    }

    pub(crate) fn to_int(l: &NoteLiteral) -> u8 {
        match l {
            NoteLiteral::C => 0,
            NoteLiteral::D => 1,
            NoteLiteral::E => 2,
            NoteLiteral::F => 3,
            NoteLiteral::G => 4,
            NoteLiteral::A => 5,
            NoteLiteral::B => 6,
        }
    }

    pub(crate) fn get_matcher(&self, root: u8, interval: u8) -> NoteMatcher {
        let interval = (root + interval) % 12;
        let i = interval % 12;
        match i {
            0 => NoteMatcher {
                matchers: vec![
                    (NoteLiteral::C, None),
                    (NoteLiteral::B, Some(Modifier::Sharp)),
                    (NoteLiteral::D, Some(Modifier::DFlat)),
                ],
            },
            1 => NoteMatcher {
                matchers: vec![
                    (NoteLiteral::D, Some(Modifier::Flat)),
                    (NoteLiteral::C, Some(Modifier::Sharp)),
                ],
            },
            2 => NoteMatcher {
                matchers: vec![
                    (NoteLiteral::D, None),
                    (NoteLiteral::C, Some(Modifier::DSharp)),
                    (NoteLiteral::E, Some(Modifier::DFlat)),
                ],
            },
            3 => NoteMatcher {
                matchers: vec![
                    (NoteLiteral::E, Some(Modifier::Flat)),
                    (NoteLiteral::D, Some(Modifier::Sharp)),
                ],
            },
            4 => NoteMatcher {
                matchers: vec![
                    (NoteLiteral::E, None),
                    (NoteLiteral::F, Some(Modifier::Flat)),
                    (NoteLiteral::D, Some(Modifier::DSharp)),
                ],
            },
            5 => NoteMatcher {
                matchers: vec![
                    (NoteLiteral::F, None),
                    (NoteLiteral::E, Some(Modifier::Sharp)),
                    (NoteLiteral::G, Some(Modifier::DFlat)),
                ],
            },
            6 => NoteMatcher {
                matchers: vec![
                    (NoteLiteral::G, Some(Modifier::Flat)),
                    (NoteLiteral::F, Some(Modifier::Sharp)),
                ],
            },
            7 => NoteMatcher {
                matchers: vec![
                    (NoteLiteral::G, None),
                    (NoteLiteral::F, Some(Modifier::DSharp)),
                    (NoteLiteral::A, Some(Modifier::DFlat)),
                ],
            },
            8 => NoteMatcher {
                matchers: vec![
                    (NoteLiteral::A, Some(Modifier::Flat)),
                    (NoteLiteral::G, Some(Modifier::Sharp)),
                ],
            },
            9 => NoteMatcher {
                matchers: vec![
                    (NoteLiteral::A, None),
                    (NoteLiteral::G, Some(Modifier::DSharp)),
                    (NoteLiteral::B, Some(Modifier::DFlat)),
                ],
            },
            10 => NoteMatcher {
                matchers: vec![
                    (NoteLiteral::B, Some(Modifier::Flat)),
                    (NoteLiteral::A, Some(Modifier::Sharp)),
                ],
            },
            11 => NoteMatcher {
                matchers: vec![
                    (NoteLiteral::B, None),
                    (NoteLiteral::A, Some(Modifier::DSharp)),
                    (NoteLiteral::C, Some(Modifier::Flat)),
                ],
            },
            _ => panic!("Dont call this with a number greater than 6"),
        }
    }
}

impl Display for NoteLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoteLiteral::C => f.write_str("C"),
            NoteLiteral::D => f.write_str("D"),
            NoteLiteral::E => f.write_str("E"),
            NoteLiteral::F => f.write_str("F"),
            NoteLiteral::G => f.write_str("G"),
            NoteLiteral::A => f.write_str("A"),
            NoteLiteral::B => f.write_str("B"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NoteMatcher {
    pub(crate) matchers: Vec<(NoteLiteral, Option<Modifier>)>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Modifier {
    Sharp,
    Flat,
    DSharp,
    DFlat,
}

impl Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Modifier::Sharp => f.write_str("#"),
            Modifier::Flat => f.write_str("b"),
            Modifier::DSharp => f.write_str("𝄪"),
            Modifier::DFlat => f.write_str("𝄫"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Note {
    pub literal: NoteLiteral,
    pub modifier: Option<Modifier>,
}

impl Note {
    pub fn new(literal: NoteLiteral, modifier: Option<Modifier>) -> Note {
        Note { literal, modifier }
    }

    fn get_difference(&self, to: &Note) -> u8 {
        let o = self.to_interval();
        let n = to.to_interval();
        (n + 12 - o) % 12
    }

    pub fn transpose_to(&self, note: &Note, to: &Note) -> Note {
        let diff = self.get_difference(to);
        dbg!(diff, (note.to_interval() + diff) % 12);
        let m = self.literal.get_matcher(note.to_interval(), diff);
        Note::new(m.matchers[0].clone().0, m.matchers[0].clone().1)
    }

    pub fn to_real_interval(semantic_interval: u8, interval: u8) -> String {
        match semantic_interval {
            1 => String::from("1"),
            2 => match interval {
                1 => String::from("b2"),
                2 => String::from("2"),
                _ => "".to_owned(),
            },
            3 => match interval {
                3 => String::from("b3"),
                4 => String::from("3"),
                _ => "".to_owned(),
            },
            4 => match interval {
                5 => String::from("4"),
                6 => String::from("#4"),
                _ => "".to_owned(),
            },
            5 => match interval {
                6 => String::from("b5"),
                7 => String::from("5"),
                8 => String::from("#5"),
                _ => "".to_owned(),
            },
            6 => match interval {
                8 => String::from("b6"),
                9 => String::from("6"),
                _ => "".to_owned(),
            },
            7 => match interval {
                9 => String::from("bb7"),
                10 => String::from("b7"),
                11 => String::from("Maj7"),
                _ => "".to_owned(),
            },
            9 => match interval {
                13 =>  String::from("b9"),
                14 => String::from("9"),
                15 => String::from("#9"),
                _ => "".to_owned(),
            },

            11 => match interval {
                17 => String::from("11"),
                18 => String::from("#11"),
                _ => "".to_owned(),
            },

            13 => match interval {
                20 => String::from("b13"),
                21 => String::from("13"),
                _ => "".to_owned(),
            },
            _ => "".to_owned(),
        }
    }

    /// Interval relative to 0
    pub fn to_interval(&self) -> u8 {
        match self.literal {
            NoteLiteral::C => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 1,
                    Modifier::Flat => 11,
                    _ => panic!("Double flat/sharp roots are not allowed"),
                },
                None => 0,
            },
            NoteLiteral::D => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 3,
                    Modifier::Flat => 1,
                    _ => panic!("Double flat/sharp roots are not allowed"),
                },
                None => 2,
            },
            NoteLiteral::E => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 5,
                    Modifier::Flat => 3,
                    _ => panic!("Double flat/sharp roots are not allowed"),
                },
                None => 4,
            },
            NoteLiteral::F => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 6,
                    Modifier::Flat => 4,
                    _ => panic!("Double flat/sharp roots are not allowed"),
                },
                None => 5,
            },
            NoteLiteral::G => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 8,
                    Modifier::Flat => 6,
                    _ => panic!("Double flat/sharp roots are not allowed"),
                },
                None => 7,
            },
            NoteLiteral::A => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 10,
                    Modifier::Flat => 8,
                    _ => panic!("Double flat/sharp roots are not allowed"),
                },
                None => 9,
            },
            NoteLiteral::B => match &self.modifier {
                Some(m) => match m {
                    Modifier::Sharp => 0,
                    Modifier::Flat => 10,
                    _ => panic!("Double flat/sharp roots are not allowed"),
                },
                None => 11,
            },
        }
    }

    pub fn get_note(&self, exact_interval: u8, semantic_interval: u8) -> Note {
        let m = self.literal.get_matcher(self.to_interval(), exact_interval);
        let root_index = NoteLiteral::to_int(&self.literal);
        let interval_index = (root_index + (semantic_interval - 1)) % 7;
        let f = m
            .matchers
            .iter()
            .find(|m| NoteLiteral::to_int(&m.0) == interval_index);
        let (literal, modifier) = f.unwrap().to_owned();
        Note::new(literal, modifier)
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m = match &self.modifier {
            Some(m) => m.to_string(),
            None => "".to_owned(),
        };
        f.write_str(&format!("{}{}", self.literal, m))?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SemInterval {
    Root,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Ninth,
    Eleventh,
    Thirteenth,
}

impl SemInterval {
    pub fn to_int(&self) -> u8 {
        match self {
            SemInterval::Root => 1,
            SemInterval::Second => 2,
            SemInterval::Third => 3,
            SemInterval::Fourth => 4,
            SemInterval::Fifth => 5,
            SemInterval::Sixth => 6,
            SemInterval::Seventh => 7,
            SemInterval::Ninth => 9,
            SemInterval::Eleventh => 11,
            SemInterval::Thirteenth => 13,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NoteDescriptor {
    pub sem_interval: SemInterval,
    pub semitone: u8,
    pub pos: usize,
}

impl NoteDescriptor {
    pub(crate) fn new(sem_interval: SemInterval, semitone: u8, pos: usize) -> NoteDescriptor {
        NoteDescriptor {
            sem_interval,
            semitone,
            pos,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::chord::semantics::Modifier;

    use super::Note;

    #[test]
    fn transpose() {
        let notea = Note::new(super::NoteLiteral::G, Some(Modifier::Sharp));
        let noteb = Note::new(super::NoteLiteral::D, Some(Modifier::Sharp));
        let notec = Note::new(super::NoteLiteral::D, Some(Modifier::Sharp));
        dbg!(notea.transpose_to(&noteb, &notec));
    }
}
