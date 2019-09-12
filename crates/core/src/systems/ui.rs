use shipyard::*;
use shared::state::ui;
use shared::consts;
use crate::components::*;

pub fn extract_ui_state(world:&World, interpolation:f64, state:&mut ui::State) {

    world.run::<(&AudioActive), _>(|(active)| {
        if let Some(active) = active.iter().next() {
            state.audio_active = active.0;
        }
    });

    world.run::<(&Speed), _>(|(speed)| {
        if let Some(speed) = speed.iter().next() {
            state.speed = speed.0;
        }
    });

    world.run::<(&InitState), _>(|(init_state)| {
        if let Some(init_state) = init_state.iter().next() {
            state.init_phase = (*init_state).phase as u32;
            if init_state.phase == InitPhase::Ready {
                //TODO - remove the entity... depends on this: https://github.com/leudz/shipyard/issues/7
            }
        }
    });
}