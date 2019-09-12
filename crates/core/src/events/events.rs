use wasm_bindgen::prelude::*;
use log::{info};
use std::convert::TryInto;
use shared::state::renderer::{WindowSize};
use shared::events::{CoreEventIndex};
use crate::components::*;
use shipyard::*;

pub fn handle_event(evt_type:u32, evt_data: JsValue, world:&World) -> Result<(), JsValue> {
    let evt_type:CoreEventIndex = evt_type.try_into()?;

    match evt_type {
        CoreEventIndex::ToggleAudio =>
        {
            world.run::<(EntitiesMut, &mut AudioActive), _>(|(mut entities, mut a)| {
                if let Some(a) = a.iter().next() {
                    a.0 = !a.0 ;
                    //info!("got audio active: {}", a.0);
                }
            });
        },
        CoreEventIndex::SetSpeed =>
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

        CoreEventIndex::WindowSize =>
        {
            let window_size:WindowSize = serde_wasm_bindgen::from_value(evt_data)?;
            world.run::<(EntitiesMut, &mut WindowSize), _>(|(mut entities, mut w)| {
                if let Some(w) = w.iter().next() {
                    w.width = window_size.width;
                    w.height = window_size.height;
                    //info!("got window size: {:?}", w);
                }
            });
        },

        CoreEventIndex::AudioLoaded => {
            world.run::<(EntitiesMut, &mut InitState), _>(|(mut entities, mut state)| {
                if let Some(state) = state.iter().next() {
                    state.audio_loaded = true;
                    if state.renderer_loaded {
                        state.phase = InitPhase::Ready
                    }
                }
            });
        },

        CoreEventIndex::RendererLoaded=> {
            world.run::<(EntitiesMut, &mut InitState), _>(|(mut entities, mut state)| {
                if let Some(state) = state.iter().next() {
                    state.renderer_loaded = true;
                    if state.audio_loaded {
                        state.phase = InitPhase::Ready
                    }
                }
            });
        },

        CoreEventIndex::Started => {
            world.run::<(EntitiesMut, &mut InitState), _>(|(mut entities, mut state)| {
                if let Some(state) = state.iter().next() {
                    if state.phase == InitPhase::Waiting {
                        if !state.audio_loaded || !state.renderer_loaded {
                            state.phase = InitPhase::Loading
                        }
                    }
                }
            });
        },
        _ => 
        {
            info!("unknown event!");
        }
    }

    Ok(())
}