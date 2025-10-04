mod gui;
mod tenori;
mod grid;
mod noise;
mod scale;
mod saveload;
mod dialog;

use std::time::Duration;
use eframe::{App, Frame};
use eframe::egui::Context;
use crate::gui::Showable;
use crate::tenori::Tenori;

#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Tenori-ish", native_options, Box::new(|_cc| {
        Ok(Box::new(Tenori::default()))
    })).expect("Error running application");
}

impl App for Tenori {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let play = self.tick();
        let cursor = self.ratio();

        self.show(ctx, &cursor);

        if play {
            for note in self.notes_for_beat() {
                self.play(note)
            }
        }
        ctx.request_repaint_after(Duration::from_millis(17))
    }
}