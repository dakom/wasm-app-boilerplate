use wasm_bindgen::prelude::*;
use serde::Serialize;
use super::{State};
use shared::state::ui;

pub fn extract_ui_state_js(state:&State, interpolation:f64) -> JsValue {
    serde_wasm_bindgen::to_value(&extract_ui_state_struct(state, interpolation)).unwrap()
}
pub fn extract_ui_state_struct(state:&State, interpolation:f64) -> ui::State {
    ui::State {
        audio_active: state.audio_active, 
        speed: state.speed
    }
}