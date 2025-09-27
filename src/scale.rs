use eframe::egui::RichText;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Scale {
    CMajor,
    CMinor,
    Chromatic,
    Pentatonic
}

impl Scale {
    pub fn label_text(self, other: Scale) -> RichText {
        let selected = other == self;
        let s = match self {
            Scale::CMajor => "C Major",
            Scale::CMinor => "C Minor",
            Scale::Chromatic => "Chromatic",
            Scale::Pentatonic => "Pentatonic",
        };

        let r = RichText::new(s);
        if selected { r.strong() } else { r }
    }

    pub fn tone(self, row: u32) -> i32 {
        let row = row as usize;
        let tones = match self {
            Scale::CMajor => [-9, -7, -5, -4, -2, 0, 2, 3, 5, 7, 8, 10, 12, 14, 15, 17],
            Scale::CMinor => [-9, -7, -6, -4, -2, -1, 1, 3, 5, 6, 8, 10, 11, 13, 15, 17],
            Scale::Chromatic => [-9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6],
            Scale::Pentatonic => [-9, -7, -5, -2, 0, 3, 5, 7, 10, 12, 15, 17, 19, 22, 24, 27]
        };

        if row >= tones.len() {
            0
        } else {
            tones[row]
        }
    }
}