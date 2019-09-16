use shipyard::*;
use shared::state::*;
use shared::consts;
use log::{info};
use crate::components::*;

pub fn extract_state(world:&World, interpolation:f64, state:&mut State) {

    world.run::<(&AudioActive), _>(|(active)| {
        if let Some(active) = active.iter().next() {
            state.audio_active = active.0;
        }
    });

    world.run::<(&Position), _>(|(positions)| {
        if let Some(pos) = positions.iter().next() {
            state.ball_position_x = pos.x - consts::BALL.radius;
            state.ball_position_y = pos.y - consts::BALL.radius;
        }
    });

    world.run::<(&WindowSize), _>(|(window_size)| {
        if let Some(window_size) = window_size.iter().next() {
            state.window_width = window_size.width;
            state.window_height = window_size.height;
        }
    });

    world.run::<(&Speed), _>(|(speed)| {
        if let Some(speed) = speed.iter().next() {
            state.speed = speed.0;
        }
    });

    let mut entity_to_delete:Option<Key> = None;
    //can't get AllStorages here, so defer the delete
    world.run::<(&InitState), _>(|init_state| {
        if let Some((id, init_state)) = init_state.iter().with_id().next() {
            if init_state.phase == InitPhase::Ready {
                entity_to_delete = Some(id);
                state.init_phase = None; 
            } else {
                state.init_phase = Some((*init_state).phase as u32);
            }
        }
    });

    if let Some(id) = entity_to_delete {
        world.run::<(EntitiesMut, AllStorages), _>(|(mut entities, mut all_storages)| {
            entities.delete(&mut all_storages, id);
        });
    }
    state.interpolation = interpolation;

}