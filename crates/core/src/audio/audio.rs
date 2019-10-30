use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, AudioBuffer};
use awsm_web::audio::AudioPlayer;

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

    pub fn play(&mut self) -> Result<(), JsValue> {
        match self.one_shot_buffer.as_ref() {
            None => Ok(()),
            Some(buf) => {
                AudioPlayer::play_oneshot( &self.ctx, &buf, None as Option<Box<dyn FnMut()>>)
                    .map(|_| ())
                    .map_err(|err| err.into())
            }
        }
    }
}