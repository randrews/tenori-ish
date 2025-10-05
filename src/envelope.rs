use std::ops::RangeInclusive;
use std::time::Duration;
use eframe::egui;
use eframe::egui::{Context, Id, Label, Slider, Window};
use rodio::{ChannelCount, SampleRate, Source};
use serde::{Deserialize, Serialize};
use crate::gui::Showable;

#[derive(Copy, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Envelope {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub hold: f32,
    pub release: f32
}

impl Default for Envelope {
    fn default() -> Self {
        Self {
            attack: 0.0,
            decay: 0.0,
            sustain: 1.0,
            hold: 0.5,
            release: 0.0,
        }
    }
}

impl Envelope {
    pub fn modulate<S: Source>(&self, source: S) -> EnvelopeSource<S> {
        EnvelopeSource {
            envelope: *self,
            source,
            elapsed: 0,
        }
    }
}

pub struct EnvelopeSource<S: Source> {
    envelope: Envelope,
    source: S,
    elapsed: usize
}

impl<S: Source> Iterator for EnvelopeSource<S> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.source.next() {
            let rate = self.source.sample_rate() as f32; // Number of samples per sec
            // Which tick are we on (a given tick might be more than one call,
            // since multiple channels)
            let mut tick = (self.elapsed / self.source.channels() as usize) as f32;
            // No matter what happens we have now consumed a sample:
            self.elapsed += 1;

            // Attack phase:
            if tick < rate * self.envelope.attack {
                let m = 1.0 / self.envelope.attack;
                return Some(val * tick / rate * m)
            }
            tick -= rate * self.envelope.attack;

            // Decay phase, reduce to sustain level
            if tick < rate * self.envelope.decay {
                let m = (1.0 - self.envelope.sustain) / self.envelope.decay;
                return Some(val * (1.0 - tick / rate * m))
            }
            tick -= rate * self.envelope.decay;

            // Hold phase, hold at sustain level:
            if tick < rate * self.envelope.hold {
                return Some(val * self.envelope.sustain)
            }
            tick -= rate * self.envelope.hold;

            // Release phase, fade to zero:
            if tick < rate * self.envelope.release {
                let m = self.envelope.sustain / self.envelope.release;
                return Some(val * (self.envelope.sustain - tick / rate * m))
            }

            None
        } else {
            None // Inner sample is done so, so are we
        }
    }
}

impl<S: Source> Source for EnvelopeSource<S> {
    fn current_span_len(&self) -> Option<usize> {
        None // Length of the sound
    }

    fn channels(&self) -> ChannelCount {
        self.source.channels()
    }

    fn sample_rate(&self) -> SampleRate {
        self.source.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        // We don't know right here what the duration is, but it will eventually stop
        None
    }
}

