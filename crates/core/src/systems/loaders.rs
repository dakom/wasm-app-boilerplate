use shipyard::prelude::*;
use crate::components::*;
use crate::events::*;

pub fn update_loaders(world:&World, event_sender:&EventSender) {

    let assets_loaded = world.borrow::<Unique<&mut AssetsLoaded>>();
    if assets_loaded.renderer && assets_loaded.audio {
        event_sender.send(&BridgeEvent::AssetsLoaded{});
    }
}