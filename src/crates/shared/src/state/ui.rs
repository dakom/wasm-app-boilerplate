use serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub audio_active: bool, 
    pub speed: f64 
}