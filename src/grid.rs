use color::ColorSpace;
use std::ops::{DerefMut, RangeInclusive};
use std::sync::{Arc, Mutex};
use eframe::egui;
use eframe::egui::{Color32, Context, Id, PointerButton, Pos2, Rangef, Sense, Ui, Vec2};
use tinyrand::{Rand, StdRand};
use crate::gui::Showable;
use crate::scale::Scale;
use crate::tenori::LOOP_LENGTH;
use crate::timbre::Timbre;

#[derive(Clone)]
pub struct Grid {
    pub volume: f32,
    pub scale: Scale,
    pub notes: Vec<bool>,
    pub id: Id,
    pub name: String,
    pub open: bool,
    pub timbre: Timbre,
    pub timbre_open: bool,
    pub color: Color32,
    pub rand: Arc<Mutex<StdRand>>
}

impl Grid {
    pub fn new(id: Id, rand: Arc<Mutex<StdRand>>) -> Self {
        let color = Self::random_color(rand.lock().unwrap());

        Self {
            volume: 1.0,
            open: true,
            scale: Scale::CMajor,
            notes: vec![false; (LOOP_LENGTH * LOOP_LENGTH) as usize],
            name: "New Track".to_string(),
            timbre: Timbre::default(),
            timbre_open: false,
            rand,
            color,
            id
        }
    }

    fn random_color<R: Rand, T: DerefMut<Target=R>>(mut rand: T) -> Color32 {
        let angle = (rand.next_lim_u32(72) * 5) as f32;
        let rgb = color::Hsl::convert::<color::Srgb>([angle, 100.0, 50.0]);
        Color32::from_rgb(
            (rgb[0] * 255.0) as u8,
            (rgb[1] * 255.0) as u8,
            (rgb[2] * 255.0) as u8)
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
                ui.painter().circle_filled(center, 10.0, self.color);
            } else {
                ui.painter().circle_stroke(center, 10.0, (1.0, Color32::from_gray(0x88)));
            }
        }

        ui.painter().vline(
            cursor * dim + rect.left(),
            Rangef::new(rect.top(), rect.top() + dim),
            (1.0, self.color)
        );

        if response.contains_pointer() {
            ui.input(|input| {
                if input.pointer.button_clicked(PointerButton::Primary) &&
                    let Some(pos) = input.pointer.latest_pos() {
                        let x = ((pos.x - rect.left()) / 20.0).floor() as usize;
                        let y = ((pos.y - rect.top()) / 20.0).floor() as usize;
                        let n = x + y * LOOP_LENGTH as usize;
                        self.notes[n] = !self.notes[n]
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

impl Showable<f32> for Grid {
    fn show(&mut self, ctx: &Context, cursor: &f32) {
        let mut open = true;
        let win = egui::Window::new(&self.name)
            .id(self.id)
            .resizable(false)
            .scroll([false, false])
            .open(&mut open);
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
                });

                if ui.button("Timbre...").clicked() {
                    self.timbre_open = !self.timbre_open;
                };

                if ui.button("Color").clicked() {
                    self.color = Self::random_color(self.rand.lock().unwrap());
                }
            });

            egui::MenuBar::new().ui(ui, |ui| {
                ui.label("Volume");
                ui.add(egui::Slider::new(&mut self.volume, RangeInclusive::new(0.0, 2.0)).show_value(false));
            });

            egui::Frame::new().inner_margin(3).show(ui, |ui| {
                self.draw_grid(ui, *cursor)
            });
        });

        if self.timbre_open {
            let mut topen = true;
            let mut name = self.name.clone();
            let (id, title) = (format!("{} timbre", self.id.value()).into(), format!("{} Timbre", self.name));
            (&mut self.timbre, &mut topen, &mut name).show(ctx, &(id, title));
            self.timbre_open = topen;
            self.name = name;
        }

        self.open = open;
    }
}