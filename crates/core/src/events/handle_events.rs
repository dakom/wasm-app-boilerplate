use wasm_bindgen::prelude::*;
use log::{info};
use std::rc::Rc;
use std::cell::RefCell;
use std::convert::TryInto;
use crate::renderer::Renderer;
use crate::audio::AudioSequencer;
use super::{BridgeEventIndex, Timestamp};
use crate::systems;
use crate::components::*;
use shipyard::*;

//if result is Ok(true) then send the updated state back

pub fn handle_event(evt_type:u32, evt_data: JsValue, world:&World, renderer:&mut Renderer, sequencer:&mut AudioSequencer) -> Result<(), JsValue> 
{
    let evt_type:BridgeEventIndex = evt_type.try_into()?;

    match evt_type {
        BridgeEventIndex::ToggleAudio =>
        {
            world.run::<Unique<&mut AudioActive>, _>(|mut a| {
                a.0 = !a.0;
                //info!("got audio active: {}", a.0);
            });
        },
        BridgeEventIndex::Speed =>
        {
            let speed:Speed = serde_wasm_bindgen::from_value(evt_data)?;

            //speed crashes
            world.run::<(EntitiesMut, &mut Speed), _>(|(mut entities, mut s)| { 
                if let Some(s) = s.iter().next() {
                    s.0 = speed.0;
                    //info!("got speed: {}", s.0);
                }
            });
        },

        BridgeEventIndex::WindowSize =>
        {
            let window_size:WindowSize = serde_wasm_bindgen::from_value(evt_data)?;
            world.run::<(EntitiesMut, &mut WindowSize), _>(|(mut entities, mut w)| {
                if let Some(w) = w.iter().next() {
                    w.width = window_size.width;
                    w.height = window_size.height;
                }
            });
        },

        BridgeEventIndex::BgTexture => 
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