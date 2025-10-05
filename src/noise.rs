use rodio::mixer::Mixer;
use rodio::Source;
use serde::{Deserialize, Serialize};
use crate::envelope::Envelope;

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
    pub envelope: Envelope
}

/// The frequency for a given tone, in Hz.
/// Tone 0 is A4, A above middle C, which is defined at 440 Hz.
/// Moving up or down one is a single semitone, which means a
/// change in `tone` of `12` is one octave.
fn freq(tone: i32) -> f32 {
    440.0 * 1.0595f32.powf(tone as f32)
}

impl NoteType {
    fn source(self, tone: i32) -> Box<dyn Source + Send> {
        let freq = freq(tone);
        let source: Box<dyn Source + Send> = match self {
            NoteType::Sine => Box::new(rodio::source::SineWave::new(freq)),
            NoteType::Triangle => Box::new(rodio::source::TriangleWave::new(freq)),
            NoteType::Sawtooth => Box::new(rodio::source::SawtoothWave::new(freq)),
            NoteType::Square => Box::new(rodio::source::SquareWave::new(freq)),
            NoteType::Noise => Box::new(rodio::source::noise::WhiteUniform::new(44100)
                .low_pass_with_q(freq as u32, 2.0))
        };
        source
    }
}

impl Note {
    pub fn play(self, mixer: &Mixer) {
        let note = self.note_type.source(self.tone);
        let note = self.envelope.modulate(note);
        let note = note.amplify(self.volume.clamp(0.0, 1.0));
        mixer.add(note)
    }
}
