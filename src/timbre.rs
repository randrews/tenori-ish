use std::ops::RangeInclusive;
use eframe::egui;
use eframe::egui::{Context, Id, Label, Slider, Window};
use rodio::mixer::MixerSource;
use rodio::Source;
use rodio::source::{SawtoothWave, SineWave, SquareWave, TriangleWave, WhiteUniform};
use serde::{Deserialize, Serialize};
use crate::envelope::{Envelope, EnvelopeSource};
use crate::gui::Showable;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Timbre {
    sine: f32,
    triangle: f32,
    square: f32,
    sawtooth: f32,
    noise: f32,
    envelope: Envelope
}

impl Default for Timbre {
    fn default() -> Self {
        Self {
            sine: 0.0,
            triangle: 0.0,
            square: 1.0,
            sawtooth: 0.0,
            noise: 0.0,
            envelope: Default::default(),
        }
    }
}

impl Timbre {
    pub fn source(&self, frequency: f32) -> EnvelopeSource<MixerSource> {
        let (mixer, source) = rodio::mixer::mixer(1, 44100);
        if self.sine > 0.0 {
            mixer.add(SineWave::new(frequency).amplify(self.sine))
        }
        if self.triangle > 0.0 {
            mixer.add(TriangleWave::new(frequency).amplify(self.triangle))
        }
        if self.square > 0.0 {
            mixer.add(SquareWave::new(frequency).amplify(self.square))
        }
        if self.sawtooth > 0.0 {
            mixer.add(SawtoothWave::new(frequency).amplify(self.sawtooth))
        }
        if self.noise > 0.0 {
            mixer.add(WhiteUniform::new(44100).amplify(self.noise))
        }
        self.envelope.modulate(source)
    }
}

impl Showable<(Id, String)> for (&mut Timbre, &mut bool, &mut String) {
    fn show(&mut self, ctx: &Context, (id, title): &(Id, String)) {
        let mut open = true;
        let window = Window::new(title)
            .id(*id)
            .open(&mut open)
            .resizable([false, false])
            .scroll([false, false]);

        window.show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(self.2);
            });

            egui::Grid::new(id).show(ui, |ui| {
                ui.add(Label::new("Sine"));
                ui.add(Slider::new(&mut self.0.sine, RangeInclusive::new(0.0, 1.0)));
                ui.add(Label::new("Attack"));
                ui.add(Slider::new(&mut self.0.envelope.attack, RangeInclusive::new(0.0, 1.0)));
                ui.end_row();

                ui.add(Label::new("Triangle"));
                ui.add(Slider::new(&mut self.0.triangle, RangeInclusive::new(0.0, 1.0)));
                ui.add(Label::new("Decay"));
                ui.add(Slider::new(&mut self.0.envelope.decay, RangeInclusive::new(0.0, 1.0)));
                ui.end_row();

                ui.add(Label::new("Square"));
                ui.add(Slider::new(&mut self.0.square, RangeInclusive::new(0.0, 1.0)));
                ui.add(Label::new("Sustain"));
                ui.add(Slider::new(&mut self.0.envelope.sustain, RangeInclusive::new(0.0, 1.0)));
                ui.end_row();

                ui.add(Label::new("Sawtooth"));
                ui.add(Slider::new(&mut self.0.sawtooth, RangeInclusive::new(0.0, 1.0)));
                ui.add(Label::new("Hold"));
                ui.add(Slider::new(&mut self.0.envelope.hold, RangeInclusive::new(0.0, 2.0)));
                ui.end_row();

                ui.add(Label::new("Noise"));
                ui.add(Slider::new(&mut self.0.noise, RangeInclusive::new(0.0, 1.0)));
                ui.add(Label::new("Release"));
                ui.add(Slider::new(&mut self.0.envelope.release, RangeInclusive::new(0.0, 1.0)));
                ui.end_row();
            });
        });

        *self.1 = open;
    }
}