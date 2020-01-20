use wasm_bindgen::prelude::*;
use log::{info};
use std::convert::TryInto;
use crate::renderer::Renderer;
use crate::audio::AudioSequencer;
use super::{BridgeEvent};
use crate::components::*;
use shipyard::prelude::*;

//if result is Ok(true) then send the updated state back

pub fn handle_event(evt_type:u32, evt_data: JsValue, world:&World, renderer:&mut Renderer, _sequencer:&mut AudioSequencer) -> Result<(), JsValue> 
{
    let evt_type:BridgeEvent = evt_type.try_into()?;

    match evt_type {
        BridgeEvent::ToggleAudio =>
        {
            let mut audio_active = world.borrow::<Unique<&mut AudioActive>>();
            audio_active.0 = !audio_active.0;
        },
        BridgeEvent::Speed =>
        {
            let speed:Speed = serde_wasm_bindgen::from_value(evt_data)?;

            let mut speeds = world.borrow::<&mut Speed>();
            if let Some(s) = (&mut speeds).iter().next() {
                s.0 = speed.0;
            }
        },

        BridgeEvent::WindowSize =>
        {
            let window_size:WindowSize = serde_wasm_bindgen::from_value(evt_data)?;
            let mut window_sizes = world.borrow::<&mut WindowSize>();
            if let Some(w) = (&mut window_sizes).iter().next() {
                w.width = window_size.width;
                w.height = window_size.height;
            }
        },

        BridgeEvent::BgTexture => 
        {
            let img_data:web_sys::ImageData = evt_data.into();
            renderer.upload_bg_texture(&img_data)?
        },

        _ => 
        {
            info!("unknown event!");
        }
    }

    Ok(())
}