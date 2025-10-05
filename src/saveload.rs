use eframe::egui::Id;
use serde::{Deserialize, Serialize};
use crate::grid::Grid;
use crate::noise::NoteType;
use crate::scale::Scale;
use crate::tenori::Tenori;
use crate::timbre::Timbre;

#[derive(Serialize, Deserialize)]
pub struct PersistedTenori {
    tempo: u32,
    grids: Vec<PersistedGrid>
}

impl From<&Tenori> for PersistedTenori {
    fn from(value: &Tenori) -> Self {
        Self {
            tempo: value.tempo,
            grids: value.grids.iter().map(PersistedGrid::from).collect()
        }
    }
}

impl PersistedTenori {
    pub fn apply_to(self, tenori: &mut Tenori) {
        tenori.grids = self.grids.into_iter().map(|g| g.into_grid(tenori.window_id())).collect();
        tenori.tempo = self.tempo;
        tenori.playing = false; // Start paused
        tenori.timer = 0.0; // Start at the beginning of the loop
    }
}

#[derive(Serialize, Deserialize)]
struct PersistedGrid {
    note_type: NoteType,
    volume: f32,
    scale: Scale,
    notes: String,
    name: String,
    timbre: Timbre,
}

impl From<&Grid> for PersistedGrid {
    fn from(value: &Grid) -> Self {
        let notes: String = value.notes.iter().map(|n| if *n { '1' } else { '0' }).collect();
        Self {
            note_type: value.note_type,
            volume: value.volume,
            scale: value.scale,
            name: value.name.clone(),
            timbre: value.timbre,
            notes
        }
    }
}

impl PersistedGrid {
    pub fn into_grid(self, id: Id) -> Grid {
        let notes: Vec<_> = self.notes.chars().map(|c| c == '1').collect();
        Grid {
            note_type: self.note_type,
            volume: self.volume,
            scale: self.scale,
            name: self.name,
            timbre: self.timbre,
            open: true,
            timbre_open: false,
            notes,
            id
        }
    }
}