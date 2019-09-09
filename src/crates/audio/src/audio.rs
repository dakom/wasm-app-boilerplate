use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::{Rc};
use std::cell::{RefCell};
use log::{info};
use shared::state::audio::{State};

use wasm_bindgen_futures::futures_0_3::future_to_promise;
use awsm_web::loaders::fetch;


pub struct AudioSequencer {
    is_active: bool,
}

impl AudioSequencer {
    pub fn new() -> Result<Self, JsValue> {
        Ok(Self{
            is_active: true
        })
    }

    pub fn on_state(&mut self, state:State) {
        if self.is_active != state.is_active {
            self.is_active = state.is_active;
            info!("audio set to: {}", state.is_active);
        }
    }

}

pub fn start() -> Result<JsValue, JsValue> {
    let mut sequencer = AudioSequencer::new()?;

    let sequencer = Rc::new(RefCell::new(sequencer));

    //Create a function which allows JS to call us for rendering
    //We will need to get a handle and forget the Closure
    //See https://stackoverflow.com/a/53219594/784519
    let _render = Closure::wrap(Box::new({
        let sequencer = Rc::clone(&sequencer);
        move |data:JsValue| {
            {
                let state:Result<State, serde_wasm_bindgen::Error> = serde_wasm_bindgen::from_value(data);
                match state {
                    Ok(state) => {
                        let mut sequencer = sequencer.borrow_mut();
                        sequencer.on_state(state);
                    },
                    Err(reason) => info!("Error: {:?}", reason)
                }
            }
        }
    }) as Box<FnMut(JsValue) -> ()>);

    let render = _render.as_ref().clone();
    _render.forget();

    //Return the event sender
    Ok(render)
}