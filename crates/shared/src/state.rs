use crate::consts;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct State {
    pub audio_active: bool, 
    pub renderer_active: bool,
    pub speed: f64,
    pub init_phase: Option<u32>, // to match InitPhase but not serialize as a key-value obj
    pub window_width: u32,
    pub window_height: u32,
    pub ball_position_x: f64,
    pub ball_position_y: f64,
    pub interpolation: f64
}

impl State {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for State {
    fn default() -> Self {
        Self{
            audio_active: false,
            renderer_active: false,
            speed: consts::INITIAL_SPEED,
            init_phase: Some(InitPhase::default() as u32),
            window_width: 0,
            window_height: 0,
            ball_position_x: 0.0,
            ball_position_y: 0.0,
            interpolation: 0.0
        }
    }
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