use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::futures_0_3::future_to_promise;
use std::rc::{Rc};
use std::cell::{RefCell};
use log::{info};
use super::events::*; 
use shared::consts;
use awsm_web::loaders::fetch;
use super::audio::Sequencer;

pub fn load_assets(sequencer:Rc<RefCell<Sequencer>>) {

    future_to_promise({
            async move {

                //if we don't clone it, sequencer will be borrowed for the duration of the promise
                //this will conflict with borrowing during state updates
                let ctx = {
                    sequencer.borrow().ctx.clone()
                };
                let one_shot_buffer = fetch::audio("media/audio/oneshot.mp3", &ctx).await?;

                let sequencer = sequencer.borrow();
                sequencer.send_event(&Event::Loaded);

                Ok(JsValue::null())
            }
        });
}