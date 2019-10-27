pub use crate::events::{Speed, WindowSize};

pub struct Collision {}
pub struct AudioActive(pub bool);
pub struct Direction {pub x: f64, pub y: f64}
pub struct Position {
    pub x: f64,
    pub y: f64 
}
pub struct LastPosition {
    pub x: f64,
    pub y: f64 
}
#[derive(Default)]
pub struct AssetsLoaded {
    pub renderer: bool,
    pub audio: bool
}
