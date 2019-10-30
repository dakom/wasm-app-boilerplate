use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::futures_0_3::future_to_promise;
use std::rc::{Rc};
use std::cell::{RefCell};
use awsm_web::loaders::fetch;
use super::audio::AudioSequencer;
use shipyard::*;
use crate::components::{AssetsLoaded};

pub fn load_assets(sequencer:Rc<RefCell<AudioSequencer>>, world:Rc<World>) {

    future_to_promise({
            async move {

                //if we don't clone it, sequencer will be borrowed for the duration of the promise
                //this will conflict with borrowing during state updates
                let ctx = {
                    sequencer.borrow().ctx.clone()
                };
                let one_shot_buffer = fetch::audio("media/audio/oneshot.mp3", &ctx).await?;

                let mut sequencer = sequencer.borrow_mut();
                sequencer.one_shot_buffer = Some(one_shot_buffer);

                world.run::<(&mut AssetsLoaded), _>(|assets_loaded| {
                    if let Some(assets_loaded) = assets_loaded.iter().next() {
                        assets_loaded.audio = true;
                    }
                });

                Ok(JsValue::null())
            }
        });
}