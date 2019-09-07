use wasm_bindgen::prelude::*;
use serde::Deserialize;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use log::{info};
use crate::state::{State};

#[derive(FromPrimitive)]
#[repr(u32)]
pub enum CoreEvent {
    AppendText,
    UpdateInput
}

#[derive(Deserialize)]
struct UpdateInput(String);

pub fn handle_event(in_evt:u32, data: JsValue, state:&mut State) -> Result<(), JsValue> {
    match FromPrimitive::from_u32(in_evt) {
        Some(CoreEvent::AppendText) =>
        {
            if !state.text_input.is_empty() {
                state.results.push(state.text_input.clone());
                state.text_input = "".to_owned();
            } else {
                info!("no text waiting to be added!");
            }
        },
        Some(CoreEvent::UpdateInput) =>
        {
            let text_input:UpdateInput = serde_wasm_bindgen::from_value(data)?;
            state.text_input = text_input.0;
        },
        _ => 
        {
            info!("unknown event!");
        }
    }

    Ok(())
}