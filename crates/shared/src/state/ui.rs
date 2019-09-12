use serde::{Serialize, Deserialize};
use super::audio;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub audio_active: bool, 
    pub speed: f64,
    pub init_phase : u32 // to match InitPhase but not serialize as a key-value obj
}

//the order must match typescript
#[derive(PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum InitPhase {
    Waiting,
    Loading,
    Ready
}

impl Default for InitPhase {
    fn default() -> Self {
        Self::Waiting
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            audio_active: audio::State::default().is_active,
            speed: 0.5,
            init_phase: InitPhase::default() as u32
        }
    }
}


impl State {
    pub fn new() -> Self {
        Default::default()
    }
}