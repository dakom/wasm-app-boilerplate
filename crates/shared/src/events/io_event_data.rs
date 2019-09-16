use serde::{Serialize, Deserialize};

//All the event data:
#[derive(Serialize, Deserialize)]
pub struct Speed(pub f64);

#[derive(Serialize, Deserialize)]
pub struct Timestamp(pub f64);

#[derive(Serialize, Deserialize)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32 
}