use shipyard::*;
use crate::consts;
use crate::components::*;
use crate::renderer::Renderer;
use std::cell::RefCell;
use std::rc::Rc;
use crate::audio::AudioSequencer;
use log::info;
use crate::events::*;

pub fn update_loaders(world:&World, event_sender:&EventSender) {

    let mut entity_to_delete:Option<Key> = None;
    world.run::<(&mut AssetsLoaded), _>(|(mut assets_loaded)| {
        if let Some((id, assets_loaded)) = assets_loaded.iter().with_id().next() {
            if assets_loaded.renderer && assets_loaded.audio {
                entity_to_delete = Some(id);
                event_sender.send(&Event::AssetsLoaded{});
            }
        } else {
            //info!("nothing to check :D");
        }
    });

    if let Some(id) = entity_to_delete {
        world.run::<(EntitiesMut, AllStorages), _>(|(mut entities, mut all_storages)| {
            entities.delete(&mut all_storages, id);
        });
    }
}