use std::fs;
use std::ops::RangeInclusive;
use std::path::Path;
use eframe::egui;
use eframe::egui::{Context, Id, TopBottomPanel};
use crate::grid::Grid;
use crate::saveload::PersistedTenori;
use crate::Tenori;

/// A trait for things that can be shown in a gui, given a Context.
pub trait Showable<T> {
    fn show(&mut self, ctx: &Context, state: &T);
}

impl Tenori {
    fn menu(&mut self, ctx: &Context) {
        TopBottomPanel::top("menu_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Load").clicked() && let Err(s) = self.load_from_file() {
                        self.dialogs.push(s.into())
                    }

                    if ui.add_enabled(self.default_filename.is_some(), egui::Button::new("Save")).clicked() {
                        let path = self.default_filename.as_ref().unwrap_or_else(|| unreachable!());
                        if let Err(s) = self.save_to_file(path) {
                            self.dialogs.push(s.into())
                        }
                    }

                    if ui.button("Save As...").clicked() && let Err(s) = self.save_as() {
                        self.dialogs.push(s.into())
                    }
                });

                if ui.button("Add track").clicked() {
                    let id = self.window_id();
                    self.grids.push(Grid::new(id));
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::Slider::new(&mut self.tempo, RangeInclusive::new(20, 180)));
                    if self.playing {
                        if ui.button("||").clicked() { self.playing = false }
                    } else if ui.button(">").clicked() { self.playing = true }
                });
            })
        });
    }

    pub fn display_dialogs(&mut self, ctx: &Context) {
        for d in self.dialogs.iter_mut() {
            d.show(ctx, &());
        }
        self.dialogs.retain(|d| d.1);
    }

    fn save_as(&self) -> Result<(), String> {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Tenori files", &["tenori"])
            .set_file_name("song.tenori").save_file() {
            return self.save_to_file(path)
        }
        Ok(())
    }

    fn save_to_file<P: AsRef<Path>>(&self, filename: P) -> Result<(), String> {
        let serialized = toml::to_string(&PersistedTenori::from(self)).map_err(|e| e.to_string())?;
        fs::write(filename, serialized).map_err(|e| e.to_string())
    }

    fn load_from_file(&mut self) -> Result<(), String> {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Tenori files", &["tenori"])
            .pick_file() {
            let serialized = fs::read_to_string(path).map_err(|e| e.to_string())?;
            let persisted = toml::from_str::<PersistedTenori>(serialized.as_str()).map_err(|e| e.to_string())?;
            persisted.apply_to(self);
        }
        Ok(())
    }

    fn display_grids(&mut self, ctx: &Context, cursor: &f32) {
        for g in self.grids.iter_mut() {
            g.show(ctx, cursor)
        }
        self.grids.retain(|g| g.open);
    }

    /// Return a unique (among all the windows created since a file load) id string for a window.
    pub fn window_id(&mut self) -> Id {
        self.window_counter += 1;
        format!("window {}", self.window_counter).into()
    }
}

impl Showable<f32> for Tenori {
    fn show(&mut self, ctx: &Context, cursor: &f32) {
        self.menu(ctx);
        self.display_grids(ctx, cursor);
        self.display_dialogs(ctx);
    }
}