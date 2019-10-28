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
use shipyard::*;
use crate::components::*;

pub struct AudioSequencer {
    pub ctx: AudioContext,
    pub is_active: bool,
    pub one_shot_buffer: Option<AudioBuffer>,
}

impl AudioSequencer {
    pub fn new(ctx:AudioContext) -> Result<Self, JsValue> {

        Ok(Self{
            ctx,
            is_active: true,
            one_shot_buffer: None,
        })
    }

    pub fn play(&mut self) {
        self.one_shot_buffer.as_ref().map(|buf| {

            AudioPlayer::play_oneshot( &self.ctx, &buf, None as Option<Box<dyn FnMut()>>);
        });
    }
}