use wasm_bindgen::prelude::*;
use serde::Serialize;
use crate::state::{State};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ui {
    audio_active: bool, 
    speed: f64 
}

impl Ui {

    pub fn new(state:&State) -> Self {
        Self {
            audio_active: state.audio_active, 
            speed: state.speed
        }
    }
    pub fn to_js(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}