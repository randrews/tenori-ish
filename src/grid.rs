use std::ops::RangeInclusive;
use eframe::egui;
use eframe::egui::{Color32, Context, Id, PointerButton, Pos2, Rangef, RichText, Sense, Ui, Vec2, Widget};
use serde::{Deserialize, Serialize};
use crate::noise::NoteType;
use crate::scale::Scale;
use crate::tenori::LOOP_LENGTH;

impl NoteType {
    fn color(&self) -> Color32 {
        match self {
            NoteType::Sine => Color32::from_rgb(3, 211, 252),
            NoteType::Triangle => Color32::from_rgb(252, 202, 3),
            NoteType::Sawtooth => Color32::from_rgb(252, 18, 61),
            NoteType::Square => Color32::from_rgb(4, 219, 51),
            NoteType::Noise => Color32::from_rgb(240, 240, 240),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Grid {
    pub note_type: NoteType,
    pub volume: f32,
    pub length: u64,
    scale: Scale,
    notes: Vec<bool>,
    id: String
}

impl Grid {
    pub fn for_note_type(note_type: NoteType, counter: usize) -> Self {
        Self {
            note_type,
            volume: 1.0,
            length: 250,
            scale: Scale::CMajor,
            notes: vec![false; (LOOP_LENGTH * LOOP_LENGTH) as usize],
            id: format!("Track {}", counter)
        }
    }

    pub fn show(&mut self, ctx: &Context, cursor: f32) {
        let win = egui::Window::new(&self.id).resizable(false).scroll([false, false]);
        win.show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                if ui.button("Clear").clicked() {
                    self.notes = vec![false; (LOOP_LENGTH * LOOP_LENGTH) as usize]
                }

                ui.menu_button("Scale...", |ui| {
                    if ui.button(Scale::CMajor.label_text(self.scale)).clicked() {
                        self.scale = Scale::CMajor
                    }
                    if ui.button(Scale::CMinor.label_text(self.scale)).clicked() {
                        self.scale = Scale::CMinor
                    }
                    if ui.button(Scale::Chromatic.label_text(self.scale)).clicked() {
                        self.scale = Scale::Chromatic
                    }
                    if ui.button(Scale::Pentatonic.label_text(self.scale)).clicked() {
                        self.scale = Scale::Pentatonic
                    }

                })
            });

            egui::MenuBar::new().ui(ui, |ui| {
                ui.label("Volume");
                ui.add(egui::Slider::new(&mut self.volume, RangeInclusive::new(0.0, 2.0)).show_value(false));

                ui.label("Length");
                ui.add(egui::Slider::new(&mut self.length, RangeInclusive::new(0, 2000)).show_value(false));

            });

            egui::Frame::new().inner_margin(3).show(ui, |ui| {
                self.draw_grid(ui, cursor)
            });
        });
    }

    fn draw_grid(&mut self, ui: &mut Ui, cursor: f32) {
        let dim = 20.0 * LOOP_LENGTH as f32;
        let (rect, response) = ui.allocate_exact_size(Vec2::new(dim, dim), Sense::click_and_drag());

        for (i, lit) in self.notes.iter().enumerate() {
            let (x, y) = (i as u32 % LOOP_LENGTH, i as u32 / LOOP_LENGTH);
            let center = Pos2::new(
                (x * 20 + 10) as f32 + rect.left(),
                (y * 20 + 10) as f32 + rect.top());
            if *lit {
                ui.painter().circle_filled(center, 10.0, self.note_type.color());
            } else {
                ui.painter().circle_stroke(center, 10.0, (1.0, Color32::from_gray(0x88)));
            }
        }

        ui.painter().vline(
            cursor * dim + rect.left(),
            Rangef::new(rect.top(), rect.top() + dim),
            (1.0, self.note_type.color())
        );

        if response.contains_pointer() {
            ui.input(|input| {
                if input.pointer.button_clicked(PointerButton::Primary) {
                    if let Some(pos) = input.pointer.latest_pos() {
                        let x = ((pos.x - rect.left()) / 20.0).floor() as usize;
                        let y = ((pos.y - rect.top()) / 20.0).floor() as usize;
                        let n = x + y * LOOP_LENGTH as usize;
                        self.notes[n] = !self.notes[n]
                    }
                }
            })
        }
    }

    pub fn notes(&self, beat: u32) -> Vec<i32> {
        let mut notes = vec![];
        for y in 0..LOOP_LENGTH {
            if self.notes[(y * LOOP_LENGTH + beat) as usize] {
                let row = LOOP_LENGTH - y - 1;
                notes.push(self.scale.tone(row))
            }
        }
        notes
    }
}
