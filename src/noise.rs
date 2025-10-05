use rodio::mixer::Mixer;
use rodio::Source;
use serde::{Deserialize, Serialize};
use crate::timbre::Timbre;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NoteType {
    Sine,
    Triangle,
    Sawtooth,
    Square,
    Noise
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Note {
    /// What kind of timbre
    pub note_type: NoteType,

    /// Which semitone up / down from A4
    pub tone: i32,

    /// How loud, 0.0 .. 2.0
    pub volume: f32,

    /// ADSR envelope
    pub timbre: Timbre
}

/// The frequency for a given tone, in Hz.
/// Tone 0 is A4, A above middle C, which is defined at 440 Hz.
/// Moving up or down one is a single semitone, which means a
/// change in `tone` of `12` is one octave.
fn freq(tone: i32) -> f32 {
    440.0 * 1.0595f32.powf(tone as f32)
}

impl Note {
    pub fn play(self, mixer: &Mixer) {
        let freq = freq(self.tone);
        let source = self.timbre.source(freq);
        let source = source.amplify_normalized(self.volume);
        mixer.add(source)
    }
}
