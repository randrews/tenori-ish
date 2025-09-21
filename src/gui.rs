use std::ops::RangeInclusive;
use eframe::egui;
use eframe::egui::{Context, TopBottomPanel};
use crate::grid::{Grid, NoteType};
use crate::Nome;

impl Nome {
    pub fn menu(&mut self, ctx: &Context) {
        TopBottomPanel::top("menu_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                if ui.button("Sine").clicked() {
                    self.add_window(NoteType::Sine)
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::Slider::new(&mut self.tempo, RangeInclusive::new(20, 180)));
                    if self.playing {
                        if ui.button("||").clicked() { self.playing = false }
                    } else {
                        if ui.button(">").clicked() { self.playing = true }
                    }
                });
            })
        });
    }
}