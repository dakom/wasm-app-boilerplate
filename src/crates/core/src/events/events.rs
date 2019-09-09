use wasm_bindgen::prelude::*;
use log::{info};
use std::convert::TryInto;
use crate::state::{State};
use shared::state::renderer::{WindowSize};
use shared::events::{CoreEvent, Speed};

pub fn handle_event(evt_type:u32, evt_data: JsValue, state:&mut State) -> Result<(), JsValue> {
    let evt_type:CoreEvent = evt_type.try_into()?;

    match evt_type {
        CoreEvent::ToggleAudio =>
        {
            state.audio_active = !state.audio_active;
        },
        CoreEvent::SetSpeed =>
        {
            let speed:Speed = serde_wasm_bindgen::from_value(evt_data)?;
            state.speed = speed.0;
        },
        CoreEvent::WindowSize =>
        {
            let window_size:WindowSize = serde_wasm_bindgen::from_value(evt_data)?;
            state.window_size = window_size; 
        },
        _ => 
        {
            info!("unknown event!");
        }
    }

    Ok(())
}