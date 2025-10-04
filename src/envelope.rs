use std::ops::RangeInclusive;
use eframe::egui;
use eframe::egui::{Context, Id, Label, Slider, Window};
use serde::{Deserialize, Serialize};
use crate::gui::Showable;

#[derive(Copy, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Envelope {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32
}

impl Default for Envelope {
    fn default() -> Self {
        Self {
            attack: 0.0,
            decay: 0.0,
            sustain: 1.0,
            release: 0.0,
        }
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
                ui.add(Label::new("Release"));
                ui.add(Slider::new(&mut self.0.release, RangeInclusive::new(0.0, 1.0)));
                ui.end_row();
            });
        });

        *self.1 = open;
    }
}