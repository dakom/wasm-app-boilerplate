use serde::{Serialize, Deserialize};

//All the core event data:
#[derive(Serialize, Deserialize)]
pub struct Speed(pub f64);