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

    world.run::<Unique<&mut AssetsLoaded>, _>(|assets_loaded| {
        if assets_loaded.renderer && assets_loaded.audio {
            event_sender.send(&Event::AssetsLoaded{});
        }
    });
}