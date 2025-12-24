use std::path::Path;

/// Parse a chord and generate a both json-string representation and a MIDI file.
pub fn main() {
    let mut parser = Parser::new();
    let origin = "Csus";
    let result = parser.parse(origin);

    match result {
        Ok(chord) => {
            dbg!(&chord);
            println!("{}", chord.to_json());

            let d5_midi_code = 74;
            // Create a voicing with ninth at top
            let midi_codes = generate_voicing(&chord, Some(d5_midi_code));
            // Save the midi file
            to_midi_file(&midi_codes, Path::new("my_chord"), 120, 4);
        }
        Err(e) => {
            for e in e.errors {
                println!("{}", format!("{}", e.verbose_display(origin)));
            }
        }
    }
}

use chordparser::{parsing::Parser, voicings::generate_voicing};
use midly::{
    Format, Header, MetaMessage, Smf, Timing, Track, TrackEvent, TrackEventKind,
    num::{u4, u7},
};

/// Generate a MIDI file from Chord.
/// # Arguments
/// * `chord_notes` - The notes of the chord in MIDI codes.
/// * `name` - The path of the file to save without extension.
/// * `bpm` - Beats per minute.
/// * `beats` - Duration in beats.
pub fn to_midi_file(chord_notes: &[u8], name: &Path, bpm: u32, beats: u16) {
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
                    vel: velocity - (i as u8).into(),
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
                    vel: velocity - (i as u8).into(),
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
