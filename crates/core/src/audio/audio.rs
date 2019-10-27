use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::{Rc};
use std::cell::{RefCell};
use log::{info};
use crate::events::*;
use web_sys::{AudioContext, AudioBuffer};
use wasm_bindgen_futures::futures_0_3::future_to_promise;
use awsm_web::loaders::fetch;
use awsm_web::audio::AudioPlayer;
use super::assets::load_assets;

pub struct Sequencer {
    pub ctx: AudioContext,
    pub is_active: bool,
    pub one_shot_buffer: Option<AudioBuffer>,
    event_sender: EventSender,
}

impl Sequencer {
    pub fn new(send_event: js_sys::Function, ctx:AudioContext) -> Result<Self, JsValue> {
        let event_sender = EventSender::new(send_event);

        Ok(Self{
            ctx,
            event_sender, 
            is_active: true,
            one_shot_buffer: None,
        })
    }

    pub fn send_event(&self, evt:&Event) {
        self.event_sender.send(evt);
    }

    /*
    pub fn on_state(&mut self, state:State, interpolation:f64) {
        if self.is_active != state.audio_active {
            self.is_active = state.audio_active;
            info!("audio set to: {}", state.audio_active);
        }


        if self.is_active && state.collision {
            info!("Playing...");
            self.one_shot_buffer.as_ref().map(|buf| {

                //TODO change to proper oneshot without the forgotten Box
                //Depends on https://github.com/dakom/awsm/issues/38
                std::mem::forget(Box::new(AudioPlayer::start( &self.ctx, &buf, None as Option<Box<dyn FnMut()>>).unwrap()));
            });
        }
    }
    */

}