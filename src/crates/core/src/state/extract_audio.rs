use wasm_bindgen::prelude::*;
use serde::Serialize;
use crate::state::{State};
use shared::state::audio;

pub fn extract_audio_state_js(state:&State, interpolation:f64) -> JsValue {
    serde_wasm_bindgen::to_value(&extract_audio_state_struct(state, interpolation)).unwrap()
}
pub fn extract_audio_state_struct(state:&State, interpolation:f64) -> audio::State {
    audio::State {
        is_active: state.audio_active,
        interpolation
    }
}