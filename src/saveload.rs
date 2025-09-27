use serde::{Deserialize, Serialize};
use crate::grid::Grid;
use crate::tenori::Tenori;

#[derive(Serialize, Deserialize)]
pub struct PersistedTenori {
    pub tempo: u32,
    pub grids: Vec<Grid>
}

impl From<&Tenori> for PersistedTenori {
    fn from(value: &Tenori) -> Self {
        Self {
            tempo: value.tempo,
            grids: value.grids.iter().map(Grid::clone).collect()
        }
    }
}