use shared::state::renderer::{WindowSize, Position};
pub struct State {
    pub audio_active: bool,
    pub speed: f64,
    pub window_size: WindowSize,
    pub ball_position: Position,
}

//Initial state
impl State {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        Self {
            audio_active: true, 
            speed: 0.5,
            window_size: WindowSize { width: window_width, height: window_height},
            ball_position: Position { x: 0.0, y: 0.0}
        }
    }
}