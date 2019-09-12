use shipyard::*;
use shared::state::renderer;
use shared::consts;
use log::{info};
use crate::components::*;

pub fn extract_render_state(world:&World, interpolation:f64, state:&mut renderer::State) {

    world.run::<(&Position), _>(|(positions)| {
        if let Some(pos) = positions.iter().next() {
            state.ball_position.x = pos.x - consts::ball.radius;
            state.ball_position.y = pos.y - consts::ball.radius;
        }
    });

    world.run::<(&WindowSize), _>(|(window_size)| {
        if let Some(window_size) = window_size.iter().next() {
            state.window_size = window_size.clone();
        }
    });

    world.run::<(&InitState), _>(|(init_state)| {
        if let Some(init_state) = init_state.iter().next() {
            if init_state.phase == InitPhase::Ready {
                state.is_active = true;
            }
        }
    });
    state.interpolation = interpolation;

}