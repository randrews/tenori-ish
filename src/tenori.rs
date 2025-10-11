use std::time::Instant;
use rodio::OutputStream;
use crate::grid::Grid;
use crate::dialog::Dialog;
use crate::noise::Note;

pub const LOOP_LENGTH: u32 = 16;

pub struct Tenori {
    /// Tempo in beats per minute
    pub tempo: u32,

    // A count _in beats_ of where we are in the loop
    pub timer: f32,

    /// Whether or not we're playing; false == paused
    pub playing: bool,

    // The instant of the last time we called tick()
    last_tick: Option<Instant>,

    /// The grids that we currently have going
    pub grids: Vec<Grid>,

    /// Running count of windows created (for ids)
    pub window_counter: usize,

    // The audio output stream to which we will play notes
    output_stream: OutputStream,

    /// Error dialogs we're currently showing
    pub dialogs: Vec<Dialog>,

    /// If present, we can save to this file without asking the
    /// user to select a file first.
    pub default_filename: Option<String>
}

impl Default for Tenori {
    fn default() -> Self {
        let output_stream = rodio::OutputStreamBuilder::open_default_stream()
            .expect("Open audio output stream");

        Self {
            tempo: 90,
            timer: 0.0,
            playing: true,
            last_tick: None,
            grids: vec![],
            window_counter: 0,
            dialogs: vec![],
            default_filename: None,
            output_stream
        }
    }
}

impl Tenori {
    /// Call this every frame to update the timer / last tick based on the current instant
    /// and the tempo
    pub fn tick(&mut self) -> bool {
        let now = Instant::now();
        let old_beat = self.beat();
        if let Some(last) = self.last_tick && self.playing {
            let dt = (now - last).as_secs_f32();
            let bps = (self.tempo as f32) / 60.0;
            // Timer is an amount of time _in beats_ and some of those beats might have been for a
            // different tempo.
            // We know now a time delta in seconds and a conversion factor to turn that to beats, so:
            self.timer += dt * bps;
            // 16 beats in the loop, so timer should never be over 16:
            while self.timer > LOOP_LENGTH as f32 { self.timer -= LOOP_LENGTH as f32 }
        }

        // Playing or not, update last_tick so that the next frame adds the correct duration to timer
        self.last_tick = Some(now);

        // Did we enter a new beat?
        self.beat() != old_beat
    }

    /// Which beat (0..loop_length) we're on
    pub fn beat(&self) -> u32 {
        self.timer.floor() as u32
    }

    /// What fraction we are (0.0..1.0) through the loop
    /// (multiply by window width to find the x coord to draw the cursor line)
    pub fn ratio(&self) -> f32 {
        self.timer / LOOP_LENGTH as f32
    }

    pub fn notes_for_beat(&self) -> Vec<Note> {
        let mut notes = vec![];
        let beat = self.beat();

        for grid in self.grids.iter() {
            for tone in grid.notes(beat).into_iter() {

                notes.push(Note {
                    tone,
                    volume: grid.volume,
                    timbre: grid.timbre
                })
            }
        }
        notes
    }

    pub fn play(&mut self, note: Note) {
        note.play(self.output_stream.mixer())
    }
}