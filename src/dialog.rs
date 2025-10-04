use eframe::egui::{Context, Window};
use crate::gui::Showable;

/// A struct to represent a simple messagebox, like for an error or something.
/// Contains a string (the message) and a bool which is cleared when the window is
/// closed.
pub struct Dialog(pub String, pub bool);

impl<T> Showable<T> for Dialog {
    fn show(&mut self, ctx: &Context, _state: &T) {
        let win = Window::new("Hey!").resizable(false).scroll([false, false]);
        let mut open = true;
        win.open(&mut open).show(ctx, |ui| {
            ui.label(&self.0);
            if ui.button("Okay").clicked() {
                self.1 = false
            }
        });
        if !open { self.1 = false }
    }
}

impl<T: AsRef<str>> From<T> for Dialog {
    fn from(value: T) -> Self {
        Self(value.as_ref().to_string(), true)
    }
}