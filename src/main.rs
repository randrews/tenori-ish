mod gui;
mod nome;
mod grid;

use std::time::Duration;
use eframe::{App, Frame};
use eframe::egui::Context;
use crate::nome::Nome;

#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Fleen", native_options, Box::new(|_cc| {
        Ok(Box::new(Nome::default()))
    })).expect("Error running application");
}

impl App for Nome {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        let play = self.tick();
        let cursor = self.ratio();
        self.menu(&ctx);
        for g in self.grids.iter_mut() {
            g.show(ctx, cursor)
        }

        if play {
            for (track, row) in self.notes_for_beat() {
                println!("{}: {}", track, row)
            }
        }
        ctx.request_repaint_after(Duration::from_millis(17))
    }
}