use std::fs;
use std::ops::RangeInclusive;
use eframe::egui;
use eframe::egui::{Context, TopBottomPanel};
use crate::noise::NoteType;
use crate::saveload::PersistedTenori;
use crate::Tenori;

impl Tenori {
    pub fn menu(&mut self, ctx: &Context) {
        TopBottomPanel::top("menu_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() &&
                        let Ok(serialized) = toml::to_string(&PersistedTenori::from(&*self)) &&
                        let Some(path) = rfd::FileDialog::new().add_filter("Tenori files", &["tenori"]).set_file_name("song.tenori").save_file() {
                        match fs::write(path, serialized) {
                            Ok(_) => {}
                            Err(e) => { dbg!(e); }
                        }
                    }

                    if ui.button("Load").clicked() &&
                        let Some(path) = rfd::FileDialog::new().add_filter("Tenori files", &["tenori"]).pick_file() &&
                        let Ok(serialized) = fs::read_to_string(path) &&
                        let Ok(persisted) = toml::from_str::<PersistedTenori>(serialized.as_str()) {
                        self.grids = persisted.grids;
                        self.tempo = persisted.tempo;
                        self.playing = false;
                        self.timer = 0.0;
                    }
                });

                ui.menu_button("Add track", |ui| {
                    if ui.button("Square").clicked() {
                        self.add_window(NoteType::Square)
                    }
                    if ui.button("Sine").clicked() {
                        self.add_window(NoteType::Sine)
                    }
                    if ui.button("Triangle").clicked() {
                        self.add_window(NoteType::Triangle)
                    }
                    if ui.button("Sawtooth").clicked() {
                        self.add_window(NoteType::Sawtooth)
                    }
                    if ui.button("Noise").clicked() {
                        self.add_window(NoteType::Noise)
                    }
                });

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