use wasm_bindgen::prelude::*;
use log::{info};
use std::rc::Rc;
use std::cell::RefCell;
use std::convert::TryInto;
use super::{IoEventIndex, Timestamp};
use crate::systems;
use crate::components::*;
use shipyard::*;

//if result is Ok(true) then send the updated state back
pub fn handle_event(evt_type:u32, evt_data: JsValue, world:&World) -> Result<(), JsValue> 
{
    let evt_type:IoEventIndex = evt_type.try_into()?;

    match evt_type {
        /*
        IoEventIndex::LoopBegin =>
        {
            let (timestamp, delta):(f64, f64) = serde_wasm_bindgen::from_value(evt_data)?;
            //info!("{} {}", timestamp, delta);
        },
        IoEventIndex::LoopUpdate =>
        {
            let delta:f64 = serde_wasm_bindgen::from_value(evt_data)?;
            systems::motion::update_motion(&world, delta);
            systems::state::extract_state(&world,state);
            //info!("{}", delta);
        },
        IoEventIndex::LoopDraw =>
        {
            let interpolation:f64 = serde_wasm_bindgen::from_value(evt_data)?;
            //info!("{}", interpolation);
        },
        IoEventIndex::LoopEnd=>
        {
            let (fps, end):(f64, bool) = serde_wasm_bindgen::from_value(evt_data)?;
            //info!("{} {}", fps, end);
        },
        */
        IoEventIndex::ToggleAudio =>
        {
            world.run::<(EntitiesMut, &mut AudioActive), _>(|(mut entities, mut a)| {
                if let Some(a) = a.iter().next() {
                    a.0 = !a.0 ;
                    //info!("got audio active: {}", a.0);
                }
            });
        },
        IoEventIndex::Speed =>
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

        IoEventIndex::WindowSize =>
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

        IoEventIndex::AudioLoaded => {
            world.run::<(EntitiesMut, &mut InitState), _>(|(mut entities, mut state)| {
                if let Some(state) = state.iter().next() {
                    state.audio_loaded = true;
                    if state.renderer_loaded {
                        state.phase = InitPhase::Ready
                    }
                }
            });
        },

        IoEventIndex::RendererLoaded=> {
            world.run::<(EntitiesMut, &mut InitState), _>(|(mut entities, mut state)| {
                if let Some(state) = state.iter().next() {
                    state.renderer_loaded = true;
                    if state.audio_loaded {
                        state.phase = InitPhase::Ready
                    }
                }
            });
        },

        IoEventIndex::Started => {
            world.run::<(EntitiesMut, &mut InitState), _>(|(mut entities, mut init_state)| {
                if let Some(init_state) = init_state.iter().next() {
                    if init_state.phase == InitPhase::Waiting {
                        if !init_state.audio_loaded || !init_state.renderer_loaded {
                            init_state.phase = InitPhase::Loading
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