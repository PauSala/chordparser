//! Generate a MIDI file from [Chord](chord/struct.Chord.html).

use midly::{
    num::{u4, u7},
    Format, Header, MetaMessage, Smf, Timing, Track, TrackEvent, TrackEventKind,
};

// #[cfg(feature = "midi")]
pub fn to_midi(chord_notes: &[u8], name: &str) {
    // Define the notes for the chord (C major chord: C, E, G)

    // Define the velocity and duration for the notes
    let velocity = u7::new(64); // Note velocity
    let note_duration = 3200; // Double the duration

    // Create the track events for the chord
    let mut events = vec![];

    // Add note-on events
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

    // Add note-off events after the note duration
    for (i, &note) in chord_notes.iter().enumerate() {
        events.push(TrackEvent {
            delta: if i == 0 {
                note_duration.into()
            } else {
                0.into()
            }, // Delay only for the first note-off event
            kind: TrackEventKind::Midi {
                channel: u4::new(0),
                message: midly::MidiMessage::NoteOff {
                    key: u7::new(note),
                    vel: velocity - (3 * i as u8).into(),
                },
            },
        });
    }

    // Add an end-of-track meta event
    events.push(TrackEvent {
        delta: 0.into(),
        kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
    });

    // Create the track
    let mut track = Track::new();
    let tempo = midly::MetaMessage::Tempo(1000.into());
    for event in events {
        track.push(event);
    }
    track.push(TrackEvent {
        delta: 0.into(),
        kind: TrackEventKind::Meta(tempo),
    });
    // Create the SMF (Standard MIDI File) with one track
    let smf = Smf {
        header: Header {
            format: Format::SingleTrack,
            timing: Timing::Metrical(midly::num::u15::new(800)), // 480 ticks per beat
        },
        tracks: vec![track],
    };

    // Write the SMF to a file
    let mut name = name.to_string();
    name.push_str(".mid");
    let mut file = std::fs::File::create(name).unwrap();
    smf.write_std(&mut file).unwrap();
}
