pub use shared::state::renderer::{WindowSize, Position};
pub use shared::events::{Speed};
pub use shared::state::ui::{InitPhase};

pub struct AudioActive(pub bool);
pub struct Direction {pub x: f64, pub y: f64}

pub struct InitState {
    pub phase: InitPhase,
    pub renderer_loaded: bool,
    pub audio_loaded: bool
}

impl InitState {
    pub fn new () -> Self {
        Default::default()
    }
}

impl Default for InitState {
    fn default() -> Self {
        Self {
            phase: InitPhase::Waiting,
            renderer_loaded: false,
            audio_loaded: false
        }
    }
}
