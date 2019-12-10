use shipyard::*;
use crate::components::*;
use crate::events::*;

pub fn update_loaders(world:&World, event_sender:&EventSender) {

    world.run::<Unique<&mut AssetsLoaded>, _>(|assets_loaded| {
        if assets_loaded.renderer && assets_loaded.audio {
            event_sender.send(&BridgeEvent::AssetsLoaded{});
        }
    });
}