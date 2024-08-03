use super::Chord;

pub fn string_representation(ch: &Chord) -> String {
    let base = ch.root.to_string();
    //let quality = &ch.quality;
    // match quality {
    //     super::quality::Quality::Power => todo!(),
    //     super::quality::Quality::Major => todo!(),
    //     super::quality::Quality::Minor => todo!(),
    //     super::quality::Quality::Dominant => todo!(),
    //     super::quality::Quality::SemiDiminished => todo!(),
    //     super::quality::Quality::Diminished => todo!(),
    //     super::quality::Quality::Augmented => todo!(),
    // }
    base
}

#[cfg(test)]
mod test {
    use crate::parser::Parser;

    use super::*;

    #[test]
    fn should_work() {
        let mut parser = Parser::new();
        let res = parser.parse("Ab7add119b5");
        let res = res.unwrap();
        string_representation(&res);
    }
}
