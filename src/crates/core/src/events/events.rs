use wasm_bindgen::prelude::*;
use serde::Deserialize;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use log::{info};
use crate::state::{State};

/**
 * `FromPrimitive` can be applied only to unitary enums and newtypes,
 * therefore we need to split the event type vs. event data
 */

#[derive(FromPrimitive)]
#[repr(u32)]
pub enum CoreEvent {
    ToggleAudio,
    SetSpeed
}

#[derive(Deserialize)]
struct Speed(f64);

pub fn handle_event(in_evt:u32, data: JsValue, state:&mut State) -> Result<(), JsValue> {
    match FromPrimitive::from_u32(in_evt) {
        Some(CoreEvent::ToggleAudio) =>
        {
            state.audio_active = !state.audio_active;
        },
        Some(CoreEvent::SetSpeed) =>
        {
            let speed:Speed = serde_wasm_bindgen::from_value(data)?;
            state.speed = speed.0;
        },
        _ => 
        {
            info!("unknown event!");
        }
    }

    Ok(())
}