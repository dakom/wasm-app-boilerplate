use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub window_size: WindowSize,
    pub ball_position: Position,
    pub interpolation: f64,
    pub has_loaded: bool
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WindowSize {
    pub width: u32,
    pub height: u32 
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: f64,
    pub y: f64 
}
