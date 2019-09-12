use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub is_active: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_active: false,
        }
    }
}


impl State {
    pub fn new() -> Self {
        Default::default()
    }
}