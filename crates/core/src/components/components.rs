pub use crate::events::{Speed, WindowSize};

pub struct Collision {}
pub struct AudioActive(pub bool);
pub struct Direction {pub x: f64, pub y: f64}
pub struct Position {
    pub x: f64,
    pub y: f64 
}
pub struct InitState {
    pub phase: InitPhase,
    pub renderer_loaded: bool,
    pub audio_loaded: bool
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
