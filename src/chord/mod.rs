//! # Chords, notes and intervals
use crate::chord::{note::Note, quality::ChordQuality};
use interval::Interval;
use serde::{Deserialize, Serialize};
use serde_json;
use std::vec;
pub mod interval;
pub mod note;
pub mod quality;

/// Chord representation of a successfully parsed string.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Chord {
    /// The string that originated the chord.
    pub origin: String,
    /// The descriptor of the chord (all beyond its root).
    pub descriptor: String,
    pub normalized: String,
    /// The root note of the chord.
    pub root: Note,
    /// The bass note of the chord if any is added with a slash.
    pub bass: Option<Note>,
    /// The notes of the chord.
    pub notes: Vec<Note>,
    /// The semitones of the notes relative to root.
    pub semitones: Vec<u8>,
    /// The real intervals of the notes.
    pub intervals: Vec<Interval>,
    pub quality: ChordQuality,
    /// The normalized intervals of the notes, used to normalize the name
    #[serde(skip_serializing)]
    pub(crate) normalized_intervals: Vec<Interval>,
    /// Interval degrees of the notes
    #[serde(skip_serializing)]
    interval_degrees: Vec<u8>,
}

impl Chord {
    pub fn builder(origin: &str, root: Note) -> ChordBuilder {
        ChordBuilder::new(origin, root)
    }

    pub fn note_literals(&self) -> impl Iterator<Item = String> {
        self.notes.iter().map(|n| n.to_string())
    }

    /// Transposes the chord to a different root note.
    /// # Arguments
    /// * `self` - The chord to transpose.
    /// * `transpose_to` - The note to transpose the chord to.
    /// # Returns
    /// * A new chord transposed to the new root note.
    pub fn transpose(&self, transpose_to: &Note) -> Chord {
        let bass = self
            .bass
            .as_ref()
            .map(|bass| self.root.transpose(bass, transpose_to));

        let mut notes = Vec::new();
        let semitones = self.semitones.clone();
        let interval_degrees = self.interval_degrees.clone();

        for (st, sem_int) in semitones.iter().zip(&interval_degrees) {
            let note = transpose_to.get_note(*st, *sem_int);
            notes.push(note);
        }

        let mut origin = transpose_to.to_string();

        // Set origin string
        // TODO: this is for chords like C##5, which when transposed to D gives D#5, which is interpreted as D#(5));
        if self.descriptor.starts_with("#5") {
            let desc = &self.descriptor[0..2].replace("#5", "+5");
            origin.push_str(desc);
        } else {
            origin.push_str(&self.descriptor);
        }

        Chord::builder(&origin, *transpose_to)
            .descriptor(&self.descriptor)
            .bass(bass)
            .notes(notes)
            .semitones(semitones)
            .interval_degrees(interval_degrees)
            .normalized_intervals(self.normalized_intervals.clone())
            .intervals(self.intervals.clone())
            .build()
    }

    /// Returns the MIDI codes for the chord, centered around central C (60 midi code).
    /// # Arguments
    /// * `self` - The chord to get the MIDI codes from.
    /// # Returns
    /// * A vector of MIDI codes.
    pub fn to_midi_codes(&self) -> Vec<u8> {
        let root = self.root.to_midi_code();
        let mut codes = vec![];
        if let Some(bass) = &self.bass {
            codes.push(bass.to_midi_code() - 12);
            codes.push(root);
        } else {
            codes.push(root - 12);
        }
        for note in self.normalized_intervals.iter().skip(1) {
            codes.push(note.st() + root);
        }
        codes
    }

    /// Returns the JSON representation of the chord.
    /// # Arguments
    /// * `self` - The chord to get the JSON representation from.
    /// # Returns
    /// * A JSON string.
    pub fn to_json(&self) -> String {
        let a = serde_json::to_string(self);
        match a {
            Ok(v) => v,
            Err(_) => "{{}}".to_string(),
        }
    }
}

/// Builder for the Chord struct.
pub struct ChordBuilder {
    origin: String,
    normalized: String,
    descriptor: String,
    root: Note,
    bass: Option<Note>,
    notes: Vec<Note>,
    semitones: Vec<u8>,
    interval_degrees: Vec<u8>,
    intervals: Vec<Interval>,
    normalized_intervals: Vec<Interval>,
    quality: ChordQuality,
}

impl ChordBuilder {
    pub fn new(origin: &str, root: Note) -> ChordBuilder {
        ChordBuilder {
            origin: origin.to_string(),
            normalized: "".to_string(),
            descriptor: String::new(),
            root,
            bass: None,
            notes: Vec::new(),
            semitones: Vec::new(),
            interval_degrees: Vec::new(),
            normalized_intervals: Vec::new(),
            intervals: Vec::new(),
            quality: Default::default(),
        }
    }

    pub fn quality(mut self, quality: ChordQuality) -> ChordBuilder {
        self.quality = quality;
        self
    }

    pub fn intervals(mut self, real_intervals: Vec<Interval>) -> ChordBuilder {
        self.intervals = real_intervals;
        self
    }

    pub fn normalized_intervals(mut self, normalized_intervals: Vec<Interval>) -> ChordBuilder {
        self.normalized_intervals = normalized_intervals;
        self
    }

    pub fn interval_degrees(mut self, semantic_intervals: Vec<u8>) -> ChordBuilder {
        self.interval_degrees = semantic_intervals;
        self
    }

    pub fn semitones(mut self, semitones: Vec<u8>) -> ChordBuilder {
        self.semitones = semitones;
        self
    }

    pub fn notes(mut self, notes: Vec<Note>) -> ChordBuilder {
        self.notes = notes;
        self
    }

    pub fn bass(mut self, bass: Option<Note>) -> ChordBuilder {
        self.bass = bass;
        self
    }

    pub fn descriptor(mut self, descriptor: &str) -> ChordBuilder {
        self.descriptor = descriptor.to_string();
        self
    }

    pub fn normalized(mut self, normalized: String) -> ChordBuilder {
        self.normalized = normalized;
        self
    }

    pub fn build(self) -> Chord {
        Chord {
            origin: self.origin,
            descriptor: self.descriptor,
            root: self.root,
            bass: self.bass,
            notes: self.notes,
            interval_degrees: self.interval_degrees,
            intervals: self.intervals,
            normalized_intervals: self.normalized_intervals,
            semitones: self.semitones,
            normalized: self.normalized,
            quality: self.quality,
        }
    }
}
