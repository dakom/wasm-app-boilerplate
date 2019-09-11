use wasm_bindgen::prelude::*;
use log::{info};
use std::convert::TryInto;
use crate::state::{State};
use shared::state::renderer::{WindowSize};
use shared::events::{CoreEventIndex, Speed};

pub fn handle_event(evt_type:u32, evt_data: JsValue, state:&mut State) -> Result<(), JsValue> {
    let evt_type:CoreEventIndex = evt_type.try_into()?;

    match evt_type {
        CoreEventIndex::ToggleAudio =>
        {
            state.audio_active = !state.audio_active;
        },
        CoreEventIndex::SetSpeed =>
        {
            let speed:Speed = serde_wasm_bindgen::from_value(evt_data)?;
            state.speed = speed.0;
        },
        CoreEventIndex::WindowSize =>
        {
            let window_size:WindowSize = serde_wasm_bindgen::from_value(evt_data)?;
            state.window_size = window_size; 
        },

        CoreEventIndex::AudioLoaded => {
            state.audio_loaded = true;
        },

        CoreEventIndex::RendererLoaded=> {
            state.renderer_loaded = true;
        },

        _ => 
        {
            info!("unknown event!");
        }
    }

    Ok(())
}