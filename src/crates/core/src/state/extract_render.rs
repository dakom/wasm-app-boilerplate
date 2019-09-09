use wasm_bindgen::prelude::*;
use serde::Serialize;
use super::{State};
use shared::state::renderer;

pub fn extract_render_state_js(state:&State, interpolation:f64) -> JsValue {
    serde_wasm_bindgen::to_value(&extract_render_state_struct(state, interpolation)).unwrap()
}
pub fn extract_render_state_struct(state:&State, interpolation:f64) -> renderer::State {
    renderer::State {
        window_size: state.window_size,
        ball_position: state.ball_position
    }
}