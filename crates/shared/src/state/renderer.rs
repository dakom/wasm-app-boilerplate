use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub window_size: WindowSize,
    pub ball_position: Position,
    pub interpolation: f64,
    pub has_loaded: bool
}

impl State {
    pub fn new() -> Self {
        Self {
            window_size: WindowSize{ width: 1024, height: 768},
            ball_position: Position {x: 0.0, y: 0.0},
            interpolation: 0.0,
            has_loaded: false
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WindowSize {
    pub width: u32,
    pub height: u32 
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: f64,
    pub y: f64 
}
