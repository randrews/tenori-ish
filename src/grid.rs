use std::ops::RangeInclusive;
use eframe::egui;
use eframe::egui::{Color32, Context, Id, Margin, PointerButton, Pos2, Rangef, Response, Sense, Stroke, TopBottomPanel, Ui, Vec2, Widget};
use crate::nome::LOOP_LENGTH;

pub enum NoteType {
    Sine,
    Triangle,
    Sawtooth,
    Square,
    Noise
}

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

pub struct Grid {
    note_type: NoteType,
    volume: f32,
    notes: Vec<bool>,
    pub id: String
}

impl Grid {
    pub fn for_note_type(note_type: NoteType, counter: usize) -> Self {
        Self {
            note_type,
            volume: 50.0,
            notes: vec![false; (LOOP_LENGTH * LOOP_LENGTH) as usize],
            id: format!("Track {}", counter)
        }
    }

    pub fn show(&mut self, ctx: &Context, cursor: f32) {
        let id = Id::from(self.id.clone());
        let win = egui::Window::new(&self.id).resizable(false).scroll([false, false]);
        win.show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.label("Volume");
                ui.add(egui::Slider::new(&mut self.volume, RangeInclusive::new(0.0, 100.0)));
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
                        println!("window: {} pos: {} {}", self.id, pos.x - rect.left(), pos.y - rect.top());
                        let x = ((pos.x - rect.left()) / 20.0).floor() as usize;
                        let y = ((pos.y - rect.top()) / 20.0).floor() as usize;
                        let n = x + y * LOOP_LENGTH as usize;
                        self.notes[n] = !self.notes[n]
                    }
                }
            })
        }
    }

    // fn frame(&mut self, ui: &mut Ui, cursor: f32) {
    //     egui::Frame::new().inner_margin(3).show(ui, |ui| {
    //         let dim = 20.0 * LOOP_LENGTH as f32;
    //         ui.set_width(dim);
    //         ui.set_height(dim);
    //         let rect = ui.clip_rect().translate(Vec2::new(6.0, 6.0));
    //         //ui.painter().debug_rect(rect, Color32::from_rgb(0, 255, 0), "blah");
    //         for (i, lit) in self.notes.iter().enumerate() {
    //             let (x, y) = (i as u32 % LOOP_LENGTH, i as u32 / LOOP_LENGTH);
    //             let center = Pos2::new(
    //                 (x * 20 + 10) as f32 + rect.left(),
    //                 (y * 20 + 10) as f32 + rect.top());
    //             if *lit {
    //                 ui.painter().circle_filled(center, 10.0, self.note_type.color());
    //             } else {
    //                 ui.painter().circle_stroke(center, 10.0, (1.0, Color32::from_gray(0x88)));
    //             }
    //         }
    //
    //         ui.painter().vline(
    //             cursor * dim + rect.left(),
    //             Rangef::new(rect.top(), rect.top() + dim),
    //             (1.0, self.note_type.color())
    //         );
    //
    //         if ui.response().contains_pointer() {
    //             ui.input(|input| {
    //                 if input.pointer.button_clicked(PointerButton::Primary) {
    //                     if let Some(pos) = input.pointer.latest_pos() {
    //                         println!("window: {} pos: {} {}", self.id, pos.x - rect.left(), pos.y - rect.top());
    //                         let x = ((pos.x - rect.left()) / 20.0).floor() as usize;
    //                         let y = ((pos.y - rect.top()) / 20.0).floor() as usize;
    //                         let n = x + y * LOOP_LENGTH as usize;
    //                         self.notes[n] = !self.notes[n]
    //                     }
    //                 }
    //             })
    //         }
    //     });
    // }

    pub fn notes(&self, beat: u32) -> Vec<u32> {
        let mut notes = vec![];
        for y in 0..LOOP_LENGTH {
            if self.notes[(y * LOOP_LENGTH + beat) as usize] {
                notes.push(y)
            }
        }
        notes
    }
}
