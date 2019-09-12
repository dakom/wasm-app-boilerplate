use shipyard::*;
use shared::state::audio;
use shared::consts;
use log::{info};
use crate::components::*;

pub fn extract_audio_state(world:&World, interpolation:f64, state:&mut audio::State) {

    world.run::<(&AudioActive), _>(|(active)| {
        if let Some(active) = active.iter().next() {
            state.is_active = active.0;
        }
    });

}