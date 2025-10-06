use std::ops::RangeInclusive;
use std::time::Duration;
use eframe::egui;
use eframe::egui::{Checkbox, Context, Id, Label, Slider, Window};
use rodio::mixer::MixerSource;
use rodio::Source;
use rodio::source::{SawtoothWave, SineWave, SquareWave, TriangleWave, WhiteUniform};
use serde::{Deserialize, Serialize};
use crate::envelope::Envelope;
use crate::gui::Showable;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Timbre {
    sine: f32,
    triangle: f32,
    square: f32,
    sawtooth: f32,
    noise: f32,
    d_gain: f32,
    d_thresh: f32,
    reverb: f32,
    reverb_duration: f32,
    filter: bool,
    filter_q: f32,
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
            d_gain: 1.0,
            d_thresh: 10.0,
            reverb: 0.0,
            reverb_duration: 0.0,
            filter: false,
            filter_q: 50.0,
            envelope: Default::default(),
        }
    }
}

impl Timbre {
    pub fn source(self, frequency: f32) -> impl Source {
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
        let source = source.distortion(self.d_gain, self.d_thresh);
        let source = source.buffered().reverb(Duration::from_millis((self.reverb_duration * 1000.0) as u64), self.reverb);

        self.envelope.modulate(source)
        // let (mixer2, m2source) = rodio::mixer::mixer(1, 44100);
        // if self.filter {
        //     // let source = source.low_pass((frequency * self.filter_q) as u32);
        //     // mixer2.add(self.envelope.modulate(source));
        // } else {
        //     mixer2.add(self.envelope.modulate(source));
        // }
        // m2source
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

                ui.add(Label::new("Dist. Gain"));
                ui.add(Slider::new(&mut self.0.d_gain, RangeInclusive::new(-10.0, 10.0)));
                ui.add(Label::new("Reverb"));
                ui.add(Slider::new(&mut self.0.reverb, RangeInclusive::new(0.0, 1.0)));
                ui.end_row();

                ui.add(Label::new("Dist. Thresh"));
                ui.add(Slider::new(&mut self.0.d_thresh, RangeInclusive::new(0.0, 10.0)));
                ui.add(Label::new("Rev. Duration"));
                ui.add(Slider::new(&mut self.0.reverb_duration, RangeInclusive::new(0.0, 1.0)));
                ui.end_row();

                // ui.add(Checkbox::new(&mut self.0.filter, "Filter"));
                // ui.end_row();
                // ui.add(Label::new("Bandwidth"));
                // ui.add_enabled(self.0.filter, Slider::new(&mut self.0.filter_q, RangeInclusive::new(0.0, 100.0)));
            });
        });

        *self.1 = open;
    }
}