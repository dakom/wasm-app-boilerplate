use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub is_active: bool,
    pub window_size: WindowSize,
    pub ball_position: Position,
    pub interpolation: f64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_active: false,
            window_size: WindowSize{ width: 0, height: 0},
            ball_position: Position {x: 0.0, y: 0.0},
            interpolation: 0.0,
        }
    }
}

impl State {
    pub fn new() -> Self {
        Default::default()
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
