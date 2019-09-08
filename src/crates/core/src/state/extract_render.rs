use wasm_bindgen::prelude::*;
use serde::Serialize;
use super::{State};
use shared::renderer::{State as RenderState};

pub fn extract_render_state_js(state:&State, interpolation:f64) -> JsValue {
    serde_wasm_bindgen::to_value(&extract_render_state_struct(state, interpolation)).unwrap()
}
pub fn extract_render_state_struct(state:&State, interpolation:f64) -> RenderState {
    RenderState {
        window_size: state.window_size,
        ball_position: state.ball_position
    }
}