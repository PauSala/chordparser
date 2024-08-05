use std::path::Path;

use chordparser::parser::Parser;
/// Parse a chord and generate a both json-string representation and the  MIDI file.
pub fn main() {
    let mut parser = Parser::new();
    let result = parser.parse("AbMaj7#5sus");
    match result {
        Ok(chord) => {
            dbg!(&chord);
            dbg!(&chord.to_json());
            to_midi_file(
                &chord.to_midi_codes(),
                Path::new("midi_files/Ab°7(Maj7,9)"),
                120,
                4,
            );
        }
        Err(e) => {
            dbg!(e);
        }
    }
}

use midly::{
    num::{u4, u7},
    Format, Header, MetaMessage, Smf, Timing, Track, TrackEvent, TrackEventKind,
};

/// Generate a MIDI file from [Chord](chord/struct.Chord.html).
/// # Arguments
/// * `chord_notes` - The notes of the chord in MIDI codes.
/// * `name` - The path of the file to save without extension.
/// * `bpm` - Beats per minute.
/// * `beats` - Duration in beats.
// #[cfg(feature = "midi")]
pub fn to_midi_file(chord_notes: &[u8], name: &Path, bpm: u32, beats: u16) {
    // Create the track events for the chord
    let mc_x_beat = 60 * 1_000_000 / bpm;
    let ticks_per_beat: u16 = 500;
    let ticks_per_quarter = ticks_per_beat * beats;
    let velocity = u7::new(64);
    let note_duration = ticks_per_quarter;
    let mut events = vec![];
    let tempo = midly::MetaMessage::Tempo(mc_x_beat.into());
    events.push(TrackEvent {
        delta: 0.into(),
        kind: TrackEventKind::Meta(tempo),
    });

    // Start chord
    for (i, &note) in chord_notes.iter().enumerate() {
        events.push(TrackEvent {
            delta: 0.into(), // No delay between note-on events
            kind: TrackEventKind::Midi {
                channel: u4::new(0),
                message: midly::MidiMessage::NoteOn {
                    key: u7::new(note),
                    vel: velocity - (3 * i as u8).into(),
                },
            },
        });
    }

    // Stop chord after duration
    for (i, &note) in chord_notes.iter().enumerate() {
        events.push(TrackEvent {
            delta: if i == 0 {
                (note_duration as u32).into()
            } else {
                0.into()
            },
            kind: TrackEventKind::Midi {
                channel: u4::new(0),
                message: midly::MidiMessage::NoteOff {
                    key: u7::new(note),
                    vel: velocity - (3 * i as u8).into(),
                },
            },
        });
    }

    events.push(TrackEvent {
        delta: 0.into(),
        kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
    });

    let mut track = Track::new();
    for event in events {
        track.push(event);
    }
    let smf = Smf {
        header: Header {
            format: Format::SingleTrack,
            timing: Timing::Metrical(midly::num::u15::new(ticks_per_beat)),
        },
        tracks: vec![track],
    };

    let path = name.with_extension("mid");
    let mut file = std::fs::File::create(path).unwrap();
    smf.write_std(&mut file).unwrap();
}
