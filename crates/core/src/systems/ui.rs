use shipyard::*;
use shared::state::ui;
use shared::consts;
use log::{info};
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



    let mut entity_to_delete:Option<Key> = None;
    //can't get AllStorages here, so defer the delete
    world.run::<(&InitState), _>(|init_state| {
        if let Some((id, init_state)) = init_state.iter().with_id().next() {
            state.init_phase = (*init_state).phase as u32;
            if init_state.phase == InitPhase::Ready {
                entity_to_delete = Some(id);
            }
        }
    });

    if let Some(id) = entity_to_delete {
        world.run::<(EntitiesMut, AllStorages), _>(|(mut entities, mut all_storages)| {
            entities.delete(&mut all_storages, id);
        });
    }

}