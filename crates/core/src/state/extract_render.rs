use wasm_bindgen::prelude::*;
use shared::state::renderer::{Position};
use shared::consts;
use serde::Serialize;
use super::{State};
use shared::state::renderer;

pub fn extract_render_state_js(state:&State, interpolation:f64) -> JsValue {
    serde_wasm_bindgen::to_value(&extract_render_state_struct(state, interpolation)).unwrap()
}
pub fn extract_render_state_struct(state:&State, interpolation:f64) -> renderer::State {
    renderer::State {
        window_size: state.window_size,
        //The ball position is considered the midpoint for physics
        //So drawing should also expect it as the midpoint
        ball_position: Position {
            x: state.ball_position.x - consts::ball.radius,
            y: state.ball_position.y - consts::ball.radius
        },
        interpolation,
        has_loaded: state.renderer_loaded
    }
}