impl Showable<(Id, String)> for (&mut Envelope, &mut bool) {
    fn show(&mut self, ctx: &Context, (id, title): &(Id, String)) {
        let mut open = true;
        let window = Window::new(title)
            .id(*id)
            .open(&mut open)
            .resizable([false, false])
            .scroll([false, false]);

        window.show(ctx, |ui| {
            egui::Grid::new(id).show(ui, |ui| {
                ui.add(Label::new("Attack"));
                ui.add(Slider::new(&mut self.0.attack, RangeInclusive::new(0.0, 1.0)));
                ui.end_row();
                ui.add(Label::new("Decay"));
                ui.add(Slider::new(&mut self.0.decay, RangeInclusive::new(0.0, 1.0)));
                ui.end_row();
                ui.add(Label::new("Sustain"));
                ui.add(Slider::new(&mut self.0.sustain, RangeInclusive::new(0.0, 1.0)));
                ui.end_row();
                ui.add(Label::new("Hold"));
                ui.add(Slider::new(&mut self.0.hold, RangeInclusive::new(0.0, 2.0)));
                ui.end_row();
                ui.add(Label::new("Release"));
                ui.add(Slider::new(&mut self.0.release, RangeInclusive::new(0.0, 1.0)));
                ui.end_row();
            });
        });

        *self.1 = open;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ConstSource(f32);
    impl Iterator for ConstSource {
        type Item = f32;
        fn next(&mut self) -> Option<Self::Item> {
            Some(self.0)
        }
    }
    impl Source for ConstSource {
        fn current_span_len(&self) -> Option<usize> {
            None
        }

        fn channels(&self) -> ChannelCount {
            1
        }

        fn sample_rate(&self) -> SampleRate {
            10
        }

        fn total_duration(&self) -> Option<Duration> {
            None
        }
    }

    fn env(attack: f32, decay: f32, sustain: f32, hold: f32, release: f32) -> Envelope {
        Envelope {
            attack, decay, sustain, hold, release
        }
    }

    fn assert_close_enough<S: Source>(mut src: S, expected: Vec<f32>) {
        for e in expected.into_iter() {
            let val = src.next().unwrap();
            assert!(val - e < 0.0001)
        }
    }

    #[test]
    fn test_attack() {
        // Normal: ramp up to 1.0 over 10 samples
        assert_close_enough(
            env(1.0, 0.0, 0.0, 0.0, 0.0).modulate(ConstSource(10.0)),
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

        // 0 attack: immediately at 1.0
        assert_close_enough(
            env(0.0, 0.0, 1.0, 100.0, 0.0).modulate(ConstSource(10.0)),
            vec![10.0; 5]);
    }

    #[test]
    fn test_decay() {
        // Ramp down from 1.0 to sustain:
        assert_close_enough(
            env(0.0, 0.3, 0.7, 100.0, 0.0).modulate(ConstSource(10.0)),
            vec![10.0, 9.0, 8.0, 7.0, 7.0, 7.0]);

        // No decay, immediately at sustain:
        assert_close_enough(
            env(0.0, 0.0, 0.7, 100.0, 0.0).modulate(ConstSource(10.0)),
            vec![7.0; 5]);

        // Sustain at 1, stay there:
        assert_close_enough(
            env(0.0, 0.5, 1.0, 100.0, 0.0).modulate(ConstSource(10.0)),
            vec![10.0; 20]);

        // Attack and then decay:
        // No decay, immediately at sustain:
        assert_close_enough(
            env(0.5, 0.3, 0.7, 100.0, 0.0).modulate(ConstSource(10.0)),
            vec![0.0, 2.0, 4.0, 6.0, 8.0, // 5 samples of attack
                 10.0, 9.0, 8.0, // three sample decay phase
                 7.0, 7.0, 7.0, 7.0]); // stuck at sustain
    }

    #[test]
    fn test_hold() {
        // No A/D, hold at sustain for the hold length:
        assert_close_enough(
            env(0.0, 0.0, 0.7, 1.0, 0.0).modulate(ConstSource(10.0)),
            vec![7.0; 10]);

        // It stops holding after the right length (10 hz, 1.0 secs)
        assert_eq!(
            env(0.0, 0.0, 0.7, 1.0, 0.0).modulate(ConstSource(10.0)).collect::<Vec<_>>().len(),
            10
        );
    }

    #[test]
    fn test_release() {
        // Simple case, release from full
        assert_close_enough(
            env(0.0, 0.0, 1.0, 0.0, 1.0).modulate(ConstSource(10.0)),
            vec![10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);

        // Release from sustain
        assert_close_enough(
            env(0.0, 0.0, 0.5, 0.0, 1.0).modulate(ConstSource(10.0)),
            vec![5.0, 4.5, 4.0, 3.5, 3.0, 2.5, 2.0, 1.5, 1.0, 0.5]);
    }

    #[test]
    fn test_adsr() {
        // The entire envelope
        assert_close_enough(
            env(0.5, 0.3, 0.7, 0.3, 0.7).modulate(ConstSource(10.0)),
            vec![
                0.0, 2.0, 4.0, 6.0, 8.0, // Attack phase
                10.0, 9.0, 8.0, // Decay phase
                7.0, 7.0, 7.0, // Hold at sustain
                7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0 // Release
            ]);
    }
